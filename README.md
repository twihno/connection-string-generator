# Connection string generator

A VERY simple crate to generate database connection strings programmatically.

## Currently supported databases

- PostgreSQL
- Microsoft SQL Server

## Examples

### PostgreSQL

```rust
let conn_string = PostgresConnectionString::new()
    .set_username_and_password("user", "password")
    .set_host_with_port("localhost", 5432)
    .set_database_name("db_name")
    .set_connect_timeout(30);

println!("{conn_string}");
```

### Microsoft SQL Server

```rust
let conn_string = SqlServerConnectionString::new()
    .set_username_and_password("user", "password")
    .set_host_with_default_port("sql.test.com")
    .set_database_name("db_name")
    .set_connect_timeout(30)
    .enable_encryption_and_trust_server_certificate();

println!("{conn_string}");
```
