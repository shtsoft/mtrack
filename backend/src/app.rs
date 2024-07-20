use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

use axum::body::Body;
use axum::extract::{Path, Query, Request, State};
use axum::http::StatusCode;
use axum::response::Response;
use axum::routing::{get, post};
use axum::Router;

use cookie::{Cookie, CookieJar, Expiration, SameSite};

use hyper::body::Incoming;
use hyper::header;
use hyper::header::{HeaderMap, HeaderValue};
use hyper::server::conn::http1;
use hyper::service;

use hyper_util::rt::TokioIo;

use rand::prelude::*;

use serde::Deserialize;

use time::OffsetDateTime;

use tokio::net::TcpStream;

use tokio_rustls::server::TlsStream;

use tower::Service;

const SESSION_TTL: u8 = 24;
const SESSION_TTL_UNIT: Duration = Duration::from_secs(3600);

type Coordinates = usize;

#[derive(Deserialize)]
pub struct UserEntry {
    name: String,
    hash: String,
}

pub struct AppState {
    pub sessions: HashMap<u128, u8>,
    pub download_users: Vec<UserEntry>,
    pub upload_users: Vec<UserEntry>,
    pub positions: HashMap<String, Coordinates>,
}

fn parse_cookies(cookies_value: &HeaderValue) -> Result<CookieJar, Response> {
    let cookies_str = match cookies_value.to_str() {
        Ok(c) => c,
        Err(err) => {
            tracing::warn!(
                "Client showing cookies with non-visible ASCII chars: {:?}",
                err
            );
            return Err(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(
                    "Cookies have to be made up by visible ASCII chars only.",
                ))
                .expect("Impossible error when building response."));
        }
    };

    let mut jar = CookieJar::new();
    for cookie in Cookie::split_parse(cookies_str.to_string()) {
        match cookie {
            Ok(c) => jar.add(c),
            Err(_) => continue,
        };
    }

    Ok(jar)
}

fn extract_session_id(headers: HeaderMap) -> Result<u128, Response> {
    match headers.get(header::COOKIE) {
        Some(cookies_value) => match parse_cookies(cookies_value) {
            Ok(jar) => match jar.get("sessionID") {
                Some(cookie) => match cookie.value().parse::<u128>() {
                    Ok(session_id) => Ok(session_id),
                    Err(err) => {
                        tracing::warn!("Client showing invalid 'sessionID': {:?}", err);
                        Err(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Body::from("The 'sessionID' has to be an integer."))
                            .expect("Impossible error when building response."))
                    }
                },
                None => Err(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from("There is no 'sessionID'-cookie."))
                    .expect("Impossible error when building response.")),
            },
            Err(response) => Err(response),
        },
        None => Err(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("The are no cookies."))
            .expect("Impossible error when building response.")),
    }
}

async fn handler_logout(
    headers: HeaderMap,
    State(state): State<Arc<RwLock<AppState>>>,
) -> Response {
    fn delete_session_cookie(
        session_id: u128,
        state: &Arc<RwLock<AppState>>,
    ) -> Result<String, Response> {
        let sessions = &mut state.write().expect("Poisoned lock.").sessions;
        if sessions.remove(&session_id).is_none() {
            tracing::warn!("Client trying to log out from non-existing session");
            return Err(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Your session does not exist."))
                .expect("Impossible error when building response."));
        }

        Ok(Cookie::build(("sessionID", "deleted"))
            .expires(Expiration::DateTime(OffsetDateTime::UNIX_EPOCH))
            .path("/")
            .secure(true)
            .http_only(true)
            .same_site(SameSite::Strict)
            .to_string())
    }

    match extract_session_id(headers) {
        Ok(session_id) => match delete_session_cookie(session_id, &state) {
            Ok(delete_session_cookie) => {
                tracing::info!("A user has logged out successfully");
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

async fn handler_login(
    Query(query): Query<HashMap<String, String>>,
    headers: HeaderMap,
    State(state): State<Arc<RwLock<AppState>>>,
) -> Response {
    fn lookup(name: &str, users: &Vec<UserEntry>) -> Option<String> {
        for user in users {
            if user.name == name {
                return Some(user.hash.clone());
            }
        }
        None
    }

    fn make_session_cookie(state: &Arc<RwLock<AppState>>) -> String {
        let mut rng = rand::thread_rng();
        let session_id: u128 = rng.gen();

        let sessions = &mut state.write().expect("Poisoned lock.").sessions;
        let _ = sessions.insert(session_id, SESSION_TTL);

        Cookie::build(("sessionID", session_id.to_string()))
            .path("/")
            .secure(true)
            .http_only(true)
            .same_site(SameSite::Strict)
            .to_string()
    }

    if let Ok(session_id) = extract_session_id(headers) {
        let sessions = &state.read().expect("Poisoned lock.").sessions;
        if sessions.contains_key(&session_id) {
            tracing::info!("Client trying to log in while logged in");
            return Response::builder()
                .status(StatusCode::SEE_OTHER)
                .header(header::LOCATION, "/map")
                .body(Body::from("You are already logged in."))
                .expect("Impossible error when building response.");
        }
    }

    let name = &query["name"];
    let password = &query["password"];

    // The following indirection is here to prevent a deadlock arising from the lifetime of the
    // guard.
    let lookup = lookup(name, &state.read().expect("Poisoned lock.").download_users);
    if let Some(hash) = lookup {
        let verified = match bcrypt::verify(password, &hash) {
            Ok(b) => b,
            Err(err) => {
                tracing::error!("Failed to verify password of user {}: {:?}", name, err);
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("Failed to validate login data."))
                    .expect("Impossible error when building response.");
            }
        };
        if verified {
            let session_cookie = make_session_cookie(&state);
            tracing::info!("User {} has logged in successfully", name);
            Response::builder()
                .status(StatusCode::OK)
                .header(header::SET_COOKIE, session_cookie)
                .body(Body::from("Log in succeeded."))
                .expect("Impossible error when building response.")
        } else {
            tracing::warn!("User {} trying to login with invalid password", name);
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("You must have valid login data."))
                .expect("Impossible error when building response.")
        }
    } else {
        tracing::warn!("Client trying to login with invalid user name");
        Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("You must have valid login data."))
            .expect("Impossible error when building response.")
    }
}

async fn handler_get_positions(
    headers: HeaderMap,
    State(state): State<Arc<RwLock<AppState>>>,
) -> Response {
    match extract_session_id(headers) {
        Ok(session_id) => {
            let state = &state.read().expect("Poisoned lock.");
            if state.sessions.contains_key(&session_id) {
                let positions = serde_json::to_string(&state.positions)
                    .expect("Impossible serialization error.");
                Response::builder()
                    .status(StatusCode::OK)
                    .body(Body::from(positions))
                    .expect("Impossible error when building response.")
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

async fn handler_post_position(
    Path(key): Path<String>,
    State(state): State<Arc<RwLock<AppState>>>,
    body: String,
) -> (StatusCode, String) {
    fn lookup(password: &str, users: &Vec<UserEntry>) -> Option<String> {
        for user in users {
            let verified = match bcrypt::verify(password, &user.hash) {
                Ok(b) => b,
                Err(err) => {
                    tracing::error!("Failed to verify password of user {}: {:?}", user.name, err);
                    continue;
                }
            };
            if verified {
                return Some(user.name.clone());
            }
        }

        None
    }

    // The following indirection is here to prevent a deadlock arising from the lifetime of the
    // guard.
    let lookup = lookup(&key, &state.read().expect("Poisoned lock.").upload_users);
    if let Some(name) = lookup {
        let coordinates = match body.parse::<Coordinates>() {
            Ok(coordinates) => coordinates,
            Err(err) => {
                tracing::warn!("Client posting invalid coordinates: {:?}", err);
                return (
                    StatusCode::BAD_REQUEST,
                    "Coordinates must be numbers.".to_string(),
                );
            }
        };

        let positions = &mut state.write().expect("Poisoned lock.").positions;
        if positions.insert(name.clone(), coordinates).is_none() {
            tracing::info!("Start tracking position of user {}", name);
        };

        (StatusCode::OK, String::new())
    } else {
        tracing::warn!("Client trying to post coordinates with invalid key");
        (
            StatusCode::BAD_REQUEST,
            "You must have valid key to post coordinates.".to_string(),
        )
    }
}

async fn handler_health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn server(tls_socket: TlsStream<TcpStream>, state: Arc<RwLock<AppState>>) {
    fn prune_sessions(state: &Arc<RwLock<AppState>>) {
        let mut dead_sessions = Vec::new();

        let sessions = &mut state.write().expect("Poisoned lock.").sessions;
        for (session, ttl) in sessions {
            if *ttl > 0 {
                *ttl -= 1;
            } else {
                dead_sessions.push(session);
            }
        }

        let sessions = &mut state.write().expect("Poisoned lock.").sessions;
        for session in dead_sessions {
            sessions.remove(session);
        }
    }

    tracing::debug!("TcpStream from proxy to downstream: {:?}", tls_socket);

    tracing::info!("Start serving connection");

    let state_clone = state.clone();
    thread::spawn(move || loop {
        thread::sleep(SESSION_TTL_UNIT);
        prune_sessions(&state_clone);
    });

    let app = Router::new()
        .route("/health_check", get(handler_health_check))
        .route("/positions/:key", post(handler_post_position))
        .route("/positions", get(handler_get_positions))
        .route("/login", post(handler_login))
        .route("/logout", post(handler_logout))
        .with_state(state);

    if let Err(err) = http1::Builder::new()
        .serve_connection(
            TokioIo::new(tls_socket),
            service::service_fn(move |request: Request<Incoming>| app.clone().call(request)),
        )
        .await
    {
        tracing::error!("Failed to serve connection: {:?}", err);
    }
}
