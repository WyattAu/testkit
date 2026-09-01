/// Generate a test JWT token with default claims.
pub fn test_jwt(user_id: &str) -> String {
    use jsonwebtoken::{encode, Header, EncodingKey};
    use serde::{Deserialize, Serialize};
    use chrono::{Utc, Duration};

    #[derive(Debug, Serialize, Deserialize)]
    struct TestClaims {
        sub: String,
        exp: usize,
        iat: usize,
    }

    let claims = TestClaims {
        sub: user_id.to_owned(),
        exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
        iat: Utc::now().timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("test-secret".as_bytes()),
    )
    .expect("failed to encode test JWT")
}

/// Generate a test API key.
pub fn test_api_key() -> String {
    format!("test-{}", uuid::Uuid::new_v4())
}

/// Generate a test user ID.
pub fn test_user_id() -> String {
    uuid::Uuid::new_v4().to_string()
}
