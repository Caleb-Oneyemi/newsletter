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

## Tests

To prettify the output for test logs.

```sh
# first install bunyan
cargo install bunyan
```

```sh
# then run tests with the ALLOW_TEST_LOGS flag and bunyan
ALLOW_TEST_LOGS=true cargo test | bunyan
```
