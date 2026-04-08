# AGENTS.md

## Repository purpose

This repository is a Rust reference project for learning and testing agentic coding workflows.
The example system is a Parcel Locker Operations Service.

## Working rules

- Prefer standard Cargo conventions before custom structure.
- Keep the bootstrap minimal and runnable.
- Do not add speculative abstractions or future-proof folders.
- Do not introduce generic technical roles such as service, manager, repository, helper, utils, shared, common, or core.
- Keep code local to the concrete task being implemented.
- Add dependencies only when they are needed by the current runnable path.
- Keep startup deterministic and explicit.

## Bootstrap expectations

- One documented startup command
- Explicit configuration
- Structured startup logging
- A health endpoint if this is a long-running service
- At least one minimal verification path

## Verification

Before finishing work, run:

- `cargo check`
- `cargo test`

If configured, also run:

- `cargo fmt --check`
- `cargo clippy -- -D warnings`

## Skills

When the task is to create or review the initial Rust project skeleton, use the `rust-project-bootstrap` skill.

## Features

When a feature spec exists under specs/features/*.md, treat that file as the authoritative contract for the feature’s HTTP behavior, validation rules, business rules, duplicate handling, and expected outcomes.

When implementing a feature, use the `feature-slice-rust` skill to keep the feature local, explicit, and small.


## Response expectations

Report briefly:

- what changed
- how startup works
- how configuration is loaded and validated
- how to verify the project locally