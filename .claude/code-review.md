# Code Review Guidelines

## What to check

### Styling
- Component styles live in `assets/styling/<name>.css`, not inline `style: "..."` in `rsx!`
- New color values use design tokens (`var(--color-primary)`) rather than hardcoded hex
- Shape via `var(--radius-sm|md|lg)` — never hardcoded radii
- Elevation via `var(--elevation-1|2)` — never hardcoded box-shadows
- Sizing units: `rem` relative to the `16px` base
- CSS is loaded with `document::Stylesheet { href: asset!("...") }`, not a raw `<link>`

### Code quality
- `cargo clippy --all-targets -- -D warnings` passes with no `#[allow(...)]` added just to silence it
- `cargo fmt` and `dx fmt` leave no diff
- No `.unwrap()` / `.expect()` / `panic!` on paths that can fail at runtime — return or handle the error
- Components are `#[component]`, `PascalCase`, return `Element`; one component per file
- No platform-specific code in shared components unless gated behind `#[cfg(feature = "...")]`
- `src/components/mod.rs` `mod` / `pub use` lists stay alphabetical and match files on disk
- No `dbg!`, `println!`, or stray logging left in component render paths
