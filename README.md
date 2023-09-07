# rust_learn

## setup

```bash

# install libpq and psql
arch -arm64 brew install libpq

# install diesel cli
cargo install --target=aarch64-apple-darwin diesel_cli --no-default-features --features postgres

```

## start database

```bash
docker compose up
```

## debugging

```bash
# get the password from the compose.yml
DB_USER=anuaimi
DB_NAME=travel
psql -h localhost -U $DB_USER -d $DB_NAME
```
