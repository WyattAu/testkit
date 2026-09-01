use testkit::db::TestDb;
use testkit::{auth, factory};
use testkit::{assert_contains, assert_error, assert_ok_eq};

#[test]
fn test_db_creation() {
    let db = TestDb::new("test");
    assert!(db.path().exists() || db.path().parent().unwrap().exists());
}

#[test]
fn test_api_key_generation() {
    let key1 = auth::test_api_key();
    let key2 = auth::test_api_key();
    assert_ne!(key1, key2);
    assert!(key1.starts_with("test-"));
}

#[test]
fn test_user_id_generation() {
    let id = auth::test_user_id();
    assert!(!id.is_empty());
}

#[test]
fn test_email_generation() {
    let email = factory::test_email();
    assert!(email.contains("@example.com"));
}

#[test]
fn test_assert_macros() {
    assert_contains!("hello world", "world");
    assert_error!(Err::<String, _>("error"));
    assert_ok_eq!(Ok::<String, String>("hello".to_string()), "hello".to_string());
}
