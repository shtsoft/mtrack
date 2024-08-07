//! This module defines the handler for logging in.

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

#[derive(Deserialize)]
struct Query {
    name: String,
    password: String,
}

/// Makes a session cookie for a newly logged in user.
/// - `name` is the name of the user who is logging in.
/// - `State(state)` is the application state.
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

/// Logs a user in if it is not already logged in.
/// - `headers` are the http headers.
/// - `State(state)` is the application state.
/// - `body` is the http body of the request.
#[instrument(skip_all)]
pub async fn post_login(
    headers: HeaderMap,
    State(state): State<Arc<RwLock<AppState>>>,
    body: String,
) -> Response {
    if let Some(response) = check_for_login(headers, state.clone()) {
        return response;
    }

    let query: Query = match serde_qs::from_str(&body) {
        Ok(query) => query,
        Err(err) => {
            tracing::warn!("Client posting invalid login query: {:?}", err);
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("You must have valid login data."))
                .expect("Impossible error when building response.");
        }
    };
    let name = &query.name;
    let password = query.password;

    // The following indirection is here to prevent a deadlock arising from the lifetime of the
    // guard.
    let lookup = lookup_hash(name, &state.read().expect("Poisoned lock.").download_users);
    if let Some(hash) = lookup {
        let result = task::spawn_blocking(move || bcrypt::verify(password, &hash))
            .await
            .expect("Impossible error when verifying password.");
        match result {
            Ok(verified) => {
                if verified {
                    let session_cookie = make_session_cookie(name, &state);
                    tracing::info!("User {} has logged in successfully", name);
                    Response::builder()
                        .status(StatusCode::SEE_OTHER)
                        .header(header::LOCATION, "/tracker")
                        .header(header::SET_COOKIE, session_cookie)
                        .body(Body::from("Log in succeeded."))
                        .expect("Impossible error when building response.")
                } else {
                    tracing::warn!("User {} trying to log in with invalid password", name);
                    Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("You must have valid login data."))
                        .expect("Impossible error when building response.")
                }
            }
            Err(err) => {
                tracing::error!("Failed to verify password of user {}: {:?}", name, err);
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("Failed to validate login data."))
                    .expect("Impossible error when building response.")
            }
        }
    } else {
        tracing::warn!("Client trying to log in with invalid user name");
        Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("You must have valid login data."))
            .expect("Impossible error when building response.")
    }
}
