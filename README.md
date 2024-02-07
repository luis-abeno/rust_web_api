## Rust Web API boilerplate

This is a simple boilerplate for a web API using Rust.

## How to contribute

If you want to contribute to this project, please follow the next steps:

- Create a new branch from `develop`
- Make your changes
- Create a pull request to `develop`
- Wait for the review

## Dependencies

- [actix-web](https://actix.rs/)
- [serde](https://serde.rs/)
- [utoipa](https://github.com/juhaku/utoipa)
- [utoipa-swagger-ui](https://crates.io/crates/utoipa-swagger-ui)

## Dev Dependencies

- [cargo-watch](https://crates.io/crates/cargo-watch)

### Development

To run the server in development mode, use the following command:

```bash
cargo run
```

or to watch for changes and restart the server automatically:

```bash
cargo watch -x run
```

## TODO

- Add a docker container
- Add a postgres database
- Add a mysql database
- Add a mongodb database
- Add a simple CRUD example
- Add env files to handle some configs
