FROM rust:1.63 as build

# Create new shell project
RUN USER=root cargo new --bin backend
WORKDIR /backend

# Copy manifest
COPY ./Cargo.toml ./Cargo.lock ./

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

CMD [ "./backend" ]
