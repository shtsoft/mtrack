mod helpers;

use helpers::make_clients;
use helpers::{ADDR, COORDINATES, PASSWORDBAD, PASSWORDBAR};

const BODYBAD: &str = "a";

#[tokio::test]
async fn test_post_position() {
    let (client, _) = make_clients();

    let response_ok = client
        .post(format!("https://{}/positions/{}", ADDR, PASSWORDBAR))
        .body(COORDINATES)
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_ok.status().is_success());

    let response_bad_key = client
        .post(format!("https://{}/positions/{}", ADDR, PASSWORDBAD))
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_bad_key.status().is_client_error());

    let response_bad_body = client
        .post(format!("https://{}/positions/{}", ADDR, PASSWORDBAR))
        .body(BODYBAD)
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_bad_body.status().is_client_error());
}
