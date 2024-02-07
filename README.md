## Rust Web API boilerplate

This is a simple boilerplate for a web API using Rust.

## How to contribute

If you want to contribute to this project, please follow the next steps:

- Create a new branch from `develop`
- Make your changes
- Create a pull request to `develop`
- Wait for the review

## Commit messages format

- Use the following format for your commit messages:
  - `[<type>] <description>`
  - Where `<type>` can be:
    - `feat`: for new features
    - `fix`: for bug fixes
    - `docs`: for changes in documentation
    - `style`: for changes in code style
    - `refactor`: for code refactoring
    - `test`: for changes in tests
    - `chore`: for changes in the build process or tools
  - And `<description>` is a short description of the changes

### Example

```bash
git commit -m "[feat] Add a new endpoint to get a user by id"
git commit -m "[fix] Fix a bug in the user creation endpoint"
git commit -m "[docs] Add a new section in the README file"
```

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
- Add auto generation of change log based on commit messages
- Add a CI/CD pipeline
- Add data obfuscation to preserve user sensitive data
- Add a simple authentication example using JWT
- Add a simple authorization example using roles
- Add a simple example of how to handle errors
- Add a simple example of how to handle logs
- Add a simple example of how to handle tests
- Add a simple example of how to handle migrations
- Add a simple example of how to handle internationalization
- Add a simple example of how to handle caching
