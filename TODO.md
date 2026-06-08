# TODO — publish `hmi-dioxus` to crates.io

**Goal:** ship `hmi-dioxus` — typed Dioxus 0.7 wrappers for the
`@ninoverse/hmi-components` web components — as a public crate consumable via
`cargo add hmi-dioxus`. The crate is the workspace root of this repo; the `demo/`
app (`publish = false`) is its first consumer and stays in-repo.

**Where things stand:** the crate is built, wired, and verified locally. Three
wrappers ship (`HmiBadge`, `HmiButton`, `HmiChip`) plus the self-injecting
`HmiAssets`. What's left is breadth (more wrappers) and the publish prerequisites
— chiefly an upstream license confirmation.

---

## Done

- [x] **Scaffolded** the crate (`src/lib.rs`, `src/assets.rs`, `src/components/`),
      versioned independently of the npm package.
- [x] **Self-contained**: the upstream bundle (`hmi-components.iife.js` + the four
      theme CSS files) is **committed** under `assets/vendor/`, so docs.rs and
      offline CI build with no network/npm. Refresh with `cargo run -p xtask`
      (the old `build.rs` `npm pack` path is gone).
- [x] **`HmiAssets`** injects the four theme stylesheets + components CSS + the
      IIFE registration script via `asset!()` — render once at app root.
- [x] **Typed wrappers**, with per-component enums (no shared `Variant` — value
      sets differ per element, mirroring the upstream `.d.ts` type aliases):
  - [x] `HmiBadge` — `BadgeVariant`, `dot`
  - [x] `HmiButton` — `ButtonVariant` / `ButtonSize` / `ButtonType`, `disabled`,
        `as_icon`, JSON `left_icon` / `right_icon`
  - [x] `HmiChip` — `selected`, JSON `icon`
- [x] **Demo consumes the crate**: `demo/src/main.rs` uses `HmiAssets` + the typed
      wrappers (no more `dangerous_inner_html`). Verified with `cargo test`,
      `cargo clippy --workspace --all-targets -- -D warnings`, `dx check`, and
      `dx build --release --platform web`.
- [x] **Repo restructured around the library**: `hmi-dioxus` at the workspace
      root, demo moved to `demo/`. Both crates share one repo-owned version via
      `[workspace.package].version`, deliberately decoupled from npm `3.1.2`
      (which is pinned only in `xtask`).

---

## Component surface (verified against vendored 3.1.2)

The bundle registers **85 custom elements** through a single helper
`tt("name", …)` that defines `hmi-<name>` — `customElements.define` is called in
exactly one place (inside `tt`), and `tt` is invoked for 85 distinct names.

> An earlier draft here split these into "54 web components / 31 React-only"; that
> does **not** hold for 3.1.2 — every name below is registered as a custom element.
> Caveat: chart/data-heavy elements (and some inputs) likely need JSON/data props,
> or a surrounding context, to render usefully — so wrap the simple presentational
> ones first and confirm each renders standalone before wrapping it.

Wrapping conventions (from the existing wrappers): complex props are passed as
JSON-encoded strings; upstream `onSelect`/`onClose`-style callbacks aren't
expressible as attributes — attach standard DOM event handlers to the rendered
element instead. Re-vendor with `cargo run -p xtask` and re-extract this list when
bumping the upstream version.

Rough grouping, to suggest order (**3 / 85 wrapped**):

**Presentational / leaf — wrap first**
- [x] badge · [x] button · [x] chip
- [ ] alert · [ ] avatar · [ ] banner · [ ] blockquote · [ ] box · [ ] breadcrumbs
- [ ] card · [ ] code · [ ] divider · [ ] flex · [ ] grid · [ ] heading · [ ] image
- [ ] kbd · [ ] link · [ ] list · [ ] meter · [ ] progress · [ ] skeleton · [ ] spacer
- [ ] spinner · [ ] stat · [ ] text · [ ] tooltip

**Inputs / form (verify standalone; need value/event plumbing)**
- [ ] checkbox · [ ] combobox · [ ] input · [ ] radio · [ ] select · [ ] slider
- [ ] stepper · [ ] switch · [ ] tabs · [ ] textarea · [ ] form-control
- [ ] color-picker · [ ] date-picker · [ ] file-upload · [ ] multi-input
- [ ] number-input · [ ] password-input · [ ] radio-group · [ ] search-input
- [ ] segmented-control · [ ] value-scale-selector

**Containers / overlays / navigation / structure**
- [ ] accordion · [ ] aspect-ratio · [ ] avatar-stack · [ ] carousel · [ ] drawer
- [ ] menu · [ ] modal · [ ] navbar · [ ] pagination · [ ] popover · [ ] scroll-area
- [ ] sidebar · [ ] table · [ ] timeline · [ ] toast · [ ] tree · [ ] visually-hidden
- [ ] command-palette · [ ] confirm-dialog · [ ] context-menu · [ ] hover-card
- [ ] empty-state

**Data viz (need data props — lower priority)**
- [ ] gauge · [ ] heatmap · [ ] legend · [ ] sparkline · [ ] area-chart · [ ] bar-chart
- [ ] bullet-chart · [ ] cartesian-grid · [ ] chart-tooltip · [ ] donut-chart
- [ ] funnel-chart · [ ] line-chart · [ ] radar-chart · [ ] responsive-container
- [ ] scatter-plot

---

## Before publishing — Phase 0 (blocking)

- [ ] **Confirm the upstream license.** A public crate redistributes
      `hmi-components.iife.js` + the theme CSS inside the published `.crate`; this
      is only legal if `@ninoverse/hmi-components`'s license permits redistribution
      and is compatible with this crate's MIT. **Owner must confirm/set it — not
      verifiable from the repo.**
- [ ] Document the supported Dioxus range (`0.7`) and the MSRV.
- [ ] Fill publish metadata in the root `Cargo.toml`: `repository`, `keywords`,
      `categories`, `readme` (and re-confirm `license`, `description`).

## Publish

- [ ] `cargo publish -p hmi-dioxus --dry-run`; confirm the packaged `.crate`
      includes `assets/vendor/` and excludes `demo/`.
- [ ] `cargo publish -p hmi-dioxus` (outward-facing, irreversible — confirm first).

---

## Open questions

- [ ] An ergonomic pattern for component **event callbacks** (`onSelect`, `onClose`,
      …) and two-way value binding before wrapping the interactive set en masse —
      current wrappers omit callbacks and lean on raw DOM handlers.
- [ ] Do the **data-viz** elements render acceptably as standalone custom elements,
      or do they need a React / `responsive-container` context? Spike one before
      committing to wrap the group.
- [ ] Once published, does the `demo/` stay in this repo (current plan) or move to
      a separate example, to keep library release history clean?
