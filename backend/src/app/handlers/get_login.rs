//! This module defines the handler for retrieving the login page.

use crate::app::handlers::utils::check_for_login;
use crate::app::AppState;

use std::sync::{Arc, RwLock};

use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;

use hyper::header::HeaderMap;

use tracing::instrument;

/// Serves the login page.
/// - `headers` are the incoming HTTP headers.
/// - `State(state)` is the shared application state.
///
/// # Panics
///
/// Panics if the internal `check_for_login` utility panics.
#[instrument(skip_all)]
pub async fn get_login(headers: HeaderMap, State(state): State<Arc<RwLock<AppState>>>) -> Response {
    if let Some(response) = check_for_login(&headers, &state) {
        return response;
    }

    let pages = &state.read().expect("Poisoned lock.").pages;
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(pages["login"].clone()))
        .expect("Failed to build response.")
}
