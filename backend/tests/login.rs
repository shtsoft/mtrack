mod helpers;

use helpers::make_clients;
use helpers::{ADDR, NAMEFOO, PASSWORDBAD, PASSWORDFOO};

const NAMEBAD: &str = "F";

#[tokio::test]
async fn test_login() {
    let (client, client_cookie) = make_clients();

    let response_ok = client
        .post(format!(
            "https://{}/login?name={}&password={}",
            ADDR, NAMEFOO, PASSWORDFOO
        ))
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_ok.status().is_success());

    client_cookie
        .post(format!(
            "https://{}/login?name={}&password={}",
            ADDR, NAMEFOO, PASSWORDFOO
        ))
        .send()
        .await
        .expect("Failed to send request");
    let response_redirection = client_cookie
        .post(format!(
            "https://{}/login?name={}&password={}",
            ADDR, NAMEFOO, PASSWORDFOO
        ))
        .send()
        .await
        .expect("Failed to send request");
    println!("{:?}", response_redirection);
    assert!(response_redirection.status().is_redirection());

    let response_bad_name = client
        .post(format!(
            "https://{}/login?name={}&password={}",
            ADDR, NAMEBAD, PASSWORDFOO
        ))
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_bad_name.status().is_client_error());

    let response_bad_password = client
        .post(format!(
            "https://{}/login?name={}&password={}",
            ADDR, NAMEFOO, PASSWORDBAD
        ))
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_bad_password.status().is_client_error());
}
