# testkit

Shared test utilities for Rust — test databases, HTTP servers, auth helpers, and assertion macros.

## Features

- **TestDb** — Temporary SQLite databases with RAII cleanup
- **TestServer** — Axum-based HTTP test servers on random ports
- **Auth helpers** — JWT tokens, API keys, and user ID generation
- **Assert macros** — `assert_contains!`, `assert_error!`, `assert_ok_eq!`
- **Factory functions** — Test emails, usernames, and timestamps

## Installation

```bash
cargo add testkit
```

## Quick Start

```rust
use testkit::{TestDb, TestServer, test_jwt, assert_contains};
use axum::Router;

#[tokio::main]
async fn main() {
    // Temporary database
    let db = TestDb::new("my_test");
    db.init_schema("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)").unwrap();

    // Test HTTP server
    let app = Router::new().route("/health", axum::routing::get(|| async { "ok" }));
    let server = TestServer::new(app).await;
    let resp = reqwest::get(format!("{}/health", server.base_url())).await.unwrap();

    // Auth helpers
    let token = test_jwt("user-123");

    // Assert macros
    assert_contains!("hello world", "world");
}
```

## Modules

### `db` — Test Database

```rust
use testkit::db::TestDb;

let db = TestDb::new("test");
db.init_schema("CREATE TABLE items (id INTEGER PRIMARY KEY)").unwrap();
// Database is automatically cleaned up on drop
```

### `http` — Test Server

```rust
use testkit::http::TestServer;

let server = TestServer::new(app).await;
server.base_url(); // http://127.0.0.1:PORT
server.stop().await;
```

### `auth` — Authentication

```rust
use testkit::auth::{test_jwt, test_api_key, test_user_id};

let token = test_jwt("user-123");       // JWT with 1hr expiry
let key = test_api_key();               // "test-<uuid>"
let uid = test_user_id();               // "<uuid>"
```

### `assert` — Assertion Macros

```rust
use testkit::{assert_contains, assert_error, assert_ok_eq};

assert_contains!("hello world", "world");
assert_error!(Err::<(), _>("something"));
assert_ok_eq!(Ok(42), 42);
```

### `factory` — Test Data Factories

```rust
use testkit::factory::{test_email, test_username, recent_timestamp};

let email = test_email();       // "test-<uuid>@example.com"
let name = test_username();     // "user-<uuid>"
let ts = recent_timestamp();    // 1 hour ago
```

## License

MIT OR Apache-2.0
