use reqwest::Client;
use serde_json::json;
use std::env;

#[tokio::test]
async fn test_register_and_login() {
    dotenvy::dotenv().ok();
    let base_url = env::var("TEST_API_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let client = Client::new();

    println!("Using base_url: {}", base_url);

    // 1. Register
    let register_resp = client
        .post(format!("{}/register", base_url))
        .json(&json!({
            "email": "testuser3@example.com",
            "password": "StrongPass123"
        }))
        .send()
        .await
        .expect("Register request failed");
    assert_eq!(register_resp.status().as_u16(), 200);

    // 2. Login
    let login_resp = client
        .post(format!("{}/login", base_url))
        .json(&json!({
            "email": "testuser3@example.com",
            "password": "StrongPass123"
        }))
        .send()
        .await
        .expect("Login request failed");

    assert_eq!(login_resp.status().as_u16(), 200);

    let body: serde_json::Value = login_resp.json().await.unwrap();
    let token = body["token"].as_str().unwrap();

    // 3. Create transaction
    let txn_resp = client
        .post(format!("{}/transactions", base_url))
        .bearer_auth(token)
        .json(&json!({
            "amount": 150,
            "description": "Test payment"
        }))
        .send()
        .await
        .expect("Transaction request failed");

    assert_eq!(txn_resp.status().as_u16(), 200);

    // 4. Check balance
    let balance_resp = client
        .get(format!("{}/balance", base_url))
        .bearer_auth(token)
        .send()
        .await
        .expect("Balance request failed");

    assert_eq!(balance_resp.status().as_u16(), 200);
    let balance_json: serde_json::Value = balance_resp.json().await.unwrap();
    assert_eq!(balance_json["balance"].as_str().unwrap(), "150");
}
