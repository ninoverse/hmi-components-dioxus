# Directories and File Naming

## Source directories (`src/`)

> Layout note: the workspace root `src/` is the **library** crate (`src/lib.rs`, no
> `main`). The demo **app** lives in `demo/src/` (`demo/src/main.rs`). The
> conventions below apply within each crate's own `src/`.

| Path | Contents |
|------|----------|
| `src/main.rs` | App entry point: `dioxus::launch(App)` and the root `App` component |
| `src/components/` | Dioxus components (one component per file) |
| `src/components/mod.rs` | Module that re-exports each component (`pub use`) |
| `src/models/` | Shared structs/enums and `Props` types used by 2+ components |
| `src/lib.rs` | Optional: shared library surface if the crate is split into lib + bin |

## Asset directories (`assets/`)

| Path | Contents |
|------|----------|
| `assets/main.css` | Global design tokens (`:root` custom properties) + reset |
| `assets/styling/` | Per-component CSS (`<name>.css`) |
| `assets/` | Images, fonts, icons referenced via `asset!()` |

## File naming conventions

| Item | Convention | Example |
|------|-----------|---------|
| Component file | `snake_case.rs` | `unordered_list.rs` |
| Component identifier | `PascalCase` | `UnorderedList` |
| Component CSS | `snake_case.css` in `assets/styling/` | `unordered_list.css` |
| Module file | `mod.rs` inside the folder | `src/components/mod.rs` |
| Shared types | `snake_case.rs` in `src/models/` | `src/models/user.rs` |
| Functions / variables | `snake_case` | `format_date` |
| Constants / statics | `SCREAMING_SNAKE_CASE` | `const MAIN_CSS: Asset` |

These follow standard Rust naming (`rustfmt` + clippy enforce most of them). The
only project-specific rule is the one-component-per-file layout under
`src/components/`.

## Module re-export pattern

Rust has no barrel files. Instead, each folder's `mod.rs` declares its
submodules and re-exports the public component(s). Keep `pub use` lines
**alphabetical**:

```rust
// src/components/mod.rs
mod hero;
mod unordered_list;

pub use hero::Hero;
pub use unordered_list::UnorderedList;
```

The parent (`src/main.rs`) then writes `mod components;` and
`use components::Hero;`.

## CSS reference pattern

Each component declares its CSS as an `Asset` const and injects it at the top of
its `rsx!` via `document::Stylesheet`. The `asset!()` path is absolute from the
crate root:

```rust
const HERO_CSS: Asset = asset!("/assets/styling/hero.css");
// ...
rsx! {
    document::Stylesheet { href: HERO_CSS }
    // markup
}
```

Component CSS uses only design tokens (`var(--color-primary)`, …) — no hardcoded
color, radius, or shadow values.
