# newsletter

A WIP email newsletter system built with rust


## Stack:

- [Rust](https://www.rust-lang.org)
- [Actix Web](https://github.com/actix/actix-web)
- [Postgres](https://www.postgresql.org/)
- [sqlx](https://github.com/launchbadge/sqlx)
- [docker](https://www.docker.com)

## Requirements

- [Rust Toolchain](https://rustup.rs/)
- [SQLx CLI](https://crates.io/crates/sqlx-cli): `cargo install sqlx-cli --no-default-features --features rustls,postgres`

## Getting Started

Run Postgres Container and Create Newsletter Database:

```sh
chmod +x scripts/init_db.sh

scripts/init_db.sh
```

If Postgres Container is already running then run with SKIP_DOCKER flag:

```sh
chmod +x scripts/init_db.sh

SKIP_DOCKER=true scripts/init_db.sh
```

Running the container

```sh
docker build --tag newsletter .
docker run -p 8000:8000 newsletter
```

## Logs

To prettify the output for logs.

```sh
# first install bunyan
cargo install bunyan
```

Prettify for Running Server:

```sh
cargo run | bunyan
```

Prettify for Tests:

```sh
# ALLOW_TEST_LOGS flag needed to show logs when running tests
ALLOW_TEST_LOGS=true cargo test | bunyan
```

## Environment Variables

A sample `.env.example` file is provider to show env variables like so:

```
APP_PORT=
DB_PORT=
```

If those keys are present in the main `.env` file, the config builder will use them to override the default values provided by the `.yaml` files in the `config` folder. If the keys are provided in the `.env` file without values, the process will panic.
