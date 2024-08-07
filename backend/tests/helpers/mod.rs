use std::io::Read;

use reqwest::redirect::Policy;
use reqwest::{Certificate, Client, ClientBuilder};

pub const ADDR: &str = "127.0.0.1:10443";
#[allow(dead_code)]
pub const NAMEFOO: &str = "Foo";
#[allow(dead_code)]
pub const NAMEBAR: &str = "Bar";
#[allow(dead_code)]
pub const PASSWORDFOO: &str = "0000";
#[allow(dead_code)]
pub const PASSWORDBAR: &str = "1111";
#[allow(dead_code)]
pub const PASSWORDBAD: &str = "a";
#[allow(dead_code)]
pub const COORDINATES: &str = "{\"latitude\":0.0,\"longitude\":0.0}";
#[allow(dead_code)]
pub const BAD_COOKIE: &str = "foo=bar";
#[allow(dead_code)]
pub const BAD_CHAR: &str = "ß";
#[allow(dead_code)]
pub const BAD_ID_1: &str = "sessionID=xyz";
#[allow(dead_code)]
pub const BAD_ID_2: &str = "sessionID=1234";

const ROOT_CERT: &str = "data/root.crt";

pub fn make_clients() -> (Client, Client) {
    let mut buf = Vec::new();
    std::fs::File::open(ROOT_CERT)
        .expect("Failed to open root certificate")
        .read_to_end(&mut buf)
        .expect("Failed to read root certificate");
    let cert = Certificate::from_pem(&buf).expect("Failed to load root certificate");
    let client = ClientBuilder::new()
        .add_root_certificate(cert.clone())
        .redirect(Policy::none())
        .build()
        .expect("Failed to build client");
    let client_cookie = ClientBuilder::new()
        .add_root_certificate(cert)
        .redirect(Policy::none())
        .cookie_store(true)
        .build()
        .expect("Failed to build client");

    (client, client_cookie)
}
