# Rust + Dioxus Template

A starter template for cross-platform [Dioxus 0.7](https://dioxuslabs.com) apps
(web, desktop, mobile) wired up with Claude Code configuration and architectural
rules. Clone it, rename the crate, and start building components.

## What's inside

```
.
├── Cargo.toml            # crate + per-platform feature flags (web/desktop/mobile)
├── Dioxus.toml           # Dioxus app config
├── src/
│   ├── main.rs           # dioxus::launch(App) entry point
│   └── components/
│       ├── mod.rs        # re-exports each component (alphabetical)
│       └── hero.rs       # example #[component]
├── assets/
│   ├── main.css          # design tokens (:root custom properties) + reset
│   └── styling/
│       └── hero.css      # per-component CSS
├── CLAUDE.md             # architectural rules Claude Code must follow
└── .claude/              # detailed rule files (see table below)
```

## Getting started

```bash
# One-time: install the Dioxus CLI (keep its major.minor matching the dioxus crate)
cargo install dioxus-cli      # or: cargo binstall dioxus-cli

cargo fetch                   # fetch dependencies
dx serve --platform web       # dev server with hot reload (try desktop|mobile too)
```

`web` is the default cargo feature, so plain `cargo build` and rust-analyzer
target web. Select another renderer with `--platform desktop|mobile`.

### Verify your work

```bash
cargo clippy --all-targets -- -D warnings   # lint (warnings are errors)
dx check                                     # validate rsx! macros
cargo test                                   # run tests
dx build --release --platform web            # production build
dx fmt && cargo fmt                          # format rsx! then Rust source
```

## Conventions at a glance

- **Components** are `PascalCase` functions annotated with `#[component]`,
  returning `Element` via `rsx!`, one per `snake_case.rs` file under
  `src/components/`.
- **Styling** uses CSS custom-property design tokens (`var(--color-primary)`),
  loaded via `asset!()` + `document::Stylesheet`. No hardcoded colors, radii,
  or shadows.
- **Cross-platform**: no platform-specific code in shared components unless
  gated behind `#[cfg(feature = "...")]`.

## Rule files

| File | Purpose |
|------|---------|
| `CLAUDE.md` | Top-level commands, framework rules, styling conventions |
| `.claude/branch-naming.md` | Branch prefix and format conventions |
| `.claude/commit-conventions.md` | Conventional Commits rules |
| `.claude/pr-guidelines.md` | PR title, description template, size guidance |
| `.claude/testing-requirements.md` | Test + verification gates |
| `.claude/file-naming.md` | Directory layout and Rust/Dioxus naming conventions |
| `.claude/code-review.md` | Review checklist (styling + Rust code quality) |
| `.claude/component-workflow.md` | Step-by-step procedure to add a component |
| `.claude/execution-order.md` | Branching strategy and (optional) phased build order |

## Renaming the crate

Change `name` in `Cargo.toml` and `[application].name` in `Dioxus.toml` to your
project name, then update the `title` under `[web.app]` in `Dioxus.toml`.
