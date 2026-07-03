FROM rust:1.85-slim AS builder

WORKDIR /app

COPY Cargo.* ./
COPY migrations ./migrations

RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release 2>/dev/null || true

COPY src ./src
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/posterbot /app/posterbot
RUN mkdir -p /app/data

CMD ["/app/posterbot"]
