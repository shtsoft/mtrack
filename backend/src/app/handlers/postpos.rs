//! This module defines the handler for getting the postpos page.

use crate::app::AppState;

use std::fs;
use std::sync::{Arc, RwLock};

use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;

use tracing::instrument;

/// Returns the postpos page.
#[instrument(skip_all)]
pub async fn postpos(State(state): State<Arc<RwLock<AppState>>>) -> Response {
    let state = &state.read().expect("Poisoned lock.");
    match fs::read_to_string(state.dist.clone() + "/postpos/index.html") {
        Ok(postpos) => Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(postpos))
            .expect("Impossible error when building response."),
        Err(err) => {
            tracing::error!("Failed to load postpos page: {:?}", err);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to load postpos page."))
                .expect("Impossible error when building response.")
        }
    }
}
