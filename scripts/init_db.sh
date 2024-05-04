#!/usr/bin/env bash
#set -x
set -eo pipefail

if ! [ -x "$(command -v sqlx)" ]; then
  echo "Error: sqlx is not installed." >&2
  echo "You can install it by running 'cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres'" >&2
  exit 1
fi

if ! [ -x "$(command -v psql)" ]; then
  echo "Error: psql is not installed." >&2
  echo "You can install it by running 'sudo apt-get install postgresql-client'" >&2
  echo "You can install it by running 'brew install postgresql'" >&2
  exit 1
fi

# check if a custom user has been set, otherwise default to 'postgres'
DB_USER="${POSTGRES_USER:=postgres}"

# check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"

# check if a custom database name has been set, otherwise default to 'newsletter'
DB_NAME="${POSTGRES_DB:=newsletter}"

# check if a custom port has been set, otherwise default to '5432
DB_PORT="${POSTGRES_PORT:=5432}"

# check if a custom host has been set, otherwise default to 'localhost'
DB_HOST="${POSTGRES_HOST:=localhost}"

if [[ -z "${SKIP_DOCKER}" ]]; then
  # start postgres using docker
  docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}:5432" \
    -d postgres \
    postgres -N 1000
    # ^ Increased maximum number of connections for testing purposes
fi

echo "Waiting for postgres to start..."
until PGPASSWORD="$DB_PASSWORD"  psql -h "$DB_HOST" -U "$DB_USER" -p "$DB_PORT" -c '\q'; do
  >&2 echo "..."
  sleep 1
done

echo "Postgres is accepting connections."

DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
# supply sqlx with the connection string
export DATABASE_URL

sqlx database create
sqlx migrate run

echo "Database has been migrated."