use dodo_payments_backend::utils::hash::{hash_password, verify_password};
use dodo_payments_backend::utils::jwt::{create_jwt, verify_jwt};
use uuid::Uuid;

#[test]
fn test_password_hash_and_verify() {
    let password = "SuperSecret123!";
    let hashed = hash_password(password);
    assert!(verify_password(&hashed, password));
}

#[test]
fn test_jwt_creation_and_decoding() {
    dotenvy::dotenv().ok();

    let user_id = Uuid::new_v4().to_string();
    let token = create_jwt(&user_id);
    let decoded = verify_jwt(&token).unwrap();
    assert_eq!(decoded, user_id);
}
