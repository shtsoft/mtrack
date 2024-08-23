//! This module defines the application and its state.
//! Additionally, the module declares a submodule with the handlers.

pub mod handlers;

use handlers::get_login::get_login;
use handlers::get_positions::get_positions;
use handlers::health_check::health_check;
use handlers::home::home;
use handlers::logout::logout;
use handlers::post_login::post_login;
use handlers::post_position::post_position;
use handlers::postpos::postpos;
use handlers::tracker::tracker;

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

use serde::{Deserialize, Serialize};

use tokio::net::TcpStream;

use tokio_rustls::server::TlsStream;

use tower::Service;

use tower_http::services::ServeDir;

/// The number of time units a new session is alive.
pub const SESSION_TTL: u8 = 24;
/// The length of a session time unit in seconds.
const SESSION_TTL_UNIT: Duration = Duration::from_secs(3600);

/// The name of the session ID cookie.
pub const SESSION_ID_COOKIE_NAME: &str = "sessionID";

/// A type of names.
type Name = String;

/// A type of session IDs.
type SessionID = u128;

/// Abstracts coordinates.
#[derive(Deserialize, Serialize)]
pub struct Coordinates {
    latitude: f32,
    longitude: f32,
}

/// Abstracts session state.
pub struct SessionState {
    name: Name,
    ttl: u8,
}

/// Abstracts entries in the user databases.
#[derive(Deserialize, Clone)]
pub struct UserEntry {
    name: Name,
    hash: String,
}

/// Abstracts the application state.
pub struct AppState {
    pub sessions: HashMap<SessionID, SessionState>,
    pub positions: HashMap<Name, Coordinates>,
    pub download_users: Vec<UserEntry>,
    pub upload_users: Vec<UserEntry>,
    pub pages: HashMap<&'static str, String>,
}

/// Abstracts the state.
#[derive(Clone)]
pub struct State {
    pub app_state: Arc<RwLock<AppState>>,
    pub dist: String,
}

/// Prunes the application state from expired sessions.
/// - `state` is the application state.
///
/// # Panics
///
/// A panic is caused if there is an issue with the `RwLock`.
fn prune_sessions(state: &Arc<RwLock<AppState>>) {
    let mut dead_sessions = Vec::new();
    let mut lock = state.write().expect("Poisoned lock.");

    for (session, state) in &mut lock.sessions {
        if state.ttl > 0 {
            state.ttl -= 1;
        } else {
            dead_sessions.push(*session);
        }
    }

    for session in dead_sessions {
        lock.sessions.remove(&session);
    }
}

/// Defines the application.
/// - `tls_socket` is the TLS connection the server runs on.
/// - `state` is the server state.
pub async fn server(tls_socket: TlsStream<TcpStream>, state: State) {
    tracing::debug!("TcpStream from proxy to downstream: {:?}", tls_socket);

    tracing::info!("Start serving connection");

    let state_clone = state.app_state.clone();
    thread::spawn(move || loop {
        thread::sleep(SESSION_TTL_UNIT);
        prune_sessions(&state_clone);
    });

    let assets = state.dist + "/assets";
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/positions/:key", post(post_position))
        .route("/positions", get(get_positions))
        .route("/login", post(post_login))
        .route("/logout", post(logout))
        .route("/", get(home))
        .route("/index.html", get(home))
        .route("/login", get(get_login))
        .route("/login/index.html", get(get_login))
        .route("/postpos", get(postpos))
        .route("/postpos/index.html", get(postpos))
        .route("/tracker", get(tracker))
        .route("/tracker/index.html", get(tracker))
        .nest_service("/assets", ServeDir::new(assets))
        .with_state(state.app_state);

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
