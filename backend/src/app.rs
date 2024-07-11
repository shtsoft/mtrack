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

pub struct AppState {
    pub key: String,
}

async fn handler_post(
    Path(key): Path<String>,
    State(state): State<Arc<RwLock<AppState>>>,
) -> (StatusCode, String) {
    if key == state.read().expect("Poisoned lock.").key {
        (StatusCode::OK, "".to_string())
    } else {
        (StatusCode::BAD_REQUEST, "".to_string())
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
        .route("/:key", post(handler_post))
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
