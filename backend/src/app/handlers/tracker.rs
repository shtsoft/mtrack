//! This module defines the handler for getting the tracker page.

use crate::app::AppState;

use std::fs;
use std::sync::{Arc, RwLock};

use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;

use tracing::instrument;

/// Returns the tracker page.
#[instrument(skip_all)]
pub async fn tracker(State(state): State<Arc<RwLock<AppState>>>) -> Response {
    let state = &state.read().expect("Poisoned lock.");
    match fs::read_to_string(state.dist.clone() + "/tracker/index.html") {
        Ok(tracker) => Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(tracker))
            .expect("Impossible error when building response."),
        Err(err) => {
            tracing::error!("Failed to load tracker page: {:?}", err);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to load tracker page."))
                .expect("Impossible error when building response.")
        }
    }
}
