# My Example Operations Service

Bootstrap foundation for a Rust HTTP service. This version only provides startup, environment-based configuration, structured logging, and a health endpoint.

## Configuration

Configuration is loaded directly from environment variables at startup.

Required variables:

- none

Optional variables:

- `APP_HOST` with default `127.0.0.1`
- `APP_PORT` with default `3000`
- `APP_LOG` with default `info`

Invalid configuration causes startup to exit non-zero with an error log.

## Run

```bash
cp .env.example .env
export $(grep -v '^#' .env | xargs)
cargo run
```

The service listens on `http://127.0.0.1:3000` by default.

Health check:

```bash
curl http://127.0.0.1:3000/health
```

## Verify

Run the full local verification set:

```bash
cargo check
cargo test
cargo fmt --check
cargo clippy -- -D warnings
```
