mod helpers;

use helpers::make_clients;
use helpers::{
    ADDR, BAD_CHAR, BAD_COOKIE, BAD_ID_1, BAD_ID_2, COORDINATES, NAMEFOO, PASSWORDBAR, PASSWORDFOO,
};

use hyper::header;

#[tokio::test]
async fn test_get_positions() {
    let (client, client_cookie) = make_clients();

    client_cookie
        .post(format!("https://{}/login", ADDR))
        .body(format!("name={}&password={}", NAMEFOO, PASSWORDFOO))
        .send()
        .await
        .expect("Failed to send request");
    client_cookie
        .post(format!("https://{}/positions/{}", ADDR, PASSWORDBAR))
        .body(COORDINATES)
        .send()
        .await
        .expect("Failed to send request");
    let response_ok = client_cookie
        .get(format!("https://{}/positions", ADDR))
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_ok.status().is_success());
    let response_ok_body = response_ok
        .text()
        .await
        .expect("Failed to get response text");
    assert_eq!(format!("{{\"bar\":{}}}", COORDINATES), response_ok_body);

    let response_empty_header_1 = client
        .get(format!("https://{}/positions", ADDR))
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_empty_header_1.status().is_client_error());

    let response_bad_cookie = client
        .get(format!("https://{}/positions", ADDR))
        .header(header::COOKIE, BAD_COOKIE)
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_bad_cookie.status().is_client_error());

    let response_bad_char = client
        .get(format!("https://{}/positions", ADDR))
        .header(header::COOKIE, BAD_CHAR)
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_bad_char.status().is_client_error());

    let response_bad_id_1 = client
        .get(format!("https://{}/positions", ADDR))
        .header(header::COOKIE, BAD_ID_1)
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_bad_id_1.status().is_client_error());

    let response_bad_id_2 = client
        .get(format!("https://{}/positions", ADDR))
        .header(hyper::header::COOKIE, BAD_ID_2)
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_bad_id_2.status().is_client_error());
}
