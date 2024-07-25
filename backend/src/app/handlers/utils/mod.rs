//! This module defines some utility functions to handle requests.

use crate::app::UserEntry;
use crate::app::SESSION_ID_COOKIE_NAME;

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
pub fn lookup_name(password: &str, users: &Vec<UserEntry>) -> Option<String> {
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

/// Extracts the session id cookie from the http headers.
/// - `headers` are the http headers.
///
/// # Errors
///
/// An error is returned if
/// - there is no cookie header.
/// - parsing the cookie header fails.
/// - there is no session id cookie.
/// - the session id cookie is not an integer.
pub fn extract_session_id(headers: HeaderMap) -> Result<u128, Response> {
    match headers.get(header::COOKIE) {
        Some(cookies_value) => match parse_cookies(cookies_value) {
            Ok(jar) => match jar.get(SESSION_ID_COOKIE_NAME) {
                Some(cookie) => match cookie.value().parse::<u128>() {
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
