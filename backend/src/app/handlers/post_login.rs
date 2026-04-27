//! This module defines the handler for user login.

use crate::app::handlers::utils::{check_for_login, lookup_hash};
use crate::app::{AppState, SessionState};
use crate::app::{SESSION_ID_COOKIE_NAME, SESSION_TTL};

use std::sync::{Arc, RwLock};

use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;

use cookie::{Cookie, SameSite};

use hyper::header;
use hyper::header::HeaderMap;

use rand::prelude::*;

use serde::Deserialize;

use tokio::task;

use tracing::instrument;

/// Represents the login query parameters.
#[derive(Deserialize)]
struct Query {
    name: String,
    password: String,
}

/// Creates a session and returns a session ID cookie for the newly logged-in user.
/// - `name` is the username of the user logging in.
/// - `state` is he shared application state.
///
/// # Panics
///
/// Panics if the `RwLock` becomes poisoned.
fn make_session_cookie(name: &str, state: &Arc<RwLock<AppState>>) -> String {
    let mut rng = rand::thread_rng();
    let session_id: u128 = rng.gen();

    let sessions = &mut state.write().expect("Poisoned lock.").sessions;
    let _ = sessions.insert(
        session_id,
        SessionState {
            name: name.to_string(),
            ttl: SESSION_TTL,
        },
    );

    Cookie::build((SESSION_ID_COOKIE_NAME, session_id.to_string()))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .to_string()
}

/// Processes the login credentials.
/// - `name` is the username of the user.
/// - `password` is the password provided by the user.
/// - `state` is the shared application state.
///
/// # Panics
///
/// Panics if the `RwLock` is poisoned or if `make_session_cookie` panics.
async fn login(name: &str, password: String, state: Arc<RwLock<AppState>>) -> Response {
    // This indirection prevents a deadlock by ensuring the read guard is dropped before
    // potentially acquiring a write lock in make_session_cookie.
    let lookup = lookup_hash(name, &state.read().expect("Poisoned lock.").download_users);
    if let Some(hash) = lookup {
        let result = task::spawn_blocking(move || bcrypt::verify(password, &hash))
            .await
            .expect("Failed to verify password.");
        match result {
            Ok(true) => {
                let session_cookie = make_session_cookie(name, &state);
                tracing::info!("User {} has logged in successfully", name);
                Response::builder()
                    .status(StatusCode::SEE_OTHER)
                    .header(header::LOCATION, "/tracker")
                    .header(header::SET_COOKIE, session_cookie)
                    .body(Body::from("Login successful."))
                    .expect("Failed to build response.")
            }
            Ok(false) => {
                tracing::warn!("User {} attempted to log in with an incorrect password", name);
                Response::builder()
                    .status(StatusCode::SEE_OTHER)
                    .header(header::LOCATION, "/login")
                    .body(Body::from("You must have valid login data."))
                    .expect("Failed to build response.")
            }
            Err(err) => {
                tracing::error!("Failed to verify password for user {}: {:?}", name, err);
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("Failed to validate login data."))
                    .expect("Failed to build response.")
            }
        }
    } else {
        tracing::warn!("Login attempt with non-existent username: {}", name);
        Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(header::LOCATION, "/login")
            .body(Body::from("You must have valid login data."))
            .expect("Failed to build response.")
    }
}

/// Handles the login request if the user is not already authenticated.
/// - `headers` are the incoming HTTP headers.
/// - `State(state)` is the shared application state.
/// - `body` is the HTTP request body containing credentials.
///
/// # Panics
///
/// Panics if `check_for_login` or `login` panics.
#[instrument(skip_all)]
pub async fn post_login(
    headers: HeaderMap,
    State(state): State<Arc<RwLock<AppState>>>,
    body: String,
) -> Response {
    if let Some(response) = check_for_login(&headers, &state) {
        return response;
    }

    let query: Query = match serde_qs::from_str(&body) {
        Ok(query) => query,
        Err(err) => {
            tracing::warn!("Client sent an invalid login query: {:?}", err);
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("You must have valid login data."))
                .expect("Failed to build response.");
        }
    };

    login(&query.name, query.password, state).await
}
