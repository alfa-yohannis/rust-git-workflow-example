# Stage 1: Build
FROM rust:1.83 AS builder

WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo build --release

# Stage 2: Runtime (compatible with rust:1.83)
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust-example /usr/local/bin/rust-example

EXPOSE 8080
CMD ["rust-example"]
