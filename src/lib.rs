//! A VERY simple crate to generate database connection strings programmatically.
//!
//! # Currently supported databases
//! - PostgreSQL

#[cfg(feature = "postgres")]
pub mod postgres;

/// Username & password bundled as struct
#[derive(Debug)]
pub struct UsernamePassword {
    username: String,
    password: String,
}

/// host & port bundled as struct
#[derive(Debug)]
pub struct HostPort {
    host: String,
    port: usize,
}
