# Rust Project Bootstrap

Use this skill when the task is to create or review the **first working skeleton** of a new Rust service, CLI, or application.

This skill defines the **initial foundation only**. It does not define product logic, feature workflows, or domain-specific behavior.

## Outcome

Produce a starter project that is:

* easy to run locally
* explicit about configuration
* observable from first startup
* testable immediately
* aligned with standard Rust conventions
* clean enough that later feature work has an obvious place to live

## Default Approach

Prefer the **smallest valid Rust setup** that creates a reliable starting point.

Start from:

* `cargo new --bin` for executables and services
* `cargo new --lib` only when the project is clearly a library

Use Cargo defaults first.

Do not invent custom structure, technical layers, or future-facing abstractions at bootstrap stage.

The first goal is a **working, verifiable project**, not a designed architecture.

## Structure

Prefer Rust’s native defaults before adding folders.

Typical starting point:

* `Cargo.toml`
* `src/main.rs` for a binary
* `src/lib.rs` only if shared logic is already necessary
* `tests/` only when an integration or smoke test adds immediate value
* `.env.example` only when environment variables are needed
* `README.md` with exact run and verification steps

Only add folders such as `scripts/`, `docs/`, `config/`, or `deploy/` when they solve a real problem now.

Do not create empty placeholder folders.

Do not create feature slices before the first real feature exists.

## Bootstrapping

The first runnable path must be obvious.

Include:

* one documented startup command
* one documented verification command
* clear failure behavior when required configuration is missing

Typical commands:

* `cargo run`
* `cargo test`

If the project is stricter from day one, also include:

* `cargo check`
* `cargo fmt --check`
* `cargo clippy -- -D warnings`

## Configuration

Keep configuration explicit and narrow.

Rules:

* separate code from environment-specific values
* validate required configuration at startup
* use descriptive variable names
* provide defaults only when they are truly safe
* never commit secrets
* document where configuration is loaded from

At minimum, document:

* required variables
* optional variables
* default values
* example local setup

Prefer simple environment-based configuration first.

## Logging

Add logging from the first runnable version.

For Rust, prefer:

* `tracing`
* `tracing-subscriber`

Logging should:

* identify startup clearly
* identify configuration load and validation failures clearly
* identify external dependency failures clearly
* avoid noisy debug output by default

Minimum startup logs:

* application start
* configuration loaded
* service listening or main process started
* fatal startup failure or clean shutdown

## Startup And Lifecycle

Bootstrap must define a clear startup contract.

For long-running services:

* startup either succeeds fully or exits non-zero
* listening address and port are explicit
* health endpoint exists when appropriate
* readiness exists only when it reflects real ability to serve

For CLI or batch tools:

* entrypoint is deterministic
* inputs and outputs are explicit
* failure returns non-zero exit code

## Validation

Add one small validation path immediately.

Examples:

* a smoke test that starts the service and checks `/health`
* a CLI test using sample input
* a startup config validation test

The purpose is to prove the bootstrap works, not to simulate full coverage.

## Documentation

Keep initial documentation short and operational.

Document only what is needed to:

* run the project
* configure the project
* verify the project

Avoid design writing, architecture language, or speculative explanation in the bootstrap stage.

## Boundaries

This skill does not cover:

* domain design
* feature design
* business workflows
* optimization
* infrastructure redesign
* speculative abstractions
* shared cross-feature code structures

If the task moves into a concrete feature, stop using this skill alone and switch to a feature-specific skill.

## Rust-Specific Rules

* Prefer Cargo conventions over custom folder schemes
* Keep `main.rs` small and obvious
* Extract only when the code becomes harder to read or test
* Do not introduce `service`, `manager`, `repository`, `helper`, or similar generic technical roles
* Do not create `shared`, `common`, `utils`, or `core` folders at bootstrap stage
* Do not create a “domain” folder unless there is already real domain logic to place there
* Add dependencies only when they are used in the first runnable path
* Keep the dependency set small and explain why each dependency exists

## Agent Checklist

Before finishing, verify:

* the project uses standard Rust conventions
* the project starts through one documented command
* required configuration is explicit
* startup behavior is deterministic
* logs are present and useful
* health/readiness exists only when appropriate
* at least one minimal validation path runs
* no speculative abstraction was added
* no generic technical folders were created without clear need

## Response Pattern

When using this skill, report:

* what structure was created or aligned
* how startup works
* how configuration is loaded and validated
* what logging was added
* how to verify the bootstrap locally
