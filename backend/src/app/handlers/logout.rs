//! This module defines the handler for logging out.

use crate::app::handlers::utils::extract_session_id;
use crate::app::AppState;
use crate::app::SESSION_ID_COOKIE_NAME;

use std::sync::{Arc, RwLock};

use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;

use cookie::{Cookie, Expiration, SameSite};

use hyper::header;
use hyper::header::HeaderMap;

use time::OffsetDateTime;

use tracing::instrument;

/// Removes a session and return a cookie deleting the session id cookie on client.
/// - `session_id` is the id of the session which is to be removed.
/// - `State(state)` is the application state.
fn delete_session_cookie(
    session_id: u128,
    state: &Arc<RwLock<AppState>>,
) -> Result<(String, String), Response> {
    let sessions = &mut state.write().expect("Poisoned lock.").sessions;
    match sessions.remove(&session_id) {
        Some(session_state) => Ok((
            Cookie::build((SESSION_ID_COOKIE_NAME, "deleted"))
                .expires(Expiration::DateTime(OffsetDateTime::UNIX_EPOCH))
                .path("/")
                .secure(true)
                .http_only(true)
                .same_site(SameSite::Strict)
                .to_string(),
            session_state.name,
        )),
        None => {
            tracing::warn!("Client trying to log out from non-existing session");
            Err(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Your session does not exist."))
                .expect("Impossible error when building response."))
        }
    }
}

/// Logs a logged in user out.
/// - `headers` are the http headers.
/// - `State(state)` is the application state.
#[instrument(skip_all)]
pub async fn logout(headers: HeaderMap, State(state): State<Arc<RwLock<AppState>>>) -> Response {
    match extract_session_id(headers) {
        Ok(session_id) => match delete_session_cookie(session_id, &state) {
            Ok((delete_session_cookie, name)) => {
                tracing::info!("User {} has logged out successfully", name);
                Response::builder()
                    .status(StatusCode::OK)
                    .header(header::SET_COOKIE, delete_session_cookie)
                    .body(Body::from("Log out succeeded."))
                    .expect("Impossible error when building response.")
            }
            Err(response) => response,
        },
        Err(response) => response,
    }
}
