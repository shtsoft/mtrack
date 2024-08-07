//! This module defines the handler for getting the login page.

use crate::app::handlers::utils::extract_session_id;
use crate::app::AppState;

use std::fs;
use std::sync::{Arc, RwLock};

use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;

use hyper::header;
use hyper::header::HeaderMap;

use tracing::instrument;

/// Returns the login page.
#[instrument(skip_all)]
pub async fn get_login(headers: HeaderMap, State(state): State<Arc<RwLock<AppState>>>) -> Response {
    if let Ok(session_id) = extract_session_id(headers) {
        let sessions = &state.read().expect("Poisoned lock.").sessions;
        if sessions.contains_key(&session_id) {
            tracing::info!("Client trying to log in while logged in");
            return Response::builder()
                .status(StatusCode::SEE_OTHER)
                .header(header::LOCATION, "/tracker")
                .body(Body::from("You are already logged in."))
                .expect("Impossible error when building response.");
        }
    }

    let state = &state.read().expect("Poisoned lock.");
    match fs::read_to_string(state.dist.clone() + "/login/index.html") {
        Ok(login) => Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(login))
            .expect("Impossible error when building response."),
        Err(err) => {
            tracing::error!("Failed to load login page: {:?}", err);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to load login page."))
                .expect("Impossible error when building response.")
        }
    }
}
