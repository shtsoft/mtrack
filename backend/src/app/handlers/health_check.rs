//! This module defines the handler for checking the health of the application.

use axum::http::StatusCode;

use tracing::instrument;

/// Returns a status code that everything is all right.
///
/// # Notes
///
/// This endpoint can be used to check if the application is up.
#[instrument(skip_all)]
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
