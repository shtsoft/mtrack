//! This module defines some utility functions to handle requests.

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

/// Looks up a name to a given password in a users database and returns `None` if it was not found.
/// - `password` is the given password.
/// - `users` is the users database.
///
/// # Notes
///
/// This function is compute heavy.
pub fn lookup_name(password: &str, users: &Vec<UserEntry>) -> Option<Name> {
    for user in users {
        match bcrypt::verify(password, &user.hash) {
            Ok(verified) => {
                if verified {
                    return Some(user.name.clone());
                }
            }
            Err(err) => {
                tracing::error!("Failed to verify password of user {}: {:?}", user.name, err);
                continue;
            }
        };
    }

    None
}

/// Looks up a hash to a given name in a users database and returns `None` if it was not found.
/// - `name` is the given name.
/// - `users` is the users database.
pub fn lookup_hash(name: &str, users: &Vec<UserEntry>) -> Option<String> {
    for user in users {
        if user.name == name {
            return Some(user.hash.clone());
        }
    }
    None
}

/// Parses a cookie header into cookie jar.
/// - `cookies_value` is the cookie header.
///
/// # Errors
///
/// An error is returned if the cookie header contains chars other than visible ASCII.
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
                "Client showing cookies containing other chars than visible ASCII: {:?}",
                err
            );
            Err(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(
                    "Cookies have to be made up by visible ASCII chars only.",
                ))
                .expect("Impossible error when building response."))
        }
    }
}

/// Extracts the session ID cookie from the http headers.
/// - `headers` are the http headers.
///
/// # Errors
///
/// An error is returned if
/// - there is no cookie header.
/// - parsing the cookie header fails.
/// - there is no session ID cookie.
/// - the session ID cookie is not an integer.
pub fn extract_session_id(headers: &HeaderMap) -> Result<SessionID, Response> {
    match headers.get(header::COOKIE) {
        Some(cookies_value) => match parse_cookies(cookies_value) {
            Ok(jar) => match jar.get(SESSION_ID_COOKIE_NAME) {
                Some(cookie) => match cookie.value().parse::<SessionID>() {
                    Ok(session_id) => Ok(session_id),
                    Err(err) => {
                        tracing::warn!(
                            "Client showing invalid '{}': {:?}",
                            SESSION_ID_COOKIE_NAME,
                            err
                        );
                        Err(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Body::from(format!(
                                "The '{}' has to be an integer.",
                                SESSION_ID_COOKIE_NAME
                            )))
                            .expect("Impossible error when building response."))
                    }
                },
                None => Err(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from(format!(
                        "There is no '{}'-cookie.",
                        SESSION_ID_COOKIE_NAME
                    )))
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

/// Checks if a client is already logged in and if so returns a redirection response to the tracker.
/// - `headers` are the http headers.
/// - `state` is the application state.
pub fn check_for_login(headers: &HeaderMap, state: &Arc<RwLock<AppState>>) -> Option<Response> {
    if let Ok(session_id) = extract_session_id(headers) {
        let sessions = &state.read().expect("Poisoned lock.").sessions;
        if sessions.contains_key(&session_id) {
            tracing::info!("Client trying to log in while logged in");
            return Some(
                Response::builder()
                    .status(StatusCode::SEE_OTHER)
                    .header(header::LOCATION, "/tracker")
                    .body(Body::from("You are already logged in."))
                    .expect("Impossible error when building response."),
            );
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
