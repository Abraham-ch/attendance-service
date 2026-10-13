# Attendance Service

Attendance tracking for students, their representatives and school periods: a Rust/Axum API backed by Postgres, and a React dashboard.

| Path | What | Stack |
| --- | --- | --- |
| [`api/`](./api) | REST + WebSocket API | Rust 2024, Axum, sqlx, aide (OpenAPI), JWT auth |
| [`web/`](./web) | Dashboard | React 19, Vite, TypeScript, Tailwind, React Router |
| `compose.yaml` | Local Postgres 17 | Docker Compose |

## Requirements

- Rust (stable, edition 2024)
- Node.js + [pnpm](https://pnpm.io)
- Docker + Docker Compose
- [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli): `cargo install sqlx-cli --no-default-features --features postgres`

## Quick start

```bash
# 1. Database
docker compose up -d

# 2. API (http://127.0.0.1:3000, docs at /docs)
cp api/.env.example api/.env
cd api
cargo run

# 3. Web (http://localhost:5173), in another terminal
cd web
echo "VITE_API_URL=http://127.0.0.1:3000" > .env
pnpm install
pnpm dev
```

The API runs pending migrations on startup. To get a user to log in with, seed the database from `api/`:

```bash
cargo run --bin user_seed
cargo run --bin student_seed
cargo run --bin period_seed
```

## Configuration

`api/.env` (see [`api/.env.example`](./api/.env.example)):

| Variable | Purpose |
| --- | --- |
| `DATABASE_URL` | Postgres connection string |
| `API_URL` | Address the API listens on, e.g. `127.0.0.1:3000` |
| `ALLOWED_ORIGIN` | Origin allowed by CORS (the web app URL) |
| `SECRET_KEY` | JWT signing secret |
| `SQLX_OFFLINE` | `true` to check queries against `api/.sqlx/` instead of a live database |
| `SMTP_HOST`, `SMTP_PORT`, `SMTP_USERNAME`, `SMTP_NAME`, `APP_PASSWORD` | Outgoing mail |

`web/.env`:

| Variable | Purpose |
| --- | --- |
| `VITE_API_URL` | Base URL of the API |

## API

Interactive docs (Scalar) are served at `/docs`. Everything except `POST /auth` and `POST /user` requires an `Authorization: Bearer <token>` header.

| Route | Methods |
| --- | --- |
| `/auth` | `POST` log in, returns `{ user, token }` |
| `/user` | `GET`, `POST` |
| `/user/get-me` | `GET` the logged-in user |
| `/user/{id}` | `GET`, `PATCH`, `DELETE` |
| `/student`, `/student/{id}` | `GET`, `POST`, `PATCH`, `DELETE` |
| `/representative`, `/representative/{id}` | `GET`, `POST`, `PATCH`, `DELETE` |
| `/period`, `/period/{id}` | `GET`, `POST`, `DELETE` |
| `/assistance` | WebSocket |

## Development

### API

Run these from `api/`:

```bash
cargo check
cargo test                  # needs Docker running (testcontainers)
sqlx migrate add <name>     # new migration in api/migrations/
cargo sqlx prepare          # after adding or changing a query
```

Keep `SQLX_OFFLINE=true` in `api/.env`. Without it every `cargo check` validates each query against the database in `DATABASE_URL`, which is slow. The trade-off is that `cargo sqlx prepare` has to be re-run (with the database up) whenever a `query!` changes, and the updated `api/.sqlx/` files committed; CI fails on a stale cache.

More on migrations and seeding in [`api/README.md`](./api/README.md).

### Web

Run these from `web/`:

```bash
pnpm dev          # dev server
pnpm build        # type-check + production build
pnpm lint         # oxlint
pnpm fmt          # oxfmt
```

## CI

[`.github/workflows/rust.yml`](./.github/workflows/rust.yml) runs on changes under `api/`: it starts Postgres, applies migrations, verifies the sqlx query cache, then builds and tests.

## License

MIT — see [LICENSE](./LICENSE).
