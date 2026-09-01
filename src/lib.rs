#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Shared test utilities for Rust projects.
//!
//! Provides test database setup, HTTP test servers, authentication helpers,
//! and assertion utilities for consistent testing across projects.

/// Test database setup and management.
pub mod db;
/// Test HTTP server utilities.
pub mod http;
/// Test authentication helpers.
pub mod auth;
/// Custom assertion macros.
pub mod assert;
/// Test data factory functions.
pub mod factory;
