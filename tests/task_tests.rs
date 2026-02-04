mod common;

use common::{cleanup, create_test_user, setup};
use reqwest::StatusCode;
use serde_json::json;

#[tokio::test]
async fn test_register_user_success() {
    let pool = setup().await;

    let client = reqwest::Client::new();
    let base_url = "http://localhost:8080";

    let payload = json!({
        "email": "newuser@example.com",
        "password": "secure_password_123",
        "name": "New User"
    });

    let response = client
        .post(&format!("{}/api/auth/register", base_url))
        .json(&payload)
        .send()
        .await
        .expect("Falha ao enviar requisição");

    assert_eq!(response.status(), StatusCode::CREATED);

    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body["token"].is_string());
    assert_eq!(body["user"]["email"], "newuser@example.com");

    cleanup(&pool).await;
}

#[tokio::test]
async fn test_register_duplicate_email() {
    let pool = setup().await;
    let (_, email) = create_test_user(&pool).await;

    let client = reqwest::Client::new();
    let base_url = "http://localhost:8080";

    let payload = json!({
        "email": email,
        "password": "password123",
        "name": "Duplicate User"
    });

    let response = client
        .post(&format!("{}/api/auth/register", base_url))
        .json(&payload)
        .send()
        .await
        .expect("Falha ao enviar requisição");

    assert_eq!(response.status(), StatusCode::CONFLICT);

    cleanup(&pool).await;
}

#[tokio::test]
async fn test_login_success() {
    let pool = setup().await;
    let (_, email) = create_test_user(&pool).await;

    let client = reqwest::Client::new();
    let base_url = "http://localhost:8080";

    let payload = json!({
        "email": email,
        "password": "test_password_123"
    });

    let response = client
        .post(&format!("{}/api/auth/login", base_url))
        .json(&payload)
        .send()
        .await
        .expect("Falha ao enviar requisição");

    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body["token"].is_string());
    assert_eq!(body["user"]["email"], email);

    cleanup(&pool).await;
}

#[tokio::test]
async fn test_login_invalid_credentials() {
    let pool = setup().await;
    let (_, email) = create_test_user(&pool).await;

    let client = reqwest::Client::new();
    let base_url = "http://localhost:8080";

    let payload = json!({
        "email": email,
        "password": "wrong_password"
    });

    let response = client
        .post(&format!("{}/api/auth/login", base_url))
        .json(&payload)
        .send()
        .await
        .expect("Falha ao enviar requisição");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    cleanup(&pool).await;
}

#[tokio::test]
async fn test_login_nonexistent_user() {
    let pool = setup().await;

    let client = reqwest::Client::new();
    let base_url = "http://localhost:8080";

    let payload = json!({
        "email": "nonexistent@example.com",
        "password": "password123"
    });

    let response = client
        .post(&format!("{}/api/auth/login", base_url))
        .json(&payload)
        .send()
        .await
        .expect("Falha ao enviar requisição");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    cleanup(&pool).await;
}
