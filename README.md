# Attendance Service

Rust/Axum API for managing users, students, and representatives attendance, using `aide` for OpenAPI docs and `sqlx` + Postgres for persistence.

## Requirements

- Rust (edition 2024)
- Docker + Docker Compose
- [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) for managing migrations:

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

## Setup

Create a `.env` file in the project root (see variables used in `src/main.rs`):

```env
DATABASE_URL=postgres://postgres:password@localhost:5432/attendance_service
API_URL=127.0.0.1:3000
ALLOWED_ORIGIN=http://localhost:5173
SECRET_KEY=your-jwt-secret
```

## Database (Docker)

Start Postgres:

```bash
docker compose up -d
```

Stop it:

```bash
docker compose down
```

Stop and wipe data volume:

```bash
docker compose down -v
```

## Migrations (sqlx-cli)

Migrations live in `migrations/`. The app also runs pending migrations automatically on startup (`sqlx::migrate!` in `src/main.rs`), but use the CLI during development:

```bash
# Create a new migration (generates a timestamped .sql file)
sqlx migrate add <name>

# Run all pending migrations
sqlx migrate run

# Check migration status
sqlx migrate info

# Revert the last migration
sqlx migrate revert
```

`sqlx-cli` reads `DATABASE_URL` from `.env` automatically.

## Running the app

```bash
cargo run
```

Server listens on `API_URL`. API docs (Scalar UI) are served at `/docs`.

## Seeding data

Seed binaries live in `src/bin/` and connect using `DATABASE_URL`:

```bash
cargo run --bin user_seed
cargo run --bin student_seed
```

## Recommendations

- **Dockerfile for the app itself** — `compose.yaml` only runs Postgres; adding an `attendance-service` build stage would let the whole stack run with one `docker compose up`.

## License

MIT — see [LICENSE](./LICENSE).
