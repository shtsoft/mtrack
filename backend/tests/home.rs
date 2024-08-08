mod helpers;

use helpers::make_clients;
use helpers::ADDR;

#[tokio::test]
async fn test_home() {
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
