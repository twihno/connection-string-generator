//! Connection string generator for `Microsoft SQL Server`

use std::{
    cmp::{max, min},
    collections::HashMap,
    fmt::Display,
};

/// Struct representing a `Microsoft SQL Server` connection string
///
/// All parameter values will be automatically escaped to match the required format
#[derive(Debug)]
pub struct SqlServerConnectionString {
    parameter_list: HashMap<String, String>,
}

impl Default for SqlServerConnectionString {
    #[must_use]
    fn default() -> Self {
        Self::new()
    }
}

impl SqlServerConnectionString {
    /// Creates a new and empty [`SqlServerConnectionString`]
    ///
    /// This function initializes a new [`SqlServerConnectionString`] with empty values.
    /// Without any further changes this results in an empty string which isn't really useful.
    ///
    /// This function can be chained other functions to fill the missing fields in the connection string.
    ///
    /// # Examples
    /// ```rust
    /// use connection_string_generator::sqlserver::SqlServerConnectionString;
    ///
    /// SqlServerConnectionString::new()
    ///   .set_username_and_password("user", "password")
    ///   .set_host_with_port("localhost", 5432)
    ///   .set_database_name("db_name")
    ///   .set_connect_timeout(30)
    ///   .enable_encryption_and_trust_server_certificate();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        SqlServerConnectionString {
            parameter_list: HashMap::new(),
        }
    }

    /// Sets/replaces ANY parameter even if it doesn't exist in the list of allowed/implemented parameters.
    ///
    /// Automatically escapes all values to match the format required by SQL server
    ///
    /// # Examples
    /// ```rust
    /// use connection_string_generator::sqlserver::SqlServerConnectionString;
    ///
    /// SqlServerConnectionString::new().dangerously_set_parameter("parameter", "value");
    /// ```
    #[must_use]
    pub fn dangerously_set_parameter(mut self, key: &str, value: &str) -> Self {
        self.parameter_list
            .insert(key.to_string(), simple_encode(value));
        self
    }

    /// Sets/Replaces the username and removes the password parameter (if it has been previously set)
    ///
    /// Parameters: `user=<username>`
    ///
    /// # Examples
    /// ```rust
    /// use connection_string_generator::sqlserver::SqlServerConnectionString;
    ///
    /// SqlServerConnectionString::new().set_username_without_password("user");
    /// ```
    #[must_use]
    pub fn set_username_without_password(self, username: &str) -> Self {
        let mut connection_string = self.dangerously_set_parameter("user", username);

        // Remove password parameter if it previously has been set
        connection_string.parameter_list.remove("password");

        connection_string
    }

    /// Sets/Replaces the username and the password
    ///
    /// Parameters: `user=<username>;password=<password>`
    ///
    /// # Examples
    /// ```rust
    /// use connection_string_generator::sqlserver::SqlServerConnectionString;
    ///
    /// SqlServerConnectionString::new().set_username_and_password("user", "password");
    /// ```
    #[must_use]
    pub fn set_username_and_password(self, username: &str, password: &str) -> Self {
        self.dangerously_set_parameter("user", username)
            .dangerously_set_parameter("password", password)
    }

    /// Sets/Replaces the host and omits the port in the connection string
    /// (this usually results in the usage of the default port)
    ///
    /// Parameters: `server=<host>`
    ///
    /// # Examples
    /// ```rust
    /// use connection_string_generator::sqlserver::SqlServerConnectionString;
    ///
    /// SqlServerConnectionString::new().set_host_with_default_port("localhost");
    /// ```
    #[must_use]
    pub fn set_host_with_default_port(self, host: &str) -> Self {
        self.dangerously_set_parameter("server", host)
    }

    /// Sets/Replaces the host and the port
    ///
    /// Parameters: `server=<host>,<port>`
    ///
    /// # Examples
    /// ```rust
    /// use connection_string_generator::sqlserver::SqlServerConnectionString;
    ///
    /// SqlServerConnectionString::new().set_host_with_port("localhost", 5432);
    /// ```
    #[must_use]
    pub fn set_host_with_port(self, host: &str, port: usize) -> Self {
        self.dangerously_set_parameter("server", &format!("{host},{port}"))
    }

    /// Enables encryption
    ///
    /// Parameters: `encrypt=true`
    ///
    /// # Examples
    /// ```rust
    /// use connection_string_generator::sqlserver::SqlServerConnectionString;
    ///
    /// SqlServerConnectionString::new().enable_encryption();
    /// ```
    #[must_use]
    pub fn enable_encryption(self) -> Self {
        self.dangerously_set_parameter("encrypt", "true")
    }

    /// Enables encryption and trusts the server certificate
    /// (**even if it isn't normally trusted(!)** (e.g. self-signed, untrusted root CA, ...))
    ///
    /// Parameters: `encrypt=true;trustServerCertificate=true`
    ///
    /// # Examples
    /// ```rust
    /// use connection_string_generator::sqlserver::SqlServerConnectionString;
    ///
    /// SqlServerConnectionString::new().enable_encryption_and_trust_server_certificate();
    /// ```
    #[must_use]
    pub fn enable_encryption_and_trust_server_certificate(self) -> Self {
        self.enable_encryption()
            .dangerously_set_parameter("trustServerCertificate", "true")
    }

    /// Sets/Replaces the database name
    ///
    /// Parameters: `database=<db_name>`
    ///
    /// # Examples
    /// ```rust
    /// use connection_string_generator::sqlserver::SqlServerConnectionString;
    ///
    /// SqlServerConnectionString::new().set_database_name("db_name");
    /// ```
    #[must_use]
    pub fn set_database_name(self, db_name: &str) -> Self {
        self.dangerously_set_parameter("database", db_name)
    }

    /// Sets/Replaces the connect timeout (in seconds)
    ///
    /// If the provided value is negative, the action will be ignored
    ///
    /// Parameters: `timeout=<connect_timeout>`
    ///
    /// # Examples
    /// ```rust
    /// use connection_string_generator::sqlserver::SqlServerConnectionString;
    ///
    /// SqlServerConnectionString::new().set_connect_timeout(30);
    /// ```
    #[must_use]
    pub fn set_connect_timeout(self, connect_timeout: i32) -> Self {
        if connect_timeout < 0 {
            return self;
        }

        self.dangerously_set_parameter("timeout", &connect_timeout.to_string())
    }

    /// Sets/Replaces the command timeout (in seconds)
    ///
    /// If the provided value is negative, the action will be ignored
    ///
    /// Parameters: `Command Timeout=<command_timeout>`
    ///
    /// # Examples
    /// ```rust
    /// use connection_string_generator::sqlserver::SqlServerConnectionString;
    ///
    /// SqlServerConnectionString::new().set_command_timeout(30);
    /// ```
    #[must_use]
    pub fn set_command_timeout(self, command_timeout: i32) -> Self {
        if command_timeout < 0 {
            return self;
        }
        self.dangerously_set_parameter("command timeout", &command_timeout.to_string())
    }

    /// Sets/Replaces the connection retry count
    ///
    /// Parameters: `ConnectRetryCount=<connect_retry_count>`
    ///
    /// # Examples
    /// ```rust
    /// use connection_string_generator::sqlserver::SqlServerConnectionString;
    ///
    /// SqlServerConnectionString::new().set_connect_retry_count(30);
    /// ```
    #[must_use]
    pub fn set_connect_retry_count(self, connect_retry_count: u8) -> Self {
        self.dangerously_set_parameter("connectRetryCount", &connect_retry_count.to_string())
    }

    /// Sets/Replaces the connection retry interval (in seconds)
    ///
    /// Allowed values: 1..=60 . The value will be increased/decreased to fit this range
    ///
    /// Parameters: `ConnectRetryInterval=<connect_retry_interval>`
    ///
    /// # Examples
    /// ```rust
    /// use connection_string_generator::sqlserver::SqlServerConnectionString;
    ///
    /// SqlServerConnectionString::new().set_connect_retry_interval(30);
    /// ```
    #[must_use]
    pub fn set_connect_retry_interval(self, connect_retry_interval: u8) -> Self {
        // Clip to range 1..=60
        let connect_retry_interval = min(max(1, connect_retry_interval), 60);

        self.dangerously_set_parameter("connectRetryInterval", &connect_retry_interval.to_string())
    }
}

impl Display for SqlServerConnectionString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let conn_string = self
            .parameter_list
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<String>>()
            .join(";");

        write!(f, "{conn_string}")
    }
}

/// Simple encoding for values in a SQL server connection string
///
/// According to [Microsoft](https://learn.microsoft.com/en-us/sql/connect/ado-net/connection-strings?view=sql-server-ver16)
/// (Accessed: 2024-08-20):
/// > If a value contains the semicolon, Unicode control characters,
/// > or leading or trailing white space, it must be enclosed in single or double quotation marks
///
/// > The enclosing character may not occur within the value it encloses.
/// > Therefore, a value containing single quotation marks can be enclosed only in double quotation marks, and vice versa
///
/// > You can also escape the enclosing character by using two of them together
///
/// This function checks if quotation marks are needed and only adds them if they are required.
///
/// Double quotation marks are preferred:
///   - If the string only contains single or double quotation marks, the other type will be used for enclosing the string
///   - If both types are present, the double quotation marks will be escaped (replaced by `""`)
///     and double quotation marks will be used to enclose the string
fn simple_encode(s: &str) -> String {
    let quotes_needed =
        str_includes_control_char(s) || s.starts_with(' ') || s.ends_with(' ') || s.contains(';');

    if !quotes_needed {
        return s.to_string();
    }

    let includes_double_quotation = s.contains('"');
    let includes_single_quotation = s.contains('\'');

    if !includes_double_quotation {
        return format!("\"{s}\"");
    }

    if !includes_single_quotation {
        return format!("'{s}'");
    }

    let s = s.replace('"', "\"\"");

    format!("\"{s}\"")
}

/// Checks if the given &str contains a control character by using [`char::is_control`]
fn str_includes_control_char(s: &str) -> bool {
    s.chars().any(char::is_control)
}

#[cfg(test)]
mod test {
    use crate::sqlserver::simple_encode;

    use super::SqlServerConnectionString;

    /// Test functionality of [`simple_encode`]
    #[test]
    fn test_simple_encode() {
        // No changes
        assert_eq!(&simple_encode("a"), "a");
        assert_eq!(&simple_encode("a a"), "a a");
        assert_eq!(&simple_encode("a \"a"), "a \"a");
        assert_eq!(&simple_encode("a' a"), "a' a");
        assert_eq!(&simple_encode("a' \"a"), "a' \"a");

        // Leading/trailing spaces
        assert_eq!(&simple_encode(" a"), "\" a\"");
        assert_eq!(&simple_encode("a "), "\"a \"");
        assert_eq!(&simple_encode(" a "), "\" a \"");
        assert_eq!(&simple_encode("🥙"), "🥙");
        assert_eq!(&simple_encode("🥙 "), "\"🥙 \"");

        // Semicolon
        assert_eq!(&simple_encode("a;a"), "\"a;a\"");
        assert_eq!(&simple_encode(" a;a"), "\" a;a\"");
        assert_eq!(&simple_encode("a;a "), "\"a;a \"");
        assert_eq!(&simple_encode(" a;a "), "\" a;a \"");

        // Control characters
        assert_eq!(&simple_encode("\0"), "\"\0\"");
        assert_eq!(&simple_encode("a\0a"), "\"a\0a\"");

        // Includes single quotation mark
        assert_eq!(&simple_encode(" a'a"), "\" a'a\"");

        // Includes double quotation mark
        assert_eq!(&simple_encode(" a\"a"), "' a\"a'");

        // Includes both quotation marks
        assert_eq!(&simple_encode(" 'a\"a"), "\" 'a\"\"a\"");
        assert_eq!(&simple_encode(" 'a\"\"a"), "\" 'a\"\"\"\"a\"");
    }

    /// Test empty/default config
    #[test]
    fn test_empty() {
        let conn_string = SqlServerConnectionString::new();
        assert_eq!(&conn_string.to_string(), "");
    }

    /// Test functionality of [`SqlServerConnectionString::dangerously_set_parameter`]
    #[test]
    fn test_dangerously_set_parameter() {
        let conn_string = SqlServerConnectionString::new();

        let conn_string = conn_string.dangerously_set_parameter("Key", "Value");
        assert_eq!(&conn_string.to_string(), "Key=Value");

        let conn_string = conn_string.dangerously_set_parameter("Key", " Value");
        assert_eq!(&conn_string.to_string(), "Key=\" Value\"");
    }

    /// Test setting username (and password)
    #[test]
    fn test_set_username() {
        let conn_string = SqlServerConnectionString::new();

        // Set username
        let conn_string = conn_string.set_username_without_password("User");
        assert_eq!(&conn_string.to_string(), "user=User");

        // Set username and password
        let conn_string = conn_string.set_username_and_password("User1", "Pwd");
        let conn_string_as_string = conn_string.to_string();
        assert!(
            &conn_string_as_string == "user=User1;password=Pwd"
                || &conn_string_as_string == "password=Pwd;user=User1"
        );

        // Replace username and implicitly delete password
        let conn_string = conn_string.set_username_without_password("User2");
        assert_eq!(&conn_string.to_string(), "user=User2");
    }

    /// Test setting host config (host, host&port)
    #[test]
    fn test_set_host() {
        let conn_string = SqlServerConnectionString::new();

        let conn_string = conn_string.set_host_with_default_port("Host");
        assert_eq!(&conn_string.to_string(), "server=Host");

        let conn_string = conn_string.set_host_with_port("Host1", 80);
        assert_eq!(&conn_string.to_string(), "server=Host1,80");

        let conn_string = conn_string.set_host_with_default_port("Host2");
        assert_eq!(&conn_string.to_string(), "server=Host2");
    }

    /// Test enabling encryption
    #[test]
    fn test_enable_encryption() {
        let conn_string = SqlServerConnectionString::new().enable_encryption();

        assert_eq!(&conn_string.to_string(), "encrypt=true");
    }

    /// Test enabling encryption and trusting server certificate
    #[test]
    fn test_enable_encryption_and_trust_server_certificate() {
        let conn_string =
            SqlServerConnectionString::new().enable_encryption_and_trust_server_certificate();

        let conn_string_as_string = conn_string.to_string();
        assert!(
            &conn_string_as_string == "encrypt=true;trustServerCertificate=true"
                || &conn_string_as_string == "trustServerCertificate=true;encrypt=true"
        );
    }

    /// Test database name
    #[test]
    fn test_set_database_name() {
        let conn_string = SqlServerConnectionString::new().set_database_name("DbName");

        assert_eq!(&conn_string.to_string(), "database=DbName");
    }

    /// Test connect timeout
    #[test]
    fn test_set_connect_timeout() {
        let conn_string = SqlServerConnectionString::new();

        // Negative value => ignored
        let conn_string = conn_string.set_connect_timeout(-2);
        assert_eq!(&conn_string.to_string(), "");

        // Normal value
        let conn_string = conn_string.set_connect_timeout(30);
        assert_eq!(&conn_string.to_string(), "timeout=30");

        // Negative value => ignored
        let conn_string = conn_string.set_connect_timeout(-2);
        assert_eq!(&conn_string.to_string(), "timeout=30");
    }

    /// Test command timeout
    #[test]
    fn test_command_timeout() {
        let conn_string = SqlServerConnectionString::new();

        // Negative value => ignored
        let conn_string = conn_string.set_command_timeout(-2);
        assert_eq!(&conn_string.to_string(), "");

        // Normal value
        let conn_string = conn_string.set_command_timeout(30);
        assert_eq!(&conn_string.to_string(), "command timeout=30");

        // Negative value => ignored
        let conn_string = conn_string.set_command_timeout(-2);
        assert_eq!(&conn_string.to_string(), "command timeout=30");
    }

    /// Test connect retry count
    #[test]
    fn test_set_connect_retry_count() {
        let conn_string = SqlServerConnectionString::new();

        let conn_string = conn_string.set_connect_retry_count(0);
        assert_eq!(&conn_string.to_string(), "connectRetryCount=0");

        let conn_string = conn_string.set_connect_retry_count(255);
        assert_eq!(&conn_string.to_string(), "connectRetryCount=255");
    }

    /// Test connect retry interval
    #[test]
    fn test_set_connect_retry_interval() {
        let conn_string = SqlServerConnectionString::new();

        // <= 0 => replaced by 1 (min value)
        let conn_string = conn_string.set_connect_retry_interval(0);
        assert_eq!(&conn_string.to_string(), "connectRetryInterval=1");

        // Normal values
        let conn_string = conn_string.set_connect_retry_interval(1);
        assert_eq!(&conn_string.to_string(), "connectRetryInterval=1");

        let conn_string = conn_string.set_connect_retry_interval(60);
        assert_eq!(&conn_string.to_string(), "connectRetryInterval=60");

        // > 60 => replaced by 60 (max value)
        let conn_string = conn_string.set_connect_retry_interval(61);
        assert_eq!(&conn_string.to_string(), "connectRetryInterval=60");
    }
}
