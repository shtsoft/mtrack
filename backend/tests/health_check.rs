mod helpers;

use helpers::make_clients;
use helpers::ADDR;

#[tokio::test]
async fn test_health_check() {
    let (client, _) = make_clients();

    let response_ok = client
        .get(format!("https://{}/health_check", ADDR))
        .send()
        .await
        .expect("Failed to send request");
    assert!(response_ok.status().is_success());
}
