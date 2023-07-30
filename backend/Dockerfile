FROM rust:slim-buster AS rs

# Dummy build to cache dependencies
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src && echo "fn main() {}" > src/dummy.rs
RUN sed -i 's/main.rs/dummy.rs/' Cargo.toml
RUN cargo build --release
RUN sed -i 's/dummy.rs/main.rs/' Cargo.toml

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
CMD [ "./tstats_backend" ]
