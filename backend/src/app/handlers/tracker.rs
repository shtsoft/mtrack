//! This module defines the handler for getting the tracker page.

use axum::http::StatusCode;

use tracing::instrument;

/// Returns the tracker page.
#[instrument(skip_all)]
pub async fn tracker() -> (StatusCode, String) {
    (StatusCode::OK, String::new())
}
