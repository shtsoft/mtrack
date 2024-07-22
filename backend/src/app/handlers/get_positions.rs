use crate::app::handlers::utils::extract_session_id;
use crate::app::AppState;

use std::sync::{Arc, RwLock};

use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;

use hyper::header::HeaderMap;

pub async fn get_positions(
    headers: HeaderMap,
    State(state): State<Arc<RwLock<AppState>>>,
) -> Response {
    match extract_session_id(headers) {
        Ok(session_id) => {
            let state = &state.read().expect("Poisoned lock.");
            if state.sessions.contains_key(&session_id) {
                match serde_json::to_string(&state.positions) {
                    Ok(positions) => Response::builder()
                        .status(StatusCode::OK)
                        .body(Body::from(positions))
                        .expect("Impossible error when building response."),
                    Err(err) => {
                        tracing::error!("Failed to serialize positions: {:?}", err);
                        Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from("Failed to generate position data."))
                            .expect("Impossible error when building response.")
                    }
                }
            } else {
                tracing::warn!("Client trying to get positions without being logged in");
                Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from("You have to be logged in to get positions."))
                    .expect("Impossible error when building response.")
            }
        }
        Err(response) => response,
    }
}
