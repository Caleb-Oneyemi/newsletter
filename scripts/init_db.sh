#!/bin/bash
set -x
set -eo pipefail

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    echo >&2 "Use:"
    echo >&2 "    cargo install sqlx-cli --no-default-features --features rustls,postgres"
    echo >&2 "to install it."
    exit 1
fi

# use env variables for db with defaults if not present
DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_HOST="${POSTGRES_HOST:=localhost}"

# Skip if Postgres db container is already running
# -z: true if string length is zero
if [[ -z "${SKIP_DOCKER}" ]]
then
    CONTAINER_ID=$(docker ps --filter 'name=newsletter_db' --format '{{.ID}}')
    # -n: true if string length is NOT zero
    if [[ -n $CONTAINER_ID ]]; then
        echo >&2 "there is a postgres container already running, kill it with"
        echo >&2 "    docker kill ${CONTAINER_ID}"
        echo >&2 "    docker rm ${CONTAINER_ID}"
        exit 1
    fi
    
    # run postgres with docker in detach mode with maximum 1k connections
    docker run \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -p "${DB_PORT}":5432 \
        -d \
        --name "newsletter_db" \
        postgres -N 1000
fi

export PGPASSWORD="${DB_PASSWORD}"
# Keep pinging Postgres until it's ready to accept commands
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1 
done

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}

# create uses DATABASE_URL to execute
sqlx database create

sqlx migrate run
