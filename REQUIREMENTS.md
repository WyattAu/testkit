# Requirements — testkit

Numbered, testable requirements. Every requirement maps to at least one named
test. Threat IDs reference `THREAT-MODEL.md`.

Scope note: `testkit` is a **fixture crate** — shared test utilities:
ephemeral SQLite `TestDb`, axum-based `TestServer`, deterministic auth
fixtures (`test_jwt`, `test_api_key`), identity generators (`test_email`,
`test_user_id`, `test_username`), timestamp helpers, and assertion
macros. Not for production use (T1).

## Functional

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-TK-001 | `TestDb::new` creates an isolated, migrated SQLite database (unique path, schema initialized via `init_schema`) | MUST |
| REQ-TK-002 | `TestServer::new` boots an axum test server on an ephemeral port; `addr()`/`path()`/`base_url()` expose the coordinates | MUST |
| REQ-TK-003 | `test_jwt` issues signed fixture tokens; `test_api_key` yields a stable API-key string | MUST |
| REQ-TK-004 | `test_email`/`test_user_id`/`test_username` produce structured, collision-resistant identity fixtures | SHOULD |
| REQ-TK-005 | `recent_timestamp`/`future_timestamp` produce `chrono` timestamps usable to pin expiry logic | SHOULD |
| REQ-TK-006 | Assertion macros compile and fail correctly (`test_assert_macros`) | SHOULD |
| REQ-TK-007 | `TestDb::connection_string` exposes a connectable string for `sqlx`-style callers | SHOULD |

## Security

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-TK-100 | Fixture secrets are deterministic and documented as test-only; the crate-level lint allow marks the whole crate as fixture-grade (T1) | MUST |
| REQ-TK-101 | Databases and servers are test-scoped: unique temp paths and ephemeral ports prevent cross-test and cross-process interference (T2) | MUST |

## Robustness

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-TK-200 | Identity generators never collide across repeated invocations within a test run | SHOULD |

## Traceability Matrix

| Requirement | Test (fn, file) | Property class |
|-------------|-----------------|----------------|
| REQ-TK-001 | `test_db_creation` (`src/` tests, `tests/integration.rs`) | unit/integration |
| REQ-TK-002 | `TestServer` accessors (`addr`, `path`, `base_url`) exercised in integration suite | integration |
| REQ-TK-003 | `test_jwt`/`test_api_key` paths, `test_api_key_generation` | unit |
| REQ-TK-004 | `test_email_generation`, `test_user_id_generation`, `test_username` | unit |
| REQ-TK-005 | `recent_timestamp`, `future_timestamp` helper units | unit |
| REQ-TK-006 | `test_assert_macros` | unit |
| REQ-TK-100 | Design review + README/`Cargo.toml` placement as dev-dependency material | design |
| REQ-TK-101 | `test_db_creation` (unique temp dir); `TestServer` ephemeral-port binding | unit/integration |
| REQ-TK-200 | `test_email_generation`, `test_user_id_generation` | unit |

## Test Count

- 5 `#[test]` functions across unit and integration suites.
- All-features suite passes with 0 failures; no-default-features suite passes.
