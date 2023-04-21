### Project

This is a project of a Desktop APP to do the logistic of products. Project used to learn the Rust language, and the SvelteKit framework.

### Builded with

- [Tauri](https://tauri.app/) for the backend logic
- [SeaORM](https://www.sea-ql.org/SeaORM/) for handle the database
- [SvelteKit](https://kit.svelte.dev/) for handle the client side

### Running the project

#### Prerequisites

- [Rust/Cargo](https://www.rust-lang.org/)
- [Node](https://nodejs.org/en)
- SeaORM CLI `cargo install sea-orm-cli`
- [Docker](https://www.docker.com/) for run the database

### Steps to run

- Clone the repo
- Install the node packages with `yarn` or `npm install`
- Create a .env on the root, and a .env inside the `src-tauri/Docker` (use the .env.example as base)
- Run the database with `docker compose -f Docker/docker-compose.yaml up -d`
- Run the migrations with `sea-orm-cli generate entity --with-serde both --serde-skip-hidden-column --serde-skip-deserializing-primary-key -o entity/src`
- Run the seeds with `cargo run --bin seed`
- Run the project with `yarn tauri dev` or `npm run tauri dev`
