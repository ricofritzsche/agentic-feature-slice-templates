
# agentic-feature-slice-templates

Project-level skills and templates for coding agents to generate self-contained feature slices with explicit IO boundaries and no cross-feature dependencies.

Read my article here: [https://medium.com/@rico-fritzsche/functional-core-imperative-shell-for-agentic-coding-45f04beb55f5?sk=7b935b3089c0a97012a40833736beae3]

## Purpose

This repository collects project-level skills and templates that teach coding agents how features are built in a codebase that values local ownership, explicit boundaries, and meaningful naming.

The goal is not to generate more code. The goal is to generate the right shape of code.

That means:

- self-contained feature slices
- explicit IO boundaries
- Functional Core / Imperative Shell
- no shared domain logic
- no cross-feature dependencies
- file names that describe behavior instead of architecture

These templates are meant to be used at the project level. They do not belong inside individual features. They belong alongside the codebase as part of the instruction surface that agents work from.

## What is in this repository

This repository is the canonical home for reusable project-level skills such as:

- feature-slice
- language-specific variants later, such as Rust or TypeScript
- future project templates that encode the same structural rules

A typical setup looks like this:

```text
your-project/
  .codex/
    skills/
      feature-slice/
        SKILL.md
  features/
    register-client/
    evaluate-route-access/
````

The feature is the generated result.
The skill defines the generation environment.

## Why this exists

Most agentic coding problems are not prompt problems first. They are structure problems.

If a codebase still exposes services, managers, repositories, helpers, shared folders, generic file names, and cross-feature abstractions, agents will keep extending exactly those shapes.

This repository exists to provide stronger defaults.

A project-level skill can make those defaults explicit:

* what structures are forbidden
* how a feature is expected to look internally
* where IO is allowed
* how naming works
* how strict feature isolation is
* which dependencies are acceptable and which are not

## Core principles

### 1. Feature isolation

Everything that gives a feature its meaning stays inside that feature.

### 2. No shared domain logic

Shared infrastructure is acceptable. Shared domain-specific behavior is not.

### 3. Functional Core / Imperative Shell

The core stays pure and deterministic.
The shell handles boundaries and effects.

### 4. Explicit IO boundaries

IO must be visible and isolated.

### 5. Contract-first

Inputs and outputs are defined explicitly. Logic follows contracts.

### 6. No cross-feature dependencies

Features do not reach into each other’s internal behavior.

### 7. Naming is architecture

File names must describe behavior, artifacts, or boundaries.
Generic technical labels are avoided.

## Current template

The current base template is the project-level `feature-slice` skill.

Its job is to teach an agent how features are built in a codebase that rejects:

* services
* managers
* repositories as a default pattern
* shared fallback code
* common and shared folders for domain behavior
* cross-feature reuse of domain logic
* generic file names such as `logic.rs`, `service.ts`, `helper.ts`, or `manager.ts`

And instead prefers:

* self-contained feature folders
* behavior-specific file names
* explicit boundaries
* deterministic decision logic
* narrow dependency surfaces

## How to use

Copy the relevant skill into your project, for example:

```text
.codex/skills/feature-slice/SKILL.md
```

Then align the project repository with the same rules:

* feature folders reflect ownership
* file names describe behavior
* shared domain abstractions are removed
* cross-feature imports are treated as exceptions
* generated code is reviewed against those rules

The point is not to rely on the skill alone.
The project repository and the skill need to point in the same direction.

## Guiding principle

A feature should read like a story of what happens, not like a list of architectural categories.


