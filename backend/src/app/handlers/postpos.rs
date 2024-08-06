//! This module defines the handler for getting the postpos page.

use axum::http::StatusCode;

use tracing::instrument;

/// Returns the postpos page.
#[instrument(skip_all)]
pub async fn postpos() -> (StatusCode, String) {
    (StatusCode::OK, String::new())
}
