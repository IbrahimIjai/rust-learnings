# Axum Blog API

A RESTful blog API built with Rust.

## Tech Stack

| Crate                  | Purpose                                       |
| ---------------------- | --------------------------------------------- |
| **axum**               | Web framework (routing, extractors, handlers) |
| **tokio**              | Async runtime                                 |
| **sqlx**               | Async PostgreSQL driver + migrations          |
| **serde / serde_json** | JSON serialization/deserialization            |
| **uuid**               | UUID generation for primary keys              |
| **chrono**             | Timestamps (`created_at`, `updated_at`)       |
| **thiserror**          | Custom error types                            |
| **secrecy**            | Sensitive value handling                      |
| **dotenvy**            | `.env` file loading                           |

## Project Structure

```
08-axum-blog-api/
├── migrations/                  # SQL migration files
│   └── 20260222_first-migrations.sql
├── src/
│   ├── main.rs                  # Entry point
│   ├── config.rs                # DB connection + migration runner
│   ├── error.rs                 # AppError enum + IntoResponse impl
│   ├── app/
│   │   ├── mod.rs               # App bootstrap (create_app)
│   │   ├── router.rs            # Root router + health check
│   │   └── state.rs             # SharedState (Arc<AppState>)
│   ├── author/
│   │   ├── mod.rs               # Author routes + author_exists helper
│   │   ├── models.rs            # Author, CreateAuthorRequest, AuthorResponse
│   │   ├── handlers.rs          # CRUD handlers
│   │   └── queries.rs           # SQL queries
│   └── post/
│       ├── mod.rs               # Post routes
│       ├── models.rs            # Post, CreatePostRequest, PostResponse
│       ├── handlers.rs          # CRUD handlers
│       └── queries.rs           # SQL queries
├── .env                         # PORT, DATABASE_URL
├── Cargo.toml
└── README.md
```

## Setup

### Prerequisites

- Rust (latest stable)
- Docker (for PostgreSQL)

### 1. Start PostgreSQL

```bash
docker run -d --name postgres-db \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=axum_server \
  -p 5432:5432 \
  -v postgres-data:/var/lib/postgresql/data \
  postgres:15
```

### 2. Configure `.env`

```
PORT=3310
DATABASE_URL=postgres://postgres:password@localhost:5432/axum_server
```

### 3. Run

```bash
cargo run
```

Migrations run automatically on startup. Server starts at `127.0.0.1:3310`.

## API Routes

### Authors

| Method   | Route           | Description                      |
| -------- | --------------- | -------------------------------- |
| `POST`   | `/authors`      | Create author                    |
| `GET`    | `/authors`      | List authors (`?limit=&offset=`) |
| `GET`    | `/authors/{id}` | Get author by ID                 |
| `PATCH`  | `/authors/{id}` | Update author                    |
| `DELETE` | `/authors/{id}` | Delete author                    |

### Posts

| Method   | Route         | Description                               |
| -------- | ------------- | ----------------------------------------- |
| `POST`   | `/posts`      | Create post                               |
| `GET`    | `/posts`      | List posts (`?author_id=&limit=&offset=`) |
| `GET`    | `/posts/{id}` | Get post by ID                            |
| `PATCH`  | `/posts/{id}` | Update post                               |
| `DELETE` | `/posts/{id}` | Delete post                               |

### Other

| Method | Route     | Description  |
| ------ | --------- | ------------ |
| `GET`  | `/`       | Hello world  |
| `GET`  | `/health` | Health check |

## SQLx Commands

```bash
# Install SQLx CLI
cargo install sqlx-cli --no-default-features --features postgres

# Create a new migration
sqlx migrate add <migration_name>

# Run pending migrations manually
sqlx migrate run

# Revert last migration
sqlx migrate revert

# Check migration status
sqlx migrate info

# Reset database (drop + recreate)
docker exec postgres-db psql -U postgres -c "DROP DATABASE axum_server;"
docker exec postgres-db psql -U postgres -c "CREATE DATABASE axum_server;"
```
