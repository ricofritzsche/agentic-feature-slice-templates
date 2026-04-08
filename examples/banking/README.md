# Banking Feature Slices Example

Minimal Rust HTTP service for experimenting with feature-slice implementation in a banking example.

## What It Does

The service currently exposes:

* `GET /health` for a basic health check
* `POST /accounts` to open one retail account for one individual applicant

The `open_account` feature validates the request, generates an `account_id`, appends one `account_opened` fact through factstore, and returns the created account payload.

## Configuration

The service loads configuration from environment variables at startup.

Required variables:

* `APP_PORT`: port to bind, for example `3000`
* `FACTSTORE_SQLITE_PATH`: file path for the runtime SQLite fact store

Optional variables:

* `APP_HOST`: bind host, defaults to `127.0.0.1`
* `RUST_LOG`: tracing filter, defaults to `info`

Example local setup:

```bash
export APP_PORT=3000
export APP_HOST=127.0.0.1
export FACTSTORE_SQLITE_PATH=./data/banking_feature_slices_example.sqlite
export RUST_LOG=info
```

## Run

```bash
APP_PORT=3000 FACTSTORE_SQLITE_PATH=./data/banking_feature_slices_example.sqlite cargo run
```

The service starts with structured JSON logs, opens a local `factstore-sqlite` database file, and serves `GET /health` plus `POST /accounts`.

Health check:

```bash
curl --request GET \
  http://127.0.0.1:3000/health
```

Open an account:

```bash
curl --request POST \
  http://127.0.0.1:3000/accounts \
  -H 'content-type: application/json' \
  -d '{
    "first_name": "Anna",
    "last_name": "Schneider",
    "date_of_birth": "1992-04-18",
    "residential_address": {
      "street_line_1": "Kastanienallee 15",
      "street_line_2": "",
      "postal_code": "10435",
      "city": "Berlin",
      "country_code": "DE"
    },
    "government_id": "D1234567X"
  }'
```

## Verify

```bash
cargo check
cargo test
cargo fmt --check
cargo clippy -- -D warnings
```
