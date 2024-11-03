# Project Structure

## Oerview

![Project Structure](./images/architecture_overview.png)

## Crates

### `crates/rave-api`

The main crate of the application. It is responsible for setting up the webserver and the differents services.

### `crates/rave-api-graphql`

The GraphQL API of the application. It is responsible for handling the GraphQL requests and responses.

### `crates/rave-api-service-database`

The database service of the application. It is responsible for handling the database connections pool.

### `crates/rave-api-service-iam`

The IAM service of the application. It is responsible for handling the authentication and authorization operations.
> Note: This service can easily be rewritten to match specific needs (e.g: Keycloak, ...)

### `crates/rave-api-service-feed-provider`

This service is just a dummy implementation of a feed provider. It is used to demonstrate how to implement a custom service.

### `crates/rave-embedded-database`

The embedded database crate is used to handle the embedded database (PostgreSQL) and its migrations.

### `crates/rave-entity`

The entity crate contains the differents application schemas and entities.
This crate is does not contains any business logic, it only contains the data models.

