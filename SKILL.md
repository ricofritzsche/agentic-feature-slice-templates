---
name: feature-slice
description: 
Create or modify a self-contained feature slice using domain-driven naming, 
strict isolation, and explicit IO boundaries. No shared logic, 
no cross-feature dependencies.
---

## When to use

* When creating a new feature
* When modifying an existing feature
* When refactoring into a feature slice

## Non-negotiable rules

### 1. Feature isolation

* Everything lives in `features/<feature-name>/`
* No imports from other features
* No shared folders (`common`, `shared`, `utils`, etc.)
* No global abstractions

Violation → reject and redesign

### 2. No OOP structures

* No classes
* No services, managers, repositories
* No inheritance

Use:

* functions
* plain data

### 3. Functional Core, Imperative Shell

* Core = pure, deterministic decision logic
* Shell = boundaries and effects only (HTTP, DB, external systems)
* The shell may load data and persist results
* The core must not perform IO

Never mix both.

### 4. Explicit IO boundaries

* IO is visible and isolated (e.g. `*.http.ts`, `*.store.ts`)
* No hidden IO inside core

### 5. Contract-first

* Define input/output explicitly (`*.types.ts`)
* Logic follows contracts strictly

### 6. No cross-feature reuse

* Prefer duplication over shared domain abstractions
* No abstractions across features

### 7. No shared domain logic

* Do not move domain-specific rules into shared modules
* Do not extract behavior only because it appears similar
* Shared infrastructure is allowed
* Shared domain behavior is not
### 8. Naming rules (critical)

File names inside a feature must be expressive and specific to the work they do.

Good names describe:
- the boundary
- the artifact
- the transformation
- the response shape
- the query shape

Allowed examples:

* `http_handler.rs`
* `query_parameters.rs`
* `response_types.rs`
* `error_responses.rs`
* `load_context.rs`
* `decide_<...>.rs`
* `append_<...>.rs`

Forbidden:

* `logic.rs`
* `service.rs`
* `manager.rs`
* `repository.rs`
* `util.rs`
* `helper.rs`
* `common.rs`
* `shared.rs`

Rule:

* Avoid generic architecture labels as file names
* Prefer names that make the feature execution or artifact shape visible
* A technical term is acceptable when it is still concrete and specific, such as `http_handler.rs` or `response_builder.rs`
* Do not prefix every file with the feature name when the folder already provides that context

## Required structure

features/<feature-name>/

Required:

* `mod.rs`

Use expressive file names such as:

* `http_handler.rs`
* `query_parameters.rs`
* `response_types.rs`
* `error_responses.rs`
* `load_<context>.rs`
* `decide_<behavior>.rs`
* `append_<result>.rs`
* `<artifact>_response_builder.rs`
* `<feature>_state.rs`

Rules:

* The folder name already carries feature context
* File names inside the folder should describe the concrete role of the file
* Prefer descriptive names over generic architecture labels
* Use as many files as needed, but each file name must explain why it exists

## Implementation steps

1. Define the contracts and response/query shapes
2. Implement the decision logic in an explicit, behavior-specific file
3. Add IO in clearly named boundary files
4. Keep the execution flow readable from the file names inside the feature

## Anti-patterns (reject immediately)

* Cross-feature imports
* Shared modules
* Technical naming (`logic`, `service`, etc.)
* Mixed IO and decision logic
* Reusable abstractions

## Output expectations

* Fully self-contained feature folder
* Domain-driven naming
* Clear IO boundaries
* Deterministic core

## Guiding principle

The feature reads like a story of what happens, not how it is structured.
