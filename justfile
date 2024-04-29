set dotenv-load
set dotenv-filename := ".env.local"

default: # List the available recipes
  @just --list --justfile {{justfile()}}

# Run the redis container
redis:
  podman run --rm --name tstats-redis -p 6379:6379 -d redis

# Run the dragonfly container
dragonfly:
  podman run --network=host --rm --name tstats-redis -p 6379:6379 --ulimit memlock=-1 -d docker.dragonflydb.io/dragonflydb/dragonfly

# Run the postgres container
postgres:
  podman run --rm --name tstats-postgres -p 5432:5432 -e POSTGRES_USER=root -e POSTGRES_PASSWORD=root -d postgres

# Run the backend in debug mode
backend:
  cargo run --manifest-path backend/Cargo.toml

# Runs caddy
caddy:
  podman run --rm -v caddy_data:/data:z -v $PWD/Caddyfile:/etc/caddy/Caddyfile:z --name tstats-caddy -p 9900:9900 -p 9901:9901 caddy:latest

# Builds all the web-app containers and runs them
compose:
  podman-compose up --build -t 0 --force-recreate

# Run the frontend as a dev server
frontend:
  cd frontend && bun run dev

# Apply all migrations to the database
migrate:
  sqlx migrate run --source ./backend/model/migrations

# Revert the last migration to the database
revert_one_migration:
  sqlx migrate revert --source ./backend/model/migrations

# Revert all migrations to the database
revert_all_migrations:
  sqlx migrate revert --source ./backend/model/migrations --target-version 0

# Generate entities for the backend from the database schema
generate_entities:
  sea-orm-cli generate entity --expanded-format --with-serde both --with-copy-enums -o ./backend/model/src/model



