use axum::http::StatusCode;

use tracing::instrument;

#[instrument(skip_all)]
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
