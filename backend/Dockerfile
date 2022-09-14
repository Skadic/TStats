FROM rust:1.63 as build

ARG DATABASE_URL
ARG OSU_CLIENT_ID
ARG OSU_CLIENT_SECRET
ARG REDIS_URL="redis://127.0.0.1"

# Create new shell project
RUN USER=root cargo new --bin backend
WORKDIR /backend

# Copy manifest
COPY ./Cargo.toml ./Cargo.lock* ./

# Build only dependencies
RUN cargo build --release && rm src/*.rs

# Now that the dependencies are build, copy sources
COPY ./src ./src
COPY ./Rocket.toml .

# Build for release
RUN rm ./target/release/deps/* && cargo build --release

# Move to a slimmer image
FROM debian:bullseye-slim

# Copy the built binary to our new image
COPY --from=build ./backend/target/release/backend .

# Try installing CA certificates
RUN mkdir /usr/local/share/ca-certificates
COPY ./certificates/* /usr/local/share/ca-certificates
RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates

CMD [ "./backend" ]
