# Attendance Service

Attendance tracking for a school: students, their representatives and school periods, with roll call by QR code. A Rust/Axum API backed by Postgres, and a React dashboard.

| Path | What | Stack |
| --- | --- | --- |
| [`api/`](./api) | REST + WebSocket API | Rust 2024, Axum, sqlx, aide (OpenAPI), JWT auth |
| [`web/`](./web) | Dashboard | React 19, Vite, TypeScript, Tailwind, React Router |
| `compose.yaml` | Local Postgres 17 | Docker Compose |

## Features

- **Roles.** `SuperAdmin` manages admins and users, `Admin` (teachers) manages users and all school records, `User` (students) only sees their own attendance. See [Roles](#roles).
- **Roll call by QR**, two ways:
  - Staff scan each student's own QR (from their printed card or their phone) and the record shows up live for every staff screen.
  - Staff show the **daily QR** on a screen, students scan it from their phone to check themselves in. The code changes every day and only works near where it's shown (the browser's location is checked against it).
- **One record per student per day**, enforced by the database.
- **Printable student cards** (credit-card size) with picture, name, DNI and the student's QR. "Save as PDF" from the print dialog gives the file.
- **Avatars** stored in a Supabase Storage bucket: set by an admin when creating or editing a user, by students when activating their account, or by anyone from their own avatar in the sidebar.
- **Invites.** Creating a student with an email sends them an activation link; they pick a password (and optionally a picture) and get their own `User` account.

## Requirements

- Rust (stable, edition 2024)
- Node.js + [pnpm](https://pnpm.io)
- Docker + Docker Compose
- [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli): `cargo install sqlx-cli --no-default-features --features postgres`

## Quick start

All commands run from the repository root.

```bash
# 1. Database
docker compose up -d

# 2. API (listens on API_URL, docs at /docs)
cp api/.env.example .env        # then fill it in, see Configuration
sqlx migrate run --source api/migrations
cargo run

# 3. Web (http://localhost:5173), in another terminal
echo "VITE_API_URL=http://127.0.0.1:3000" > web/.env
pnpm install
pnpm dev
```

> The API also applies pending migrations when it starts (`sqlx::migrate!` in `api/src/main.rs`), so review new files in `api/migrations/` before restarting it against a database you care about.

To get accounts to log in with, seed the database (the users and their passwords are in [`api/src/bin/user_seed.rs`](./api/src/bin/user_seed.rs)):

```bash
cargo run --bin user_seed
cargo run --bin student_seed
cargo run --bin period_seed
```

## Configuration

`.env` at the repository root (template: [`api/.env.example`](./api/.env.example)):

| Variable | Purpose |
| --- | --- |
| `DATABASE_URL` | Postgres connection string |
| `API_URL` | Address the API listens on, e.g. `127.0.0.1:3000` |
| `ALLOWED_ORIGIN` | Origin allowed by CORS, the web app's URL. Also used for the link in invite emails |
| `SECRET_KEY` | JWT signing secret (login tokens and the daily QR) |
| `SQLX_OFFLINE` | `true` to check queries against `.sqlx/` instead of a live database |
| `SMTP_HOST`, `SMTP_PORT`, `SMTP_USERNAME`, `SMTP_NAME`, `APP_PASSWORD` | Outgoing mail (invites) |
| `SUPABASE_URL` | Project URL, `https://<project-id>.supabase.co`. Not the S3 endpoint from the Storage settings |
| `SUPABASE_SERVICE_ROLE_KEY` | Secret key (`sb_secret_…`) or legacy `service_role` key from **Project Settings → API Keys**. Not the S3 access key. Stays on the API |
| `SUPABASE_AVATAR_BUCKET` | Name of the avatars bucket. It must be **public**, the app shows avatars by their public URL |
| `CHECK_IN_RADIUS_METERS` | Optional, default `150`. How far from the daily QR a student may check in; phone GPS indoors is often off by 20–100 m |

`web/.env`:

| Variable | Purpose |
| --- | --- |
| `VITE_API_URL` | Base URL of the API |

## Roles

The API enforces every rule below; the dashboard only hides what a role can't use.

| | SuperAdmin | Admin | User (student) |
| --- | --- | --- | --- |
| Create / edit / delete users | Admins and users | Users | — |
| Students, representatives, periods | Read and write | Read and write | — |
| QR scan page | Scan students · show daily QR | Scan students · show daily QR | Scan daily QR · show own QR |
| Overview and Assistance | Whole school | Whole school | Own records only |
| Own avatar | ✓ | ✓ | ✓ |

Nobody can create, edit or delete the SuperAdmin through the API.

## API

Interactive docs (Scalar) are served at `/docs`. Everything except the `/auth` routes needs an `Authorization: Bearer <token>` header (the WebSocket also accepts `?token=`).

| Route | Who | What |
| --- | --- | --- |
| `POST /auth` | public | Log in, returns `{ user, token }` |
| `GET /auth/invite?token=` | public | Who an invite is for |
| `POST /auth/register` | public | Activate an invite: `{ token, password }`, answers like a login |
| `GET /user/get-me` | any | The logged-in user |
| `POST /user/me/avatar` | any | Set your own avatar, body is the image |
| `GET, POST /user` | staff | List (paginated, `?search=`) or create users |
| `GET, PATCH, DELETE /user/{id}` | staff | Within the role rules above |
| `POST /user/avatar` | staff | Upload an image, returns `{ url }` for a new user's `avatar` |
| `/student`, `/student/{id}` | staff | `GET`, `POST`, `PATCH`, `DELETE` |
| `/representative`, `/representative/{id}` | staff | `GET`, `POST`, `PATCH`, `DELETE` |
| `GET /period` | any | All periods |
| `POST /period`, `DELETE /period/{id}` | staff | Manage periods |
| `GET /period/{id}/assistance` | staff | The roll of a period |
| `GET /assistance` | staff | WebSocket: send `{ student_id, period_id }` to mark a student, receive every new record |
| `GET /assistance/daily-code?lat=&lng=` | staff | Today's check-in code for the QR, logs where it was generated |
| `POST /assistance/check-in` | any | `{ code, lat, lng }`: the logged-in student checks in with the daily QR |
| `GET /assistance/me` | any | The logged-in student's own records |

Image bodies (`/user/avatar`, `/user/me/avatar`) are JPEG, PNG or WebP, up to 1 MB; the dashboard crops pictures to 256 px before sending them.

## Development

From the repository root:

```bash
cargo check
cargo test                                # needs Docker running (testcontainers)
sqlx migrate add --source api/migrations <name>
sqlx migrate info --source api/migrations # what's applied and what's pending
sqlx migrate run --source api/migrations
cargo sqlx prepare --workspace            # after adding or changing a query
```

Keep `SQLX_OFFLINE=true` in `.env`. Without it every `cargo check` validates each query against the database in `DATABASE_URL`, which is slow. The trade-off is that `cargo sqlx prepare --workspace` has to be re-run (with an up-to-date database, `SQLX_OFFLINE=false cargo sqlx prepare --workspace` if `.env` sets it to `true`) whenever a `query!` changes, and the updated `.sqlx/` files committed; CI fails on a stale cache.

Web, from the repository root:

```bash
pnpm dev          # dev server
pnpm build        # type-check + production build
pnpm lint         # oxlint
pnpm fmt          # oxfmt
```

More on the database and seeding in [`api/README.md`](./api/README.md).

## Deployment

- **Web:** on Vercel, set the project's Root Directory to `web`. [`web/vercel.json`](./web/vercel.json) sends every path to `index.html` so routes like `/dashboard/student` load directly.
- **HTTPS is required** for the QR features: browsers only give pages the camera and the location over HTTPS (or `localhost`). With the web on HTTPS, `VITE_API_URL` must be HTTPS too, or the browser blocks the requests.

## CI

[`.github/workflows/rust.yml`](./.github/workflows/rust.yml) runs on changes under `api/` and `.sqlx/`: it starts Postgres, applies migrations, verifies the sqlx query cache, then builds and tests.

## License

MIT — see [LICENSE](./LICENSE).
