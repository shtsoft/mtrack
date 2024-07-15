use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use axum::extract::{Path, Request, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::Router;

use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service;

use hyper_util::rt::TokioIo;

use serde::Deserialize;

use tokio::net::TcpStream;

use tokio_rustls::server::TlsStream;

use tower::Service;

type Coordinates = usize;

#[derive(Deserialize)]
pub struct UserEntry {
    name: String,
    hash: String,
}

pub struct AppState {
    pub upload_users: Vec<UserEntry>,
    pub positions: HashMap<String, Coordinates>,
}

async fn handler_get_position(State(state): State<Arc<RwLock<AppState>>>) -> (StatusCode, String) {
    let positions = &state.read().expect("Poisoned lock.").positions;
    (
        StatusCode::OK,
        serde_json::to_string(positions).expect("Impossible serialization error."),
    )
}

fn lookup(password: &str, users: &Vec<UserEntry>) -> Option<String> {
    for user in users {
        let verified = match bcrypt::verify(password, &user.hash) {
            Ok(b) => b,
            Err(err) => {
                tracing::error!("Failed to verify password of user {}: {:?}", user.name, err);
                continue;
            }
        };
        if verified {
            return Some(user.name.clone());
        }
    }

    None
}

async fn handler_post_position(
    Path(key): Path<String>,
    State(state): State<Arc<RwLock<AppState>>>,
    body: String,
) -> (StatusCode, String) {
    // The following indirection is here to prevent a deadlock arising from the lifetime of the
    // guard.
    let lookup = lookup(&key, &state.read().expect("Poisoned lock.").upload_users);
    if let Some(name) = lookup {
        let coordinates = match body.parse::<Coordinates>() {
            Ok(coordinates) => coordinates,
            Err(err) => {
                tracing::warn!("Client posting invalid coordinates: {:?}", err);
                return (
                    StatusCode::BAD_REQUEST,
                    "Coordinates must be numbers.".to_string(),
                );
            }
        };

        let positions = &mut state.write().expect("Poisoned lock.").positions;

        if positions.insert(name.clone(), coordinates).is_none() {
            tracing::info!("Start tracking position of user {}", name);
        };

        (StatusCode::OK, String::new())
    } else {
        tracing::warn!("Client trying to post coordinates with invalid key");
        (
            StatusCode::BAD_REQUEST,
            "You must have valid key to post coordinates.".to_string(),
        )
    }
}

async fn handler_health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn server(tls_socket: TlsStream<TcpStream>, state: Arc<RwLock<AppState>>) {
    tracing::debug!("TcpStream from proxy to downstream: {:?}", tls_socket);

    tracing::info!("Start serving connection");

    let app = Router::new()
        .route("/health_check", get(handler_health_check))
        .route("/position/:key", post(handler_post_position))
        .route("/position", get(handler_get_position))
        .with_state(state);

    if let Err(err) = http1::Builder::new()
        .serve_connection(
            TokioIo::new(tls_socket),
            service::service_fn(move |request: Request<Incoming>| app.clone().call(request)),
        )
        .await
    {
        tracing::error!("Failed to serve connection: {:?}", err);
    }
}
