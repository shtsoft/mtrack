//! This module defines the handler for getting the login page.

use axum::http::StatusCode;

use tracing::instrument;

/// Returns the login page.
#[instrument(skip_all)]
pub async fn get_login() -> (StatusCode, String) {
    (StatusCode::OK, String::new())
}
