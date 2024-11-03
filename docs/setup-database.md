# Database Setup
The application supports two database configurations:
- Embedded PostgreSQL (for development)
- External PostgreSQL (recommended for production)

## Option 1: Embedded Database

The embedded database is a PostgreSQL instance that runs within the application. While convenient for development and testing, it is **not recommended for production use**.

### Quick Start
```bash
cargo run --features embedded-database -- --embedded_database=<database_name>
```

> **Note:** When using the embedded database:
> - The `DATABASE_URL` environment variable is ignored
> - Data persists between restarts in the specified database file
> - The migrations are embedded in the application binary and will be run at startup

## Option 2: External Database

For production deployments, using an external PostgreSQL database is recommended.

### Prerequisites
- PostgreSQL server (v16 or higher)
- Rust toolchain with `sqlx` CLI
- `.env` file in project root

### Setup Steps
1. Configure your database connection:
   ```bash
   # In your .env file
   DATABASE_URL="postgresql://user:password@localhost:5432/dbname"
   ```

2. Run database migrations:
   ```bash
   sqlx migrate run
   ```
    > **Note:** It's possible to embed database migrations and run them with the application, but this is not recommended for production.

3. Start the application:
   ```bash
   cargo run
   ```