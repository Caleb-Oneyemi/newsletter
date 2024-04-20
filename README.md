# newsletter

An email newsletter system built with rust

## Requirements

- [Rust Toolchain](https://rustup.rs/)
- [SQLx CLI](https://crates.io/crates/sqlx-cli): `cargo install sqlx-cli --no-default-features --features rustls,postgres`

## Setup

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
