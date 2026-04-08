---
name: feature-slice-rust
description: Create or modify a self-contained Rust feature slice with explicit boundaries, local naming, visible IO, and a small internal shape. Avoid generic technical roles, cross-feature dependencies, and one-file feature collapse.
---

## Purpose

Use this skill when implementing or changing a feature in this repository.

The goal is to keep each feature local, readable, and replaceable. A feature should describe its own execution directly instead of being spread across generic technical roles.

This skill defines feature structure. It does not define one specific storage technology. If the feature uses factstore, combine this skill with the separate factstore usage skill.

## Core idea

A feature is a local unit of behavior under:

- `src/features/<feature-name>/`

A feature may contain:

- boundary code
- pure decision or transformation logic
- explicit IO

Do not move domain-specific behavior into shared abstractions outside the feature.

## Non-negotiable rules

### 1. Keep the feature local

- Put feature code under `src/features/<feature-name>/`
- Keep behavior local to that feature
- Do not spread feature logic across generic shared folders
- Do not move domain-specific behavior into global abstractions

### 2. No generic technical roles

Do not create files, folders, or types named:

- `service`
- `manager`
- `repository`
- `helper`
- `utils`
- `common`
- `shared`
- `core`
- `domain`

Do not recreate layered architecture inside the feature under different names.

### 3. No cross-feature dependencies by default

- Do not import another feature's internal logic
- Do not reuse another feature's internal loaders, decisions, or write steps

Shared infrastructure is allowed only when it is truly infrastructure, for example:

- config
- logger
- database pool
- HTTP router setup
- external clients

### 4. Keep IO explicit

- IO must stay visible
- Reads and writes must not hide inside supposedly pure logic
- Shell code may load data and append or persist results
- Pure logic must not perform IO

## Internal feature shape

Do not collapse a multi-concern feature into one file.

If a feature contains several distinct concerns, split it into a small set of focused files inside the feature. Typical concerns are:

- HTTP boundary
- request parsing and boundary-local validation
- response mapping
- pure decision or transformation logic
- explicit append, load, or write step

Use the smallest split that keeps those concerns readable. Do not force a fixed file count.

## Naming inside a feature

### Boundary files

Boundary files may use local boundary-role names such as:

- `http_handler.rs`
- `request.rs`
- `response.rs`
- `routes.rs`

These names are acceptable because they describe a local role inside the feature, not a global project abstraction.

### Pure files

If a feature contains real pure logic, name the file after the actual work it performs.

Examples:

- `decide_registration.rs`
- `validate_registration.rs`
- `build_location_registered_fact.rs`
- `classify_access.rs`

Do not create a pure file unless the feature actually contains pure logic worth isolating.

### Shell files

If a file performs explicit IO, name it after that work.

Examples:

- `append_fact.rs`
- `load_registration_context.rs`
- `persist_assignment.rs`

Store writes are shell. Do not treat append or persistence steps as pure logic.

### File names to avoid

Do not name files after HTTP verbs such as:

- `post_*`
- `get_*`
- `put_*`
- `delete_*`

Do not replace that with one broad catch-all file for the whole feature when the feature already contains multiple concerns.

## Functional Core / Imperative Shell

Use this distinction when it is actually helpful.

### Core

Pure logic only.

Examples:

- decision making
- classification
- transformation
- fact construction from already validated input

### Shell

Boundaries and effects.

Examples:

- HTTP handlers
- loading from a store
- appending facts
- persisting results
- calling external systems

Do not invent pure files when the feature has no meaningful pure logic yet. Do not add ceremony to small features.

## Validation

Keep validation close to the feature.

- boundary-local validation may live in `request.rs`
- extract separate pure validation only when it has enough substance to justify its own file

Do not create a shared validator layer or validation framework.

## Router wiring

Keep route wiring simple.

- Route registration may happen in `routes.rs` or in the local module if that is enough
- Do not create global handler registries or technical dispatch layers

## Tests

Keep tests focused and local.

- Add tests that verify the feature’s actual behavior
- Prefer one relevant happy-path test over broad ceremonial coverage
- Use test infrastructure only when it helps verify the feature clearly

## Examples

### Good direction

/register_location/
- `http_handler.rs`
- `request.rs`
- `append_fact.rs`

/register_location/
- `http_handler.rs`
- `request.rs`
- `build_location_registered_fact.rs`
- `append_fact.rs`

/register_location/
- `http_handler.rs`
- `request.rs`
- `decide_registration.rs`
- `append_fact.rs`

### Bad direction

/register_location/
- `post_location.rs`

/register_location/
- `handler.rs`
- `service.rs`
- `repository.rs`

## When to split a feature into more files

Split when the feature already contains distinct concerns that would otherwise be compressed into one file.

Good reasons to split:

- HTTP boundary and pure logic are both present
- request mapping and append logic are both present
- the file name would otherwise become transport-driven
- the execution flow becomes easier to read with 2–4 focused files

Do not split only to satisfy a rule. Do not keep everything in one file once the feature already has several different responsibilities.

## Response expectations

When using this skill, report briefly:

- which files were added or changed
- how the feature is structured internally
- which files are shell and which are pure, if applicable
- how the feature was verified