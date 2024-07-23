pub mod handlers;

use handlers::get_positions::get_positions;
use handlers::health_check::health_check;
use handlers::login::login;
use handlers::logout::logout;
use handlers::post_position::post_position;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use axum::extract::Request;
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

pub const SESSION_TTL: u8 = 24;
const SESSION_TTL_UNIT: Duration = Duration::from_secs(3600);

pub const SESSION_ID_COOKIE_NAME: &str = "sessionID";

type Coordinates = usize;

pub struct SessionState {
    name: String,
    ttl: u8,
}

#[derive(Deserialize, Clone)]
pub struct UserEntry {
    name: String,
    hash: String,
}

pub struct AppState {
    pub sessions: HashMap<u128, SessionState>,
    pub positions: HashMap<String, Coordinates>,
    pub download_users: Vec<UserEntry>,
    pub upload_users: Vec<UserEntry>,
}

pub async fn server(tls_socket: TlsStream<TcpStream>, state: Arc<RwLock<AppState>>) {
    fn prune_sessions(state: &Arc<RwLock<AppState>>) {
        let mut dead_sessions = Vec::new();

        let sessions = &mut state.write().expect("Poisoned lock.").sessions;
        for (session, state) in sessions {
            if state.ttl > 0 {
                state.ttl -= 1;
            } else {
                dead_sessions.push(session);
            }
        }

        let sessions = &mut state.write().expect("Poisoned lock.").sessions;
        for session in dead_sessions {
            sessions.remove(session);
        }
    }

    tracing::debug!("TcpStream from proxy to downstream: {:?}", tls_socket);

    tracing::info!("Start serving connection");

    let state_clone = state.clone();
    thread::spawn(move || loop {
        thread::sleep(SESSION_TTL_UNIT);
        prune_sessions(&state_clone);
    });

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/positions/:key", post(post_position))
        .route("/positions", get(get_positions))
        .route("/login", post(login))
        .route("/logout", post(logout))
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
