use std::collections::{HashMap, VecDeque};
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

use tokio::net::TcpStream;

use tokio_rustls::server::TlsStream;

use tower::Service;

type Coordinates = usize;

pub struct AppState {
    pub key: String,
    pub positions: HashMap<String, VecDeque<Coordinates>>,
}

async fn handler_post_position(
    Path(key): Path<String>,
    State(state): State<Arc<RwLock<AppState>>>,
    body: String,
) -> (StatusCode, String) {
    if key == state.read().expect("Poisoned lock.").key {
        let coordinates = match body.parse::<Coordinates>() {
            Ok(coordinates) => coordinates,
            Err(err) => {
                tracing::warn!("Client posting invalid coordinates: {:?}", err);
                return (StatusCode::BAD_REQUEST, "Coordinates must be numbers.".to_string());
            }
        };

        let positions = &mut state.write().expect("Poisoned lock.").positions;

        if !positions.contains_key(&key) {
            tracing::info!("Create position queue for user KEY");
            positions.insert(key.clone(), VecDeque::with_capacity(500));
        }
        
        if let Some(deque) = positions.get_mut(&key) {
            deque.push_back(coordinates);
        }
        
        (StatusCode::OK, String::new())
    } else {
        tracing::warn!("Client trying to post coordinates with invalid key");
        (StatusCode::BAD_REQUEST, "You must have valid key to post coordinates.".to_string())
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
