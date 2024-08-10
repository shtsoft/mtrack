mod helpers;

use helpers::{make_clients, make_config};
use helpers::{ADDR, NAMEFOO, PASSWORDBAD, PASSWORDFOO};

const NAMEBAD: &str = "F";

#[tokio::test]
async fn test_post_login() {
    std::thread::spawn(|| {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let _ = rt.block_on(mtrack::run(make_config()));
    });

    let (client, client_cookie) = make_clients();

    let response_redirection_1 = client
        .post(format!("https://{}/login", ADDR))
        .body(format!("name={}&password={}", NAMEFOO, PASSWORDFOO))
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_redirection_1.status().is_redirection());

    client_cookie
        .post(format!("https://{}/login", ADDR))
        .body(format!("name={}&password={}", NAMEFOO, PASSWORDFOO))
        .send()
        .await
        .expect("Failed to send request");
    let response_redirection_2 = client_cookie
        .post(format!("https://{}/login", ADDR))
        .body(format!("name={}&password={}", NAMEFOO, PASSWORDFOO))
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_redirection_2.status().is_redirection());

    let response_bad_name = client
        .post(format!("https://{}/login", ADDR))
        .body(format!("name={}&password={}", NAMEBAD, PASSWORDFOO))
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_bad_name.status().is_client_error());

    let response_bad_password = client
        .post(format!("https://{}/login", ADDR))
        .body(format!("name={}&password={}", NAMEFOO, PASSWORDBAD))
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_bad_password.status().is_client_error());
}
