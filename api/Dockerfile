# Build stage
FROM rust:1-slim AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Cache dependencies
COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release 2>&1 && rm -rf src

# Build actual app
COPY src/ ./src/
RUN touch src/main.rs && cargo build --release

# Run stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/nodi-api /usr/local/bin/

EXPOSE 3000
CMD ["nodi-api"]
