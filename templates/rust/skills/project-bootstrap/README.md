# README — start a new Rust project with a bootstrap skill

## Goal

Set up a new Rust repo so Codex can use a **foundation skill** to scaffold the first working project skeleton in a consistent way.

This setup keeps two things separate:

* **`AGENTS.md`** for general repo rules
* **`SKILL.md`** for one reusable workflow: Rust project bootstrap ([OpenAI Developers][2])

## 1. Create the repository

```bash
mkdir my-rust-service
cd my-rust-service
git init
```

## 2. Add repo-wide instructions

Create this file in the **repository root**:

```text
AGENTS.md
```

Use it for broad rules that should apply across the repo, for example:

* keep structure minimal
* avoid speculative abstractions
* prefer feature-local code
* run tests after changes

Codex discovers `AGENTS.md` from the repo root and can also use nested overrides closer to specialized work. ([OpenAI Developers][3])

## 3. Add the bootstrap skill in the correct place

Create this folder in the **repository root**:

```text
.agents/skills/rust-project-bootstrap/
```

Then place your skill file here:

```text
.agents/skills/rust-project-bootstrap/SKILL.md
```

That is the repo-scoped location officially documented for skills. Use this when the skill belongs to the project and should travel with the repo. ([OpenAI Developers][1])

### Minimal example tree

```text
my-rust-service/
├── .agents/
│   └── skills/
│       └── rust-project-bootstrap/
│           └── SKILL.md
└── AGENTS.md
```

## 4. Put the skill content into `SKILL.md`

A Codex skill is a directory anchored by a required `SKILL.md` manifest. The file should include YAML front matter with at least `name` and `description`, followed by the Markdown instructions. Optional `scripts/`, `references/`, and `assets/` folders can be added later if the workflow truly needs them. ([OpenAI Developers][4])

Example header:

```md
---
name: rust-project-bootstrap
description: Create or review the first working skeleton of a new Rust service, CLI, or application.
---
```

Then place the rest of your bootstrap instructions below that.

## 5. Start Codex from the repo root

Launch Codex **from the repository root**, not from some random parent directory. Repo instructions and repo skills are discovered relative to where you start work in the repo hierarchy. Codex also supports nested scopes and parent-folder discovery, so location matters. ([OpenAI Developers][1])

## 6. Trigger the skill explicitly the first times

Early on, do not rely only on implicit triggering. Official guidance recommends manually triggering a new skill first to surface hidden assumptions. You can explicitly reference a skill with the `$` prefix, and Codex can also choose skills implicitly when the task matches the description. ([OpenAI Developers][5])

Example prompt:

```text
Use $rust-project-bootstrap to scaffold the first working Rust HTTP service in this repo.
Keep it minimal. Use Cargo defaults. Add explicit config, startup logging, /health, and one smoke test.
```

## 7. Let the skill create only the bootstrap, not fake architecture

For the first pass, the repo should usually end up close to this:

```text
my-rust-service/
├── .agents/
│   └── skills/
│       └── rust-project-bootstrap/
│           └── SKILL.md
├── AGENTS.md
├── Cargo.toml
├── src/
│   └── main.rs
├── tests/
│   └── smoke.rs
├── .env.example
└── README.md
```

Keep the bootstrap small:

* `cargo run` works
* config is explicit
* logs are visible
* one test runs
* `/health` exists if it is a service

## 8. Verify the bootstrap locally

The first runnable path should be obvious and documented. For a Rust project, the practical baseline is:

```bash
cargo check
cargo test
cargo run
```

If you want stricter validation from day one:

```bash
cargo fmt --check
cargo clippy -- -D warnings
```

## 9. Add feature skills later, not now

This bootstrap skill belongs at repo level because it applies to the **initial project foundation**. Later, when you have real feature slices, add more targeted skills for repeatable feature work. Skills are meant for repeatable workflows and team-specific procedures, not for stuffing all rules into one giant file. ([OpenAI Developers][2])

## 10. Use global skills only for personal defaults

If you want the same bootstrap skill available in every repo, put it in:

```text
$HOME/.agents/skills/rust-project-bootstrap/SKILL.md
```

Use that for personal reusable workflows. Use repo-scoped `.agents/skills` when the skill should be versioned with the project. ([OpenAI Developers][1])

## Important note about older examples

Some official examples and blog posts still show `.codex/skills/...` as a place to paste a skill during early iteration, but the current Codex docs for skills and customization emphasize `.agents/skills` for repo-scoped and user-scoped skill placement. For a new project, `.agents/skills` is the cleaner current choice. ([OpenAI Developers][5])

## Recommended final starting point

Use this setup:

```text
my-rust-service/
├── AGENTS.md
└── .agents/
    └── skills/
        └── rust-project-bootstrap/
            └── SKILL.md
```

Then ask Codex to use the skill to scaffold the first Rust service skeleton.

The next useful step is to turn this into a **real repo README file** plus a **matching example `AGENTS.md`** so you can copy both into every new Rust repo.

[1]: https://developers.openai.com/codex/skills/ "Agent Skills – Codex | OpenAI Developers"
[2]: https://developers.openai.com/codex/concepts/customization/ "Customization – Codex | OpenAI Developers"
[3]: https://developers.openai.com/codex/guides/agents-md/ "Custom instructions with AGENTS.md – Codex | OpenAI Developers"
[4]: https://developers.openai.com/cookbook/examples/skills_in_api/?utm_source=chatgpt.com "Skills in OpenAI API"
[5]: https://developers.openai.com/blog/eval-skills/ "Testing Agent Skills Systematically with Evals | OpenAI Developers"

