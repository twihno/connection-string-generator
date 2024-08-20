//! A VERY simple crate to generate database connection strings programmatically.
//!
//! # Currently supported databases
//! - `PostgreSQL`
//! - `Microsoft SQL Server`

#[cfg(feature = "postgres")]
pub mod postgres;

#[cfg(feature = "postgres")]
pub use postgres::PostgresConnectionString;

#[cfg(feature = "sqlserver")]
pub mod sqlserver;

#[cfg(feature = "sqlserver")]
pub use sqlserver::SqlServerConnectionString;

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
