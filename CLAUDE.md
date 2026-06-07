# CLAUDE.md

This file provides strict guidance and architectural rules for Claude Code (claude.ai/code) when working in this repository.

## Commands & Tooling

- **Toolchain:** You MUST use `cargo` for the Rust crate and the Dioxus CLI (`dx`) for serving, building, and formatting. Never introduce a JavaScript package manager (npm/pnpm/yarn) for app code.
- **Dioxus CLI:** Install once with `cargo install dioxus-cli` (or `cargo binstall dioxus-cli`). Keep the `dx` and `dioxus` crate versions on the same `major.minor`.
- **Platform selection:** This is a cross-platform app. Pass `--platform web|desktop|mobile` to `dx serve`/`dx build`. `web` is the `default` cargo feature, so plain `cargo` and rust-analyzer target web.
- **Maintain the Build:** Never leave the codebase in a state where the build, clippy, or `dx check` fails. Run the relevant commands below to verify your work before concluding a task.

```bash
cargo fetch                              # Fetch dependencies
dx serve --platform web                  # Start dev server (hot reload); swap in desktop|mobile
dx build --release --platform web        # Production build
cargo clippy --all-targets -- -D warnings # Lint check (warnings are errors)
dx fmt && cargo fmt                      # Format rsx! macros, then Rust source
cargo test                               # Run tests
```

## Architecture & Framework Rules

**Framework:** This project strictly uses **Dioxus 0.7 (Rust)**, targeting web, desktop, and mobile from a single codebase.

- **Components are functions** annotated with `#[component]`, named in `PascalCase`, returning `Element` via the `rsx!` macro. See `.claude/component-workflow.md`.
- **Entry point:** `src/main.rs` calls `dioxus::launch(App)`. The active cargo feature selects the renderer — never hardcode a platform.
- **Assets** (CSS, images, fonts) are referenced through the `asset!()` macro so the CLI can fingerprint and bundle them. Inject CSS with `document::Stylesheet { href: ... }`.
- **No platform-specific code** in shared components unless gated behind `#[cfg(feature = "...")]`.

### Styling Conventions

- **Design Tokens:** CSS custom properties are the single source of truth for theme values (e.g. `var(--color-primary)`). Global tokens live in `assets/main.css`; per-component CSS lives in `assets/styling/`.
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
