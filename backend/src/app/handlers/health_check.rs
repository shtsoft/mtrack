//! This module defines the handler for checking the application's health status.

use axum::http::StatusCode;

use tracing::instrument;

/// Returns a status code indicating that the service is healthy.
///
/// # Notes
///
/// This endpoint is used by monitoring tools to verify that the application is running.
#[instrument(skip_all)]
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
