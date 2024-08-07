//! This module defines the handler for getting the login page.

use crate::app::handlers::utils::check_for_login;
use crate::app::AppState;

use std::sync::{Arc, RwLock};

use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;

use hyper::header::HeaderMap;

use tracing::instrument;

/// Returns the login page.
#[instrument(skip_all)]
pub async fn get_login(headers: HeaderMap, State(state): State<Arc<RwLock<AppState>>>) -> Response {
    if let Some(response) = check_for_login(headers, state.clone()) {
        return response;
    }

    let pages = &state.read().expect("Poisoned lock.").pages;
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(pages["login"].clone()))
        .expect("Impossible error when building response.")
}
