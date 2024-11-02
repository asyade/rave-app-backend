# RUST Backend Template - GraphQL API/PostgreSQL database/JWKS authentication
This repository provide a production ready stateless backend application template written in Rust.
Its mainely design to be the backend of a SaaS or mobile application backend featuring a GraphQL API, a PostgreSQL database (that can be embedded in the application or external) and a JWKS authentication compatible with Auth0 or any other provider supporting JWKS and OpenID Connect.

The goal is to provide a proper starting point for building highly available, secure, and performant backend applications while avoiding common pitfalls.
The code feature clean architecture and is designed in a modular way as well as follow RUST best practices empowering concurrency and performance at runtime but also buildtime featuring fast compilation times and responsive rust analyzer performance.
This repository came with all the containerisation and CI/CD setup to deploy the application in a cloud environment with proper observability.
But even better it came with everythings you need to easyly build proper developpement flows and testing pipelines ! It features embeded database and developpement tools !

## Features overview
- Flexible webserver with axum
- Proper Asynchronous GraphQL API
- Auth0 user management with JWT verification
- PostgreSQL Database management
    - Migrations
    - Usage of Entity pattern
    - Use latest sqlx version with async support
- Tracing & Error handling
    - Use of tracing for flexible and structured logging
    - Use of thiserror for type safe error handling
- CI/CD pipeline
    - Docker image with proper dependencies caching
- Testing & Developpement
    - Feature authentication free mode for development and testing
    - Allow fast developpement setup with embeded database
    - Feature integration tests execution with isolated database and parallel execution
- Multiplatform (Linux/MacOS/Windows)

## How to run the project with minimal setup
In this section we will see how to run the project with a minimal setup, without a persistent database and without Auth0.

### Prerequisites
- Rust nightly toolchain with rustc version >= 1.84
- `sqlx_cli` golbaly installed (i.e `cargo install sqlx-cli`)

### Running the project

