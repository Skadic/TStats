# TStats

A web-app that allows looking at the results of tournament stages including player rankings, frequency of picks etc.
There is barely much of anything implemented yet so this is nowhere near ready.

This project uses [Svelte](https://svelte.dev/) and [Tailwind CSS](https://tailwindcss.com/) in the frontend, and [Rust](https://www.rust-lang.org/), [Axum](https://github.com/tokio-rs/axum) and [SurrealDB](https://surrealdb.com/) in the backend so far. 
This might be overkill but it is mostly for me to learn how to do webdev stuff.

## Prerequisites

You should have Rust and NodeJS installed and have a local instance of SurrealDB running. You can use the start script in the backend directory to start a container.
The script uses podman, but you can use docker instead by replacing `podman` with `docker` in the script.

## Building & Running

In `frontend/`:
```
npm i
npm run dev
```

In `backend/`:

```
cargo run
```
