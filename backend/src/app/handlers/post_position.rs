//! This module defines the handler for posting user positions.

use crate::app::handlers::utils::lookup_name;
use crate::app::{AppState, Coordinates};

use std::sync::{Arc, RwLock};

use axum::extract::{Path, State};
use axum::http::StatusCode;

use tokio::task;

use tracing::instrument;

/// Updates the position for a user with a valid key.
/// - `Path(key)` is the unique key identifying the user.
/// - `State(state)` is the shared application state.
/// - `body` is the HTTP request body containing the coordinates.
///
/// # Panics
///
/// Panics if the `RwLock` becomes poisoned.
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
    .expect("Poisoned lock.");
    if let Some(name) = result {
        match serde_json::from_str::<Coordinates>(&body) {
            Ok(coordinates) => {
                let positions = &mut state.write().expect("Poisoned lock.").positions;
                if positions.insert(name.clone(), coordinates).is_none() {
                    tracing::info!("Started tracking position for user: {}", name);
                };

                (StatusCode::OK, String::new())
            }
            Err(err) => {
                tracing::warn!("Client submitted invalid coordinates: {:?}", err);
                (
                    StatusCode::BAD_REQUEST,
                    "Coordinates must be a pair of floats.".to_string(),
                )
            }
        }
    } else {
        tracing::warn!("Client attempted to post coordinates with an invalid key");
        (
            StatusCode::BAD_REQUEST,
            "You must have a valid key to post coordinates.".to_string(),
        )
    }
}
