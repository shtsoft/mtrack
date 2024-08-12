mod helpers;

use helpers::{make_clients, make_config};
use helpers::{ADDR, BAD_CHAR, BAD_COOKIE, BAD_ID_1, BAD_ID_2, NAMEBAR, PASSWORDBAR};

use std::thread;

#[tokio::test]
async fn test_logout() {
    thread::spawn(|| {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Setting up tokio runtime failed");
        let _ = rt.block_on(mtrack::run(make_config()));
    });

    let (client, client_cookie) = make_clients();

    client_cookie
        .post(format!("https://{}/login", ADDR))
        .body(format!("name={}&password={}", NAMEBAR, PASSWORDBAR))
        .send()
        .await
        .expect("Failed to send request");
    let response_redirection = client_cookie
        .post(format!("https://{}/logout", ADDR))
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_redirection.status().is_redirection());

    let response_empty_header = client
        .post(format!("https://{}/logout", ADDR))
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_empty_header.status().is_client_error());

    let response_bad_cookie = client
        .post(format!("https://{}/logout", ADDR))
        .header(hyper::header::COOKIE, BAD_COOKIE)
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_bad_cookie.status().is_client_error());

    let response_bad_char = client
        .post(format!("https://{}/logout", ADDR))
        .header(hyper::header::COOKIE, BAD_CHAR)
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_bad_char.status().is_client_error());

    let response_bad_id = client
        .post(format!("https://{}/logout", ADDR))
        .header(hyper::header::COOKIE, BAD_ID_1)
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_bad_id.status().is_client_error());

    let response_bad_id_2 = client
        .post(format!("https://{}/logout", ADDR))
        .header(hyper::header::COOKIE, BAD_ID_2)
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_bad_id_2.status().is_client_error());
}
