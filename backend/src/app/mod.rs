//! This module defines the application and its state.
//! Additionally, it declares a submodule for the handlers.

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

/// The number of time units a new session remains active.
pub const SESSION_TTL: u8 = 24;
/// The duration of a single session time unit in seconds.
pub const SESSION_TTL_UNIT: Duration = Duration::from_secs(3600);

/// The name of the session ID cookie.
pub const SESSION_ID_COOKIE_NAME: &str = "sessionID";

/// Type alias for names.
type Name = String;

/// Type alias for session IDs.
type SessionID = u128;

/// Represents geographic coordinates.
#[derive(Deserialize, Serialize)]
pub struct Coordinates {
    latitude: f32,
    longitude: f32,
}

/// Represents the state of a session.
pub struct SessionState {
    pub name: Name,
    pub ttl: u8,
}

/// Represents an entry in the user database.
#[derive(Deserialize, Clone)]
pub struct UserEntry {
    name: Name,
    hash: String,
}

/// Represents the global application state.
pub struct AppState {
    pub sessions: HashMap<SessionID, SessionState>,
    pub positions: HashMap<Name, Coordinates>,
    pub download_users: Vec<UserEntry>,
    pub upload_users: Vec<UserEntry>,
    pub pages: HashMap<&'static str, String>,
}

/// Wraps the shared application state.
#[derive(Clone)]
pub struct State {
    pub app_state: Arc<RwLock<AppState>>,
    pub dist: String,
}

/// Configures and starts the application server.
/// - `tls_socket` is the TLS connection the server runs on.
/// - `state` is the shared application state.
pub async fn server(tls_socket: TlsStream<TcpStream>, state: State) {
    tracing::debug!("TcpStream from proxy to downstream: {:?}", tls_socket);

    tracing::info!("Starting connection handler");

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
