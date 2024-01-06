### System prereuistites
- openssl

### Database setup (initial)
1. Set your `DATABASE_URL` environement variable to a valide postgresql database address, ex: `postgres://postgres:postgres@localhost/postgres`
2. Create the database using sqlx-cli
    ```bash
    sqlx database create
    ```
2. Run migrations
    ```bash
    sqlx database create
    ```