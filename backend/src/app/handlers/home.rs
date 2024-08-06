//! This module defines the handler for getting the home page.

use axum::http::StatusCode;

use tracing::instrument;

/// Returns the home page.
#[instrument(skip_all)]
pub async fn home() -> (StatusCode, String) {
    (StatusCode::OK, String::new())
}
