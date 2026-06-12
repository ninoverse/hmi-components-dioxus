# hmi-dioxus

Typed [Dioxus 0.7](https://dioxuslabs.com) wrappers for the
[`@ninoverse/hmi-components`](https://www.npmjs.com/package/@ninoverse/hmi-components)
web components (custom elements), usable from web, desktop, and mobile.

The crate vendors the upstream component bundle (JS + theme CSS) and exposes it as
ordinary Dioxus components with typed props/enums, plus a single self-injecting
`HmiAssets` component. No JavaScript toolchain is needed to consume it.

## Layout

```
.                    # the `hmi-dioxus` library crate (workspace root)
├── src/             # lib.rs, assets.rs, components/ (one wrapper per file)
├── assets/vendor/   # committed @ninoverse/hmi-components bundle (JS + theme CSS)
├── xtask/           # `cargo run -p xtask` re-vendors the bundle
└── demo/            # `hmi-dioxus-demo`: a small app that consumes the library
```

## Using the library

Add the crate (path/git for now — not yet on crates.io), render `HmiAssets` once
near the root of your app, then use the `Hmi*` wrappers anywhere in the tree:

```rust
use dioxus::prelude::*;
use hmi_dioxus::HmiAssets;

#[component]
fn App() -> Element {
    rsx! {
        HmiAssets {}   // inject styles + register elements (once)
        // typed `Hmi*` wrappers go here
    }
}
```

> **Status:** the typed wrappers are being re-built against a new upstream
> version, so none ship yet — only the self-injecting `HmiAssets`. See `TODO.md`
> for the wrapping scope and the path to a crates.io release.

## Running the demo

```bash
cargo install dioxus-cli            # one-time; keep dx's major.minor matching the dioxus crate
cd demo
dx serve --platform web             # hot-reload dev server (try desktop|mobile too)
```

`web` is the demo's default cargo feature, so plain `cargo build` and rust-analyzer
target web.

## Verifying

```bash
cargo clippy --all-targets -- -D warnings    # lint the workspace (warnings are errors)
cargo test                                    # library unit tests
cd demo && dx check                           # validate rsx! macros
cd demo && dx build --release --platform web  # production build of the demo
cargo fmt                                     # format Rust; `dx fmt` formats rsx! per-crate
```

## Compatibility

- **Dioxus:** `0.7.x` (the crate depends on `dioxus` with `default-features = false,
  features = ["lib"]`, so the consuming binary brings its own renderer).
- **MSRV:** Rust `1.83`, matching the Dioxus 0.7 toolchain floor.

## Versioning

Both publishable crates share one repo-owned version via `[workspace.package].version`
(the demo inherits it), kept **independent of** the wrapped npm package. The
`@ninoverse/hmi-components` `5.0.1` pin lives only in `xtask`, not in the crate
version. CI bumps the shared version on merge to `main`.

## Conventions

- **Components** are `PascalCase` `#[component]` functions returning `Element` via
  `rsx!`, one per `snake_case.rs` file under a crate's `src/components/`.
- **Styling** uses CSS custom-property design tokens (`var(--color-primary)`),
  loaded via `asset!()` + `document::Stylesheet`; no hardcoded colors/radii/shadows.
- **Cross-platform**: no platform-specific code in shared components unless gated
  behind `#[cfg(feature = "...")]`.

See `CLAUDE.md` and `.claude/` for the full architectural rules.

## License

`hmi-dioxus` is licensed under the [MIT License](LICENSE).

The crate vendors and redistributes the upstream `@ninoverse/hmi-components` bundle
(JS + theme CSS) under `assets/vendor/`. That bundle is also MIT-licensed; see
[`THIRD-PARTY-NOTICES.md`](THIRD-PARTY-NOTICES.md) for the attribution and full
license text.
