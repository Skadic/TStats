FROM lukemathwalker/cargo-chef:latest-rust-alpine AS chef
WORKDIR /app

FROM chef AS planner
COPY backend/ .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY backend/ .
COPY ./proto ../proto
RUN ls -l ../proto
RUN apk add protoc protobuf-dev
RUN PROTOC=$(which protoc) cargo build --release --bin tstats_backend

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates
WORKDIR /app
COPY --from=builder /app/target/release/tstats_backend /usr/local/bin
ENTRYPOINT ["/usr/local/bin/tstats_backend"]

