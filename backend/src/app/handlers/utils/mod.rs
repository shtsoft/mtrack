//! This module defines utility functions for handling requests.

use crate::app::SESSION_ID_COOKIE_NAME;
use crate::app::{AppState, UserEntry};
use crate::app::{Name, SessionID};

use std::sync::{Arc, RwLock};

use axum::body::Body;
use axum::http::StatusCode;
use axum::response::Response;

use cookie::{Cookie, CookieJar};

use hyper::header;
use hyper::header::{HeaderMap, HeaderValue};

/// Looks up a name for a given password in a user database and returns `None` if it was not found.
/// - `password` is the given password.
/// - `users` is the user database.
///
/// # Notes
///
/// This function is computationally expensive.
pub fn lookup_name(password: &str, users: &Vec<UserEntry>) -> Option<Name> {
    for user in users {
        match bcrypt::verify(password, &user.hash) {
            Ok(verified) => {
                if verified {
                    return Some(user.name.clone());
                }
            }
            Err(err) => {
                tracing::error!("Failed to verify password for user {}: {:?}", user.name, err);
                continue;
            }
        };
    }

    None
}

/// Looks up a hash for a given name in a user database and returns `None` if it was not found.
/// - `name` is the given name.
/// - `users` is the user database.
pub fn lookup_hash(name: &str, users: &Vec<UserEntry>) -> Option<String> {
    for user in users {
        if user.name == name {
            return Some(user.hash.clone());
        }
    }
    None
}

/// Parses a cookie header into a cookie jar.
/// - `cookies_value` is the cookie header.
///
/// # Errors
///
/// An error is returned if the cookie header contains characters other than visible ASCII.
#[allow(clippy::result_large_err)]
fn parse_cookies(cookies_value: &HeaderValue) -> Result<CookieJar, Response> {
    match cookies_value.to_str() {
        Ok(cookies_str) => {
            let mut jar = CookieJar::new();
            for cookie in Cookie::split_parse(cookies_str.to_string()) {
                match cookie {
                    Ok(c) => jar.add(c),
                    Err(_) => continue,
                };
            }
            Ok(jar)
        }
        Err(err) => {
            tracing::warn!(
                "Client provided cookies containing characters other than visible ASCII: {:?}",
                err
            );
            Err(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(
                    "Cookies must consist of visible ASCII characters only.",
                ))
                .expect("Impossible error when building response."))
        }
    }
}

/// Extracts the session ID cookie from the HTTP headers.
/// - `headers` are the HTTP headers.
///
/// # Errors
///
/// An error is returned if:
/// - there is no cookie header.
/// - parsing the cookie header fails.
/// - there is no session ID cookie.
/// - the session ID cookie is not an integer.
#[allow(clippy::result_large_err)]
pub fn extract_session_id(headers: &HeaderMap) -> Result<SessionID, Response> {
    match headers.get(header::COOKIE) {
        Some(cookies_value) => match parse_cookies(cookies_value) {
            Ok(jar) => match jar.get(SESSION_ID_COOKIE_NAME) {
                Some(cookie) => match cookie.value().parse::<SessionID>() {
                    Ok(session_id) => Ok(session_id),
                    Err(err) => {
                        tracing::warn!(
                            "Client provided invalid '{}': {:?}",
                            SESSION_ID_COOKIE_NAME,
                            err
                        );
                        Err(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Body::from(format!(
                                "The '{}' must be an integer.",
                                SESSION_ID_COOKIE_NAME
                            )))
                            .expect("Failed to build response."))
                    }
                },
                None => Err(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from(format!(
                        "The '{}' cookie is missing.",
                        SESSION_ID_COOKIE_NAME
                    )))
                    .expect("Failed to build response.")),
            },
            Err(response) => Err(response),
        },
        None => Err(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(format!(
                "No cookies were found, specifically the '{}' cookie.",
                SESSION_ID_COOKIE_NAME
            )))
            .expect("Failed to build response.")),
    }
}

/// Checks if a client is already logged in and, if so, returns a redirection response to the tracker.
/// - `headers` are the HTTP headers.
/// - `state` is the application state.
///
/// # Panics
///
/// This function panics if the `RwLock` is poisoned.
pub fn check_for_login(headers: &HeaderMap, state: &Arc<RwLock<AppState>>) -> Option<Response> {
    if let Ok(session_id) = extract_session_id(headers) {
        let sessions = &state.read().expect("Poisoned lock.").sessions;
        if sessions.contains_key(&session_id) {
            tracing::info!("Client attempted to log in while already logged in");
            return Some(
                Response::builder()
                    .status(StatusCode::SEE_OTHER)
                    .header(header::LOCATION, "/tracker")
                    .body(Body::from("You are already logged in."))
                    .expect("Failed to build response."),
            );
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::app::SessionState;

    use std::collections::HashMap;

    use hyper::header;
    use hyper::header::{HeaderMap, HeaderValue};

    const HASHFOO: &str = "$2a$04$hu2ZTKvt/3px6oAj5XVxiOs1mRQcinxBJgGFNLF80JUSJQyMdWQma";
    const PASSWORDFOO: &str = "0000";
    const PASSWORDBAD: &str = "0";

    const COOKIE: &str = "a=123; b=456";
    const GOOD_ID: &str = "sessionID=1234";
    const BAD_CHAR: &str = "ß";
    const BAD_COOKIE: &str = "foo=bar";
    const BAD_ID: &str = "sessionID=xyz";

    const GOOD_ID_INT: u128 = 1234;

    #[test]
    fn test_lookup_name() {
        let mut users = Vec::new();
        let user = UserEntry {
            name: "foo".to_string(),
            hash: HASHFOO.to_string(),
        };
        users.push(user);

        assert!(lookup_name(PASSWORDFOO, &users).is_some());
        assert!(lookup_name(PASSWORDBAD, &users).is_none());
    }

    #[test]
    fn test_lookup_hash() {
        let mut users = Vec::new();
        let user = UserEntry {
            name: "Foo".to_string(),
            hash: HASHFOO.to_string(),
        };
        users.push(user);

        assert!(lookup_hash("Foo", &users).is_some());
        assert!(lookup_hash("Bar", &users).is_none());
    }

    #[test]
    fn test_parse_cookies() {
        assert!(parse_cookies(&HeaderValue::from_str(COOKIE).unwrap()).is_ok());
        assert!(parse_cookies(&HeaderValue::from_str(BAD_CHAR).unwrap()).is_err());
    }

    #[test]
    fn test_extract_session_id() {
        let mut headers = HeaderMap::new();
        headers.insert(header::COOKIE, GOOD_ID.parse().unwrap());
        assert!(extract_session_id(&headers).is_ok());

        let headers = HeaderMap::new();
        assert!(extract_session_id(&headers).is_err());

        let mut headers = HeaderMap::new();
        headers.insert(header::COOKIE, BAD_CHAR.parse().unwrap());
        assert!(extract_session_id(&headers).is_err());

        let mut headers = HeaderMap::new();
        headers.insert(header::COOKIE, BAD_COOKIE.parse().unwrap());
        assert!(extract_session_id(&headers).is_err());

        let mut headers = HeaderMap::new();
        headers.insert(header::COOKIE, BAD_ID.parse().unwrap());
        assert!(extract_session_id(&headers).is_err());
    }

    #[test]
    fn test_check_for_login() {
        let mut sessions = HashMap::new();
        sessions.insert(
            GOOD_ID_INT,
            SessionState {
                name: String::new(),
                ttl: 0,
            },
        );
        let app_state = Arc::new(RwLock::new(AppState {
            sessions,
            positions: HashMap::new(),
            download_users: Vec::new(),
            upload_users: Vec::new(),
            pages: HashMap::new(),
        }));

        let mut headers = HeaderMap::new();
        headers.insert(header::COOKIE, GOOD_ID.parse().unwrap());
        assert!(check_for_login(&headers, &app_state).is_some());

        let headers = HeaderMap::new();
        assert!(check_for_login(&headers, &app_state).is_none());
    }
}
