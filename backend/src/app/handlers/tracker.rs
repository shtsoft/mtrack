//! This module defines the handler for getting the tracker page.

use crate::app::AppState;

use std::sync::{Arc, RwLock};

use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;

use tracing::instrument;

/// Returns the tracker page.
///
/// # Panics
///
/// A panic is caused if there is an issue with the `RwLock`.
#[instrument(skip_all)]
pub async fn tracker(State(state): State<Arc<RwLock<AppState>>>) -> Response {
    let pages = &state.read().expect("Poisoned lock.").pages;
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(pages["tracker"].clone()))
        .expect("Impossible error when building response.")
}
