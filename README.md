# Connection string generator

A VERY simple crate to generate database connection strings programmatically.

## Currently supported databases

- PostgreSQL

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
