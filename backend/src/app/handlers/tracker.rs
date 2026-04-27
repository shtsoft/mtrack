//! This module defines the handler for retrieving the tracker page.

use crate::app::AppState;

use std::sync::{Arc, RwLock};

use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;

use tracing::instrument;

/// Serves the tracker page.
/// - `State(state)` is the shared application state.
///
/// # Panics
///
/// Panics if the `RwLock` becomes poisoned.
#[instrument(skip_all)]
pub async fn tracker(State(state): State<Arc<RwLock<AppState>>>) -> Response {
    let pages = &state.read().expect("Poisoned lock.").pages;
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(pages["tracker"].clone()))
        .expect("Failed to build response.")
}
