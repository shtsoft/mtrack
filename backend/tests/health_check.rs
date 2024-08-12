mod helpers;

use helpers::ADDR;
use helpers::{make_clients, make_config};

#[tokio::test]
async fn test_health_check() {
    std::thread::spawn(|| {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Setting up tokio runtime failed");
        let _ = rt.block_on(mtrack::run(make_config()));
    });

    let (client, _) = make_clients();

    let response_ok = client
        .get(format!("https://{}/health_check", ADDR))
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_ok.status().is_success());
}
