#!/usr/bin/env bash
set -x
set -eo pipefail

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD=${POSTGRES_PASSWORD:=postgres}
DB_NAME=${POSTGRES_DB:=users}
DB_PORT=${POSTGRES_PORT:=5432}

# launch a docker instance of psql if its not already running
if [[ -z $(docker ps | rg 'postgres') ]]; then
	docker run \
		-e POSTGRES_USER=${DB_USER} \
		-e POSTGRES_PASSWORD=${DB_PASSWORD} \
		-e POSTGRES_DB=${DB_NAME} \
		-p ${DB_PORT}:5432 \
		-d postgres \
		postgres -N 1000
fi

# keep checking if postgres is up
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
	>&2 echo "Postgres is unavailable - sleeping"
	sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
# sqlx migrate add create_users_table
sqlx migrate run
cargo sqlx prepare
