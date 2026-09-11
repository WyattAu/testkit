# Threat Model — testkit

Status: **v1.0** · Method: STRIDE over the public API surface
(`TestDb`, `TestServer`, JWT/timestamp/API-key helpers, assertion
macros).

Trust boundaries: (1) the developer's test process (fully trusted —
this crate *is* test infrastructure), (2) the machine's filesystem and
loopback network the fixtures bind, (3) the boundary between test
fixtures and production code paths that must never absorb
test-only assumptions.

The crate is explicitly a **fixture crate**: it trades hardening for
ergonomics under `cargo test`. Its cardinal threat is *fixture
behavior leaking into production assumptions* — e.g. deterministic
test tokens being mistaken for secure token generation.

## Assets

| ID | Asset | Example |
|----|-------|---------|
| A1 | Production security posture | A "test JWT" helper being copied into production auth code |
| A2 | Test isolation | Two tests sharing a SQLite file or server port, flaking or corrupting each other |
| A3 | Deterministic fixtures | Random emails/user IDs colliding across assertions |
| A4 | Developer machines | Test servers binding privileged/wildcard addresses |

## STRIDE Analysis

| # | Threat | Category | Surface | Mitigation | Verifying test |
|---|--------|----------|---------|------------|----------------|
| T1 | Test-only security shortcuts migrate to production | Elevation | `test_jwt`, `test_api_key` | Helpers are documented fixtures with hard-coded/deterministic secrets by design; the crate is dev-dependency material and the README/API docs say so. The crate-level `unwrap`/`expect` allow marks it as non-production code | Design review; `test_jwt`/`test_api_key` unit tests pin deterministic output |
| T2 | Fixture collisions between tests | DoS (flaky tests) | `TestDb::new`, `TestServer::new` | Databases are unique temp dirs (`tempfile`), servers bind an OS-assigned ephemeral port and expose `addr()`/`base_url()` | `test_db_creation`, `TestServer::addr`/`base_url` accessors |
| T3 | Non-deterministic identity fixtures | Tampering (of test expectations) | `test_email`, `test_user_id`, `test_username` | Generators produce unique-but-structured values verified by unit tests | `test_email_generation`, `test_user_id_generation`, `test_username` |
| T4 | Auth helper producing unacceptable tokens | Spoofing | `test_jwt` | Tokens are signed with a fixture key for exercising auth *paths*; production verification still rejects unknown signers — fixture tokens are not security tokens | `test_jwt` usage across assertion tests; `test_assert_macros` |
| T5 | Time-dependent logic under test mis-evaluated | Tampering | `recent_timestamp`, `future_timestamp` | Explicit recent/future constructors let tests pin expiry logic deterministically | timestamp helper unit tests |

## OPEN RISKS (missing mitigations — not fabricated)

- **OPEN-1 — deterministic secrets are a feature, not a bug, but a
  foot-gun.** `test_jwt`/`test_api_key` values must never appear in
  production configs; code review must reject such usage.
- **OPEN-2 — crate-level lint allows.** `unwrap`/`expect` are allowed
  crate-wide for fixture ergonomics. Any non-test reuse of this crate
  inherits those relaxed lints.

## Out of Scope

- Performance-realistic database or HTTP behavior (fixtures favor
  startup speed over fidelity).
- Container orchestration beyond the bundled rusqlite/axum fixtures.
- Production-grade logging, metrics, or error handling within fixtures.

## Residual Risks

- A `TestServer` bound to loopback is reachable by any local process —
  irrelevant for CI, relevant on shared dev hosts.
