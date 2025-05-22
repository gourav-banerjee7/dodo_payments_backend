# Stage 1: Build
FROM rustlang/rust:nightly-slim AS builder
WORKDIR /app
COPY . .

ENV SQLX_OFFLINE=true

RUN cargo build --release

# Stage 2: Runtime (same base to avoid glibc issues)
FROM rustlang/rust:nightly-slim AS runtime
WORKDIR /app

RUN apt-get update && apt-get install -y postgresql-client && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/dodo_payments_backend .
COPY .env .

EXPOSE 3000
CMD ["./dodo_payments_backend"]
