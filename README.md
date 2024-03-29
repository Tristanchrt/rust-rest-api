# Rust Axum REST API with PostgreSQL and Diesel in Docker

This is a basic example of a Rust project that demonstrates how to create a REST API using the Axum framework, PostgreSQL for the database, and Diesel as the ORM. The entire application can be run in Docker for easy deployment and testing.

## Prerequisites

- [Docker](https://www.docker.com/get-started)
- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Axum](https://github.com/tokio-rs/axum)
- [Diesel](https://diesel.rs/docs/)

## Getting Started

1. Clone the repository:

```bash
git clone https://github.com/your-username/rust-axum-postgres-diesel-docker.git
cd rust-axum-postgres-diesel-docker
```
2. Run and build docker
```
docker-compose up --build
```

3. Init DB and update migration
```
make runInitDB
```

4. Go to ***localhost:3000***

### User Management

The user management feature provides essential operations related to user entities.

- **List Users**
  - Endpoint: `GET /users`
  - Description: Retrieve a list of all users.

- **Get a Single User**
  - Endpoint: `GET /users/{id}`
  - Description: Retrieve details for a specific user identified by their unique ID.

- **Delete a User**
  - Endpoint: `DELETE /users/{id}`
  - Description: Delete one user for a specific user ID

- **Create a New User**
  - Endpoint: `POST /users`
  - Description: Create a new user with the provided information