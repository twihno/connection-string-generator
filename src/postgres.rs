//! Connection string generator for `PostgreSQL`

use std::{collections::HashMap, fmt::Display};

use crate::{HostPort, UsernamePassword};

/// The `userspec` part of the connection string
#[derive(Debug)]
enum UserSpec {
    Username(String),
    UsernamePassword(UsernamePassword),
}

impl Display for UserSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Username(username) => write!(f, "{username}@"),
            Self::UsernamePassword(UsernamePassword { username, password }) => {
                write!(f, "{username}:{password}@")
            }
        }
    }
}

/// The `hostspec` part of the connection string
#[derive(Debug)]
enum HostSpec {
    Host(String),
    HostPort(HostPort),
}

impl Display for HostSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Host(host) => write!(f, "{host}"),
            Self::HostPort(HostPort { host, port }) => write!(f, "{host}:{port}"),
        }
    }
}

/// The `database` part of the connection string
#[derive(Debug)]
struct Database {
    db_name: String,
}

impl Display for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write! {f, "/{}", self.db_name}
    }
}

/// Struct representing a `PostgreSQL` connection string
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct PostgresConnectionString {
    userspec: Option<UserSpec>,
    hostspec: Option<HostSpec>,
    database: Option<Database>,
    parameter_list: HashMap<String, String>,
}

impl Default for PostgresConnectionString {
    #[must_use]
    fn default() -> Self {
        Self::new()
    }
}

impl PostgresConnectionString {
    /// Creates a new and empty [`PostgresConnectionString`]
    ///
    /// This function initializes a new [`PostgresConnectionString`] with empty values.
    /// Without any further changes this results in the string `postgres://` which isn't really useful.
    ///
    /// This function can be chained other functions to fill the missing fields in the connection string.
    ///
    /// # Examples
    /// ```rust
    /// use connection_string_generator::postgres::PostgresConnectionString;
    ///
    /// PostgresConnectionString::new()
    ///   .set_username_and_password("user", "password")
    ///   .set_host_with_port("localhost", 5432)
    ///   .set_database_name("db_name")
    ///   .set_connect_timeout(30);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            userspec: None,
            hostspec: None,
            database: None,
            parameter_list: HashMap::new(),
        }
    }

    /// Replaces the userspec
    #[must_use]
    fn set_userspec(mut self, userspec: UserSpec) -> Self {
        self.userspec = Some(userspec);
        self
    }

    /// Sets/Replaces the username and omits the password in the connection string
    ///
    /// # Examples
    /// ```rust
    /// use connection_string_generator::postgres::PostgresConnectionString;
    ///
    /// PostgresConnectionString::new().set_username_without_password("user");
    /// ```
    #[must_use]
    pub fn set_username_without_password(self, username: &str) -> Self {
        self.set_userspec(UserSpec::Username(simple_percent_encode(username)))
    }

    /// Sets/Replaces the username and the password
    ///
    /// # Examples
    /// ```rust
    /// use connection_string_generator::postgres::PostgresConnectionString;
    ///
    /// PostgresConnectionString::new().set_username_and_password("user", "password");
    /// ```
    #[must_use]
    pub fn set_username_and_password(self, username: &str, password: &str) -> Self {
        self.set_userspec(UserSpec::UsernamePassword(UsernamePassword {
            username: simple_percent_encode(username),
            password: simple_percent_encode(password),
        }))
    }

    /// Replaces the hostspec
    #[must_use]
    fn set_hostspec(mut self, hostspec: HostSpec) -> Self {
        self.hostspec = Some(hostspec);
        self
    }

    /// Sets/Replaces the host and omits the port in the connection string
    /// (this usually results in the usage of the default port)
    ///
    /// # Examples
    /// ```rust
    /// use connection_string_generator::postgres::PostgresConnectionString;
    ///
    /// PostgresConnectionString::new().set_host_with_default_port("localhost");
    /// ```
    #[must_use]
    pub fn set_host_with_default_port(self, host: &str) -> Self {
        self.set_hostspec(HostSpec::Host(simple_percent_encode(host)))
    }

    /// Sets/Replaces the host and the port
    ///
    /// # Examples
    /// ```rust
    /// use connection_string_generator::postgres::PostgresConnectionString;
    ///
    /// PostgresConnectionString::new().set_host_with_port("localhost", 5432);
    /// ```
    #[must_use]
    pub fn set_host_with_port(self, host: &str, port: usize) -> Self {
        self.set_hostspec(HostSpec::HostPort(HostPort {
            host: simple_percent_encode(host),
            port,
        }))
    }

    /// Sets/Replaces the database name
    ///
    /// # Examples
    /// ```rust
    /// use connection_string_generator::postgres::PostgresConnectionString;
    ///
    /// PostgresConnectionString::new().set_database_name("db_name");
    /// ```
    #[must_use]
    pub fn set_database_name(mut self, db_name: &str) -> Self {
        self.database = Some(Database {
            db_name: simple_percent_encode(db_name),
        });
        self
    }

    /// Sets/Replaces the connection timeout in seconds
    ///
    /// # Examples
    /// ```rust
    /// use connection_string_generator::postgres::PostgresConnectionString;
    ///
    /// PostgresConnectionString::new().set_connect_timeout(30);
    /// ```
    #[must_use]
    pub fn set_connect_timeout(mut self, timeout: usize) -> Self {
        self.parameter_list
            .insert(String::from("connect_timeout"), timeout.to_string());
        self
    }

    /// Sets/replaces ANY parameter even if it doesn't exist in the list of allowed/implemented parameters
    ///
    /// # Examples
    /// ```rust
    /// use connection_string_generator::postgres::PostgresConnectionString;
    ///
    /// PostgresConnectionString::new().dangerously_set_parameter("parameter", "value");
    /// ```
    #[must_use]
    pub fn dangerously_set_parameter(mut self, key: &str, value: &str) -> Self {
        self.parameter_list
            .insert(simple_percent_encode(key), simple_percent_encode(value));
        self
    }
}

impl Display for PostgresConnectionString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut conn_string = String::from("postgres://");

        if let Some(userspec) = &self.userspec {
            conn_string.push_str(&userspec.to_string());
        }

        if let Some(hostspec) = &self.hostspec {
            conn_string.push_str(&hostspec.to_string());
        }

        if let Some(database) = &self.database {
            conn_string.push_str(&database.to_string());
        }

        if !self.parameter_list.is_empty() {
            let parameters: Vec<String> = self
                .parameter_list
                .iter()
                .map(|(key, value)| format!("{key}={value}"))
                .collect();

            conn_string.push_str(&format!("?{}", parameters.join("&")));
        }

        write!(f, "{conn_string}")
    }
}

const PERCENT_REPLACEMENTS: [(char, &str); 18] = [
    ('!', "%21"),
    ('#', "%23"),
    ('$', "%24"),
    ('&', "%26"),
    ('\'', "%27"),
    ('(', "%28"),
    (')', "%29"),
    ('*', "%2A"),
    ('+', "%2B"),
    (',', "%2C"),
    ('/', "%2F"),
    (':', "%3A"),
    (';', "%3B"),
    ('=', "%3D"),
    ('?', "%3F"),
    ('@', "%40"),
    ('[', "%5B"),
    (']', "%5D"),
];

/// Replaces reserved characters with their encoded versions
/// (<https://en.wikipedia.org/wiki/Percent-encoding#Reserved_characters>)
fn simple_percent_encode(s: &str) -> String {
    let mut s = s.to_string();

    for replacement in &PERCENT_REPLACEMENTS {
        s = s.replace(replacement.0, replacement.1);
    }

    s
}

#[cfg(test)]
mod test {
    use crate::postgres::simple_percent_encode;
    use crate::postgres::PostgresConnectionString;

    #[test]
    /// Test functionality of [`simple_percent_encode`]
    fn test_simple_percent_encode() {
        assert_eq!(
            simple_percent_encode("!#$&'()*+,/:;=?@[]"),
            "%21%23%24%26%27%28%29%2A%2B%2C%2F%3A%3B%3D%3F%40%5B%5D"
        );
        assert_eq!(simple_percent_encode("test!"), "test%21");
    }

    /// Test empty/default config
    #[test]
    fn test_empty() {
        let conn_string = PostgresConnectionString::new();
        assert_eq!(&conn_string.to_string(), "postgres://");
    }

    /// Test userspec settings
    #[test]
    fn test_userspec() {
        let conn_string = PostgresConnectionString::new();

        let conn_string = conn_string.set_username_without_password("User");
        assert_eq!(&conn_string.to_string(), "postgres://User@");

        let conn_string = conn_string.set_username_and_password("User", "Password");
        assert_eq!(&conn_string.to_string(), "postgres://User:Password@");
    }

    /// Test hostspec settings
    #[test]
    fn test_hostspec() {
        let conn_string = PostgresConnectionString::new();

        let conn_string = conn_string.set_host_with_default_port("Host");
        assert_eq!(&conn_string.to_string(), "postgres://Host");
        let conn_string = conn_string.set_host_with_port("Host", 80);
        assert_eq!(&conn_string.to_string(), "postgres://Host:80");
    }

    /// Test database settings
    #[test]
    fn test_database() {
        let conn_string = PostgresConnectionString::new();

        let conn_string = conn_string.set_database_name("db_name");
        assert_eq!(&conn_string.to_string(), "postgres:///db_name");
    }

    /// Test parameter settings
    #[test]
    fn test_parameters() {
        let conn_string = PostgresConnectionString::new();

        let conn_string = conn_string.set_connect_timeout(30);
        assert_eq!(&conn_string.to_string(), "postgres://?connect_timeout=30");

        let conn_string = conn_string.dangerously_set_parameter("param", "value#");
        let conn_string_as_string = conn_string.to_string();
        // Hashmap order isn't stable but this is irrelevant in the actual use-case
        assert!(
            conn_string_as_string == "postgres://?connect_timeout=30&param=value%23"
                || conn_string_as_string == "postgres://?param=value%23&connect_timeout=30"
        );
    }

    /// Test everything together
    #[test]
    fn test_all_together() {
        let conn_string = PostgresConnectionString::new()
            .set_username_and_password("user", "password")
            .set_host_with_port("localhost", 5432)
            .set_database_name("db_name")
            .set_connect_timeout(30);

        assert_eq!(
            &conn_string.to_string(),
            "postgres://user:password@localhost:5432/db_name?connect_timeout=30"
        );
    }
}
