#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Shared test utilities for Rust projects.
//!
//! Provides test database setup, HTTP test servers, authentication helpers,
//! and assertion utilities for consistent testing across projects.

// This crate *is* test scaffolding: fixtures that panic via expect/unwrap
// on setup failure are the intended contract for test helpers.
#![allow(clippy::expect_used, clippy::unwrap_used)]

/// Custom assertion macros.
pub mod assert;
/// Test authentication helpers.
pub mod auth;
/// Test database setup and management.
pub mod db;
/// Test data factory functions.
pub mod factory;
/// Test HTTP server utilities.
pub mod http;
