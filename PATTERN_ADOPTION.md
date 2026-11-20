# Pattern adoption: testkit

Shared Rust test utilities. Declared per
`pattern-standards/references/adoption-format.md`.

| Pattern | Status | Evidence |
|---|---|---|
| P68 | adopted | src/factory.rs:4 (test_email, test_username, recent/future timestamp builders; unique values per call instead of shared fixtures) |
| P50 | adopted | src/db.rs:5 (TestDb provisions a real per-test SQLite database) and src/http.rs:1 (real local HTTP test server instead of transport mocks) |
