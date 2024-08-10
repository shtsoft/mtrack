mod helpers;

use helpers::ADDR;
use helpers::{make_clients, make_config};

#[tokio::test]
async fn test_get_login() {
    std::thread::spawn(|| {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let _ = rt.block_on(mtrack::run(make_config()));
    });

    let (client, _) = make_clients();

    let response_ok_1 = client
        .get(format!("https://{}/", ADDR))
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_ok_1.status().is_success());

    let response_ok_2 = client
        .get(format!("https://{}/index.html", ADDR))
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_ok_2.status().is_success());
}
