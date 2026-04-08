---
name: factstore-usage
description: Use the factstore library from GitHub correctly in this repository for append-only write flows and minimal projections.
---

## Source of truth

Before implementing or changing code that uses factstore, inspect this repository:

- https://github.com/ricofritzsche/factstore

Use the README and current crate structure there as the source of truth for:
- crate names
- public contract types
- append/query semantics
- current capabilities and limitations

Do not guess the API from memory.

## Intended use in this repository

Use factstore as the write-side store for immutable facts.

- use `factstore` for the shared contract
- use `factstore-sqlite` at runtime
- use `factstore-memory` in tests when useful
- pin git dependencies to a concrete commit SHA
- do not wrap factstore behind a CRUD repository abstraction
- do not model features as mutable CRUD tables first
- keep read models separate from fact storage
- keep factstore usage visible at the feature boundary

## Storage rules

- writes append facts
- facts are immutable
- reads for APIs should come from explicit query paths or projections
- do not introduce generic service, manager, or repository layers around factstore
- do not create speculative abstractions for future stores or transport adapters

## What to verify before implementation

Confirm the current public contract and semantics from the factstore repository before coding, especially:

- contract crate and store crate names
- append-only behavior
- query behavior
- conditional append behavior
- whether a needed capability exists already or must remain outside this repository

## Response expectations

Report briefly:

- which factstore crates were used
- which git commit SHA was pinned
- how factstore was initialized
- how facts are appended in the feature
- how the feature was verified
