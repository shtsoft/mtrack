//! This module defines the handler for retrieving user positions.

use crate::app::handlers::utils::extract_session_id;
use crate::app::AppState;

use std::sync::{Arc, RwLock};

use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;

use hyper::header::HeaderMap;

use tracing::instrument;

/// Retrieves the current positions after validating the session.
/// - `headers` are the incoming HTTP headers.
/// - `State(state)` is the shared application state.
///
/// # Panics
///
/// Panics if the `RwLock` becomes poisoned.
#[instrument(skip_all)]
pub async fn get_positions(
    headers: HeaderMap,
    State(state): State<Arc<RwLock<AppState>>>,
) -> Response {
    match extract_session_id(&headers) {
        Ok(session_id) => {
            let state = &state.read().expect("Poisoned lock.");
            if state.sessions.contains_key(&session_id) {
                match serde_json::to_string(&state.positions) {
                    Ok(positions) => Response::builder()
                        .status(StatusCode::OK)
                        .body(Body::from(positions))
                        .expect("Failed to build response."),
                    Err(err) => {
                        tracing::error!("Failed to serialize positions: {:?}", err);
                        Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from("Failed to generate position data."))
                            .expect("Failed to build response.")
                    }
                }
            } else {
                tracing::warn!("Unauthorized attempt to access positions without a valid session");
                Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from("You must be logged in to retrieve positions."))
                    .expect("Failed to build response.")
            }
        }
        Err(response) => response,
    }
}
