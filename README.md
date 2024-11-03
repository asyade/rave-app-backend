# Rust Backend Template: GraphQL + PostgreSQL + OpenID Connect
A production-ready backend template featuring GraphQL API, PostgreSQL database, and JWKS authentication. Built with clean architecture principles and modern Rust practices.

## 🚀 Key Features

### Frontend template for Android/iOS applications
- [Rave Mobile App Template](https://github.com/asyade/rave-app-frontend)

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

## 🧪 Testing
```bash
# Run integration tests (uses isolated database)
cargo test --features embedded-database

# Run specific test suite
cargo test user_tests --features embedded-database
```

## Project Structure
Refer to the [Project Structure](./docs/project-structure.md) for more information on the project structure.

![Auth0 API Identifier](./docs/images/architecture_overview.png)
