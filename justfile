default: # List the available recipes
  @just --list --justfile {{justfile()}}

# Run the redis container
redis:
  podman run --rm --name tstats-redis -p 6379:6379 -d redis

# Run the postgres container
postgres:
  podman run --rm --name tstats-postgres -p 5432:5432 -e POSTGRES_USER=root -e POSTGRES_PASSWORD=root -d postgres

# Run the backend in debug mode
backend:
  cargo run --manifest-path backend/Cargo.toml

# Builds the backend as a container and runs it
container-backend:
  podman build --tag tstats-backend-manual:pre-alpha -f Dockerfile
  podman run --rm -p 3000:3000 --net tstats --name tstats-backend-manual tstats-backend-manual:pre-alpha

# Run the frontend as a dev server
frontend:
  cd frontend && bun run dev
