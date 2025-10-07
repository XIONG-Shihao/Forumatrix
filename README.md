# Forumatrix
Forumatrix is a lightweight social app + collaborative docs.
Ship fast, keep it simple: posts, comments, notifications, chat badge, admin tools — and a minimal MS-Word–style editor with A4 pages and up to 10 collaborators per doc.

## What you can do
### Forum
1. Create posts, comment posts, and reply to other's comment.
2. Like the comment and posts
3. Viewing others' profile to see their posts, comments and liked posts
4. Suspend users (admin only): Suspend a user account and force logout all sessions

### Docs (lightweight collaborative editor)
1. Create a document (1–10 A4 pages).
2. Membership = owner + editors. Cap: max 10 members (owner + up to 9 editors).
3. Join requests: any user can request to join; owner can approve/deny.
4. Edit pages with inline style runs: Title / Heading / Body.

## Tech stack
Backend: Rust, Axum, SQLx, SQLite
Frontend: React + TypeScript + Vite
Auth: Cookie session (HTTP-only), simple helpers
Persistence: SQLite with SQLx migrations
Editor: ContentEditable (custom), storing page HTML bytes for now (placeholder for future CRDT/Yjs)

## Quickstart
### Prereqs
Rust (stable), Node 18+, SQLite
```bash
# Docker build
make build

# Docker run
make up

# Docker stop
make down

# Docker rebuild
make rebuild
```


### Local-run: Backend
```bash
# from repo root
cd backend

# (optional) set env if your app expects them
# export DATABASE_URL=sqlite://arena.db
# export SESSION_SECRET=some-long-random-string
# export PORT=8080

# run (migrations are applied by the app or can be run via sqlx-cli)
cargo run
# backend listens on http://localhost:8080
```

If you prefer manual migrations:
```bash
sqlx database create
sqlx migrate run
```

### Local-run: Frontend
```bash
cd frontend
npm i
npm run dev
# http://localhost:5173
```