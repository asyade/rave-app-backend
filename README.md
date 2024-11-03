# Rust Backend Template: GraphQL + PostgreSQL + JWKS
A production-ready backend template featuring GraphQL API, PostgreSQL database, and JWKS authentication. Built with clean architecture principles and modern Rust practices.

## 🚀 Key Features

### Core Technologies
- **GraphQL API** using async-graphql with playground support
- **PostgreSQL Database** with both embedded (for fast development or testing) and external options
- **JWKS Authentication** compatible with Auth0 and other OpenID Connect providers
- **Axum Web Framework** for flexible and performant HTTP handling

### Developer Experience
- Fast compilation times and responsive IDE support
- Embedded database option for rapid development
- Authentication-free mode for testing
- Comprehensive integration tests with isolated database support
- Cross-platform support (Linux/MacOS/Windows)

### Production Ready
- Docker containerization with optimized caching
- CI/CD pipeline configuration
- Structured logging with `tracing`
- Type-safe error handling using `thiserror`
- Database migrations and entity patterns using `sqlx`

## 🏁 Getting Started

### Prerequisites
- Rust toolchain (latest stable)
- PostgreSQL (optional if using embedded database)
- Auth0 account or other OIDC provider

### Environment Setup
1. Copy `.env.example` to `.env` and update the variables with your own values.
2. Refer to the [Auth0 documentation](./docs/setup-auth0.md) for more information on how to configure the authentication system.
3. Refer to [Database Setup](./docs/setup-database.md) for more information on how to configure the database.

### Running the Application

#### Development Mode (with embedded database)
```bash
cargo run --features embedded-database
```

#### Standard Mode (external database)
```bash
cargo run --database-url "postgresql://user:pass@localhost/dbname"
# or with .env configuration
cargo run
```

### Testing the API
1. Access GraphQL playground: `http://<listen_address>/graphql`
2. Try a sample query:
   ```graphql
   query {
     getCurrentUserFeed(category: HOME, limit: 10) {
       offset
       posts {
         content
       }
     }
   }
   ```

## 🔐 Authentication Setup

### Auth0 Configuration
Refer to the [Auth0 documentation](./docs/setup-auth0.md) for more information on how to configure the authentication system.

### Other OIDC Providers
The authentication system is compatible with any OIDC provider that supports JWKS. Ensure your provider exposes a JWKS endpoint and configure the environment variables accordingly.

## 📱 Demo Frontend
Includes an example frontend application built with Expo framework:
- Mobile and web support
- Auth0 integration
- GraphQL API consumption examples

## 🧪 Testing
```bash
# Run integration tests (uses isolated database)
cargo test --features embedded-database

# Run specific test suite
cargo test user_tests --features embedded-database
```

// ... rest of the existing content if needed ...


# *WIP* RUST Backend template: GraphQL/PostgreSQL/JWKS
This repository provide a starting point for building a backend application using Rust.
It features a GraphQL API, a PostgreSQL database (that can be embedded in the application or external) and a JWKS authentication compatible with Auth0 or any other provider supporting JWKS and OpenID Connect.

The goal is to provide a proper starting point for building highly available, secure, and performant backend applications while avoiding common pitfalls.
The code feature clean architecture and is designed in a modular way as well as follow RUST best practices empowering concurrency and performance at runtime but also buildtime featuring fast compilation times and responsive rust analyzer performance.
This repository came with all the containerisation and CI/CD setup to deploy the application in a cloud environment with proper observability.
But even better it came with everythings you need to easyly build proper developpement flows and testing pipelines ! It features embeded database and developpement tools !

## Features overview
- Flexible webserver with axum
- Proper Asynchronous GraphQL API
- OpenID Connect authentication using JWKS (compatible with Auth0)
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

## Demo frontend
This template came with an fully featured example of frontend application that uses the `expo` framework to build a mobile/web application that use the  API and perform authentication using Auth0.

## How to prepare and run the project
There is various way to run the project depending on your needs.
For development, you can use the `embedded-database` feature to have a fast setup with an embeded database.
For production or regular development, you will want to use an externaly installed database.

### Prerequisites
- Latest stable Rust toolchain

### OIDC Provider
You need to setup an OIDC provider that support JWKS and OpenID Connect in order to use the authentication feature.
To do so we will use Auth0 as an example but you can use any other OpenID Connect provider that support JWKS and OpenID Connect.


### Preparing your environment
You can use the `.env.example` as a template to create your own `.env` file with the correct environment variables.

### Running the project
#### Development
To run the project in development mode, you can use the following command:

```bash
cargo run --features embedded-database
```

or if you want to run the project without the embedded database:
> You also can use the `DATABASE_URL` environment variable to set the database url.
```bash
cargo run --database-url <your-database-url>
```

```bash
cargo run
```

Once the application is running, you can test the API using your browser and directly access the GraphQL playground at `http://<listen_address>/graphql`

Once whithin the playground you can test the API with the following query:

```graphql
query {
  getCurrentUserFeed(category: HOME, limit: 0) {
    offset
    posts {
      content
    }
  }
}
```

