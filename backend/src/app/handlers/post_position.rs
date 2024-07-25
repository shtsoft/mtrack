//! This module defines the handler for posting positions.

use crate::app::handlers::utils::lookup_name;
use crate::app::{AppState, Coordinates};

use std::sync::{Arc, RwLock};

use axum::extract::{Path, State};
use axum::http::StatusCode;

use tokio::task;

use tracing::instrument;

/// Posts the position of a user with a valid key.
/// - `Path(path)` is the path of the URL.
/// - `State(state)` is the application state.
/// - `body` is the http body of the request.
#[instrument(skip_all)]
pub async fn post_position(
    Path(key): Path<String>,
    State(state): State<Arc<RwLock<AppState>>>,
    body: String,
) -> (StatusCode, String) {
    let state_clone = state.clone();
    let result = task::spawn_blocking(move || {
        lookup_name(
            &key,
            &state_clone.read().expect("Poisoned lock.").upload_users,
        )
    })
    .await
    .expect("Impossible error when looking up name.");
    if let Some(name) = result {
        match serde_json::from_str::<Coordinates>(&body) {
            Ok(coordinates) => {
                let positions = &mut state.write().expect("Poisoned lock.").positions;
                if positions.insert(name.clone(), coordinates).is_none() {
                    tracing::info!("Start tracking position of user {}", name);
                };

                (StatusCode::OK, String::new())
            }
            Err(err) => {
                tracing::warn!("Client posting invalid coordinates: {:?}", err);
                (
                    StatusCode::BAD_REQUEST,
                    "Coordinates must be numbers.".to_string(),
                )
            }
        }
    } else {
        tracing::warn!("Client trying to post coordinates with invalid key");
        (
            StatusCode::BAD_REQUEST,
            "You must have valid key to post coordinates.".to_string(),
        )
    }
}
