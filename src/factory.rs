use chrono::Utc;

/// Generate a test email address.
pub fn test_email() -> String {
    format!("test-{}@example.com", &uuid::Uuid::new_v4().to_string()[..8])
}

/// Generate a test username.
pub fn test_username() -> String {
    format!("user-{}", &uuid::Uuid::new_v4().to_string()[..8])
}

/// Generate a test timestamp (recent past).
pub fn recent_timestamp() -> chrono::DateTime<Utc> {
    Utc::now() - chrono::Duration::hours(1)
}

/// Generate a test future timestamp.
pub fn future_timestamp() -> chrono::DateTime<Utc> {
    Utc::now() + chrono::Duration::hours(1)
}
