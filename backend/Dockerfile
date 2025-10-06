# syntax=docker/dockerfile:1

##########
# Build stage
##########
FROM rust:1.85-slim AS builder
WORKDIR /app

# Install only what’s needed for OpenSSL + SQLite builds
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev libsqlite3-dev \
 && rm -rf /var/lib/apt/lists/*

# 1) Create dummy project to prime Cargo cache
RUN USER=root cargo new --bin arena-ai-backend
WORKDIR /app/arena-ai-backend

# 2) Copy manifests for caching
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release || true

# 3) Copy real sources + migrations
COPY src ./src
COPY migrations ./migrations
COPY assets ./assets

# 4) Final build
RUN cargo build --release

##########
# Runtime stage
##########
FROM debian:bookworm-slim AS runtime

# Install minimal runtime deps
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary and migrations into the runtime image
COPY --from=builder /app/arena-ai-backend/target/release/arena-ai-backend /usr/local/bin/arena-ai-backend
COPY --from=builder /app/arena-ai-backend/migrations ./migrations
COPY --from=builder /app/arena-ai-backend/assets ./assets

COPY scripts/seed_demo.sh /app/seed_demo.sh
RUN chmod +x /app/seed_demo.sh

# In runtime stage
WORKDIR /app

# Make sure uploads directory exists
RUN mkdir -p /app/uploads/avatars && chmod -R 755 /app/uploads

# Add a VOLUME for persistence (optional, better for dev/prod separation)
VOLUME ["/app/uploads"]

# ---- Runtime env defaults (can be overridden at runtime) ----
ENV RUST_LOG=info \
    PORT=8080 \
    DATABASE_URL=file:/app/data/app.db?mode=rwc

# Create the db directory at build time (permissions handled at runtime by Rust code)
RUN mkdir -p /app/data

EXPOSE 8080

# No entrypoint script needed, the binary handles setup gracefully
CMD ["arena-ai-backend"]