FROM rust:slim-buster AS rs

# Dummy build to cache dependencies
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Proper project build
COPY src/ ./src/
RUN cargo build --release
RUN strip target/release/tstats_backend
# Update ca certificates
RUN apt-get install -y --no-install-recommends ca-certificates
RUN update-ca-certificates

# Image for running
FROM debian:buster-slim
COPY --from=rs /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=rs target/release/tstats_backend .
COPY .en[v] .
CMD ./tstats_backend
