# CLAUDE.md

This file provides strict guidance and architectural rules for Claude Code (claude.ai/code) when working in this repository.

## Repository layout

This repo **is** the `hmi-dioxus` library — typed Dioxus wrappers for the
`@ninoverse/hmi-components` web components — with a small demo app alongside it:

- **`.` (workspace root)** — the renderer-agnostic `hmi-dioxus` library crate (`src/lib.rs`, no `main`/`launch`).
- **`assets/vendor/`** — the committed upstream bundle (JS + theme CSS), refreshed by `xtask`.
- **`xtask/`** — `cargo run -p xtask` re-vendors the bundle from npm (maintainer-only).
- **`demo/`** — the `hmi-dioxus-demo` app that consumes the library; it owns `Dioxus.toml`. **Run `dx` from here.**

## Commands & Tooling

- **Toolchain:** You MUST use `cargo` for the Rust crates and the Dioxus CLI (`dx`) for serving, building, and formatting. Never introduce a JavaScript package manager (npm/pnpm/yarn) for app code.
- **Dioxus CLI:** Install once with `cargo install dioxus-cli` (or `cargo binstall dioxus-cli`). Keep the `dx` and `dioxus` crate versions on the same `major.minor`.
- **Platform selection:** Pass `--platform web|desktop|mobile` to `dx serve`/`dx build`, run from `demo/`. `web` is the demo's `default` cargo feature, so plain `cargo` and rust-analyzer target web.
- **Versioning:** Both publishable crates share one repo-owned version via `[workspace.package].version` (the demo inherits it), deliberately **independent of** the wrapped npm package (`@ninoverse/hmi-components` `3.1.2`, pinned only in `xtask`). Don't hand-edit versions — CI bumps them on merge to `main`.
- **Maintain the Build:** Never leave the codebase in a state where the build, clippy, or `dx check` fails. Run the relevant commands below to verify your work before concluding a task.

```bash
cargo fetch                                    # Fetch dependencies (whole workspace)
cd demo && dx serve --platform web             # Demo dev server (hot reload); swap in desktop|mobile
cd demo && dx build --release --platform web   # Production build of the demo
cargo clippy --all-targets -- -D warnings      # Lint the whole workspace (warnings are errors)
cargo test                                     # Run tests (library unit tests)
cargo fmt                                      # Format Rust source (workspace)
dx fmt                                         # Format rsx! macros — run in the crate you edited (root lib or demo/)
cargo run -p xtask                             # Refresh the vendored @ninoverse/hmi-components assets
```

## Architecture & Framework Rules

**Framework:** This project strictly uses **Dioxus 0.7 (Rust)**. The repo ships the `hmi-dioxus` **library** (typed wrappers for the `@ninoverse/hmi-components` web components) at the workspace root, plus a `demo/` app that consumes it and targets web, desktop, and mobile.

- **Components are functions** annotated with `#[component]`, named in `PascalCase`, returning `Element` via the `rsx!` macro. See `.claude/component-workflow.md`.
- **Entry point:** the demo's `demo/src/main.rs` calls `dioxus::launch(App)`; the library crate (repo root, `src/lib.rs`) is renderer-agnostic and has no `main`/`launch`. The active cargo feature selects the demo's renderer — never hardcode a platform.
- **Assets** (CSS, images, fonts) are referenced through the `asset!()` macro so the CLI can fingerprint and bundle them. Inject CSS with `document::Stylesheet { href: ... }`. Library consumers inject the vendored theme + registration script once via `HmiAssets`.
- **No platform-specific code** in shared components unless gated behind `#[cfg(feature = "...")]`.

### Styling Conventions

- **Design Tokens:** CSS custom properties are the single source of truth for theme values (e.g. `var(--color-primary)`). In the demo, global tokens live in `demo/assets/main.css` and per-component CSS in `demo/assets/styling/`; the library ships the vendored hmi-components theme CSS under `assets/vendor/` (injected by `HmiAssets`).
- **Typography & Scaling:** Base font size is `16px`; `rem` units scale from this base. Available font tokens: `var(--font-sans)`, `var(--font-mono)`.
- **No hardcoded values:** colors, radii (`var(--radius-*)`), and shadows (`var(--elevation-*)`) must come from tokens — never literal hex/px in component CSS.

## Behavioral Guidelines

**Tradeoff:** Bias toward caution over speed. For trivial tasks, use judgment.

### 1. Think Before Coding

**Don't assume. Don't hide confusion. Surface tradeoffs.**

- State your assumptions explicitly. If uncertain, stop and ask.
- If multiple interpretations exist, present them - don't pick silently.
- If a simpler approach exists, propose it. Push back when warranted.

### 2. Simplicity First

**Minimum code that solves the problem. Nothing speculative.**

- No features beyond what was asked. No abstractions for single-use code.
- No "flexibility" or error handling for impossible scenarios.
- If you write 200 lines and it could be 50, rewrite it.

### 3. Surgical Changes

**Touch only what you must. Clean up only your own mess.**

- Don't "improve" adjacent code, comments, or formatting.
- Match existing style exactly.
- Remove imports/variables/functions that YOUR changes made unused. Don't remove pre-existing dead code unless asked.

### 4. Goal-Driven Execution

**Define success criteria. Loop until verified.**

- Transform tasks into verifiable goals (e.g., "Add validation" → "Write tests for invalid inputs, then make them pass").
- For multi-step tasks, state a brief plan and verify each step independently.

---

## Extended Rules (Read Before Acting)

Use your file-reading capabilities to read the exact rules in the `.claude/` directory **before** executing any of the following tasks:

- **Committing code:** Read `.claude/commit-conventions.md`
- **Creating branches:** Read `.claude/branch-naming.md`
- **Reviewing PRs:** Read `.claude/code-review.md`
- **Testing/Verifying:** Read `.claude/testing-requirements.md`
- **Opening PRs:** Read `.claude/pr-guidelines.md`
- **Creating new files:** Read `.claude/file-naming.md`
- **Building a component:** Read `.claude/component-workflow.md`
- **Deciding what to build next / branching strategy:** Read `.claude/execution-order.md`
