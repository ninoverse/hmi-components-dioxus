# TODO — publish `hmi-dioxus` to crates.io

**Goal:** ship `hmi-dioxus` — typed Dioxus 0.7 wrappers for the
`@ninoverse/hmi-components` web components — as a public crate consumable via
`cargo add hmi-dioxus`. The crate is the workspace root of this repo; the `demo/`
app (`publish = false`) is its first consumer and stays in-repo.

**Where things stand:** the crate is built, wired, and verified locally.
Thirteen wrappers ship — `HmiBadge`/`HmiButton`/`HmiChip`, seven more
presentational (`HmiCard`, `HmiDivider`, `HmiHeading`, `HmiText`, `HmiAvatar`,
`HmiSpinner`, `HmiProgress`), and three interactive (`HmiInput`, `HmiSwitch`,
`HmiCheckbox`) — plus the self-injecting `HmiAssets` and an optional `web`
feature that powers DOM event binding. What's left is breadth (more wrappers)
and the actual `cargo publish` (the license + metadata prerequisites are done).

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
  - [x] `HmiCard` (`CardVariant`) · `HmiDivider` (`DividerOrientation`/`DividerAlign`)
        · `HmiHeading` (`level`, `HeadingSize`/`HeadingTone`, `truncate`)
        · `HmiText` (`tag`, `TextSize`/`TextWeight`/`TextTone`/`TextAlign`, `truncate`)
        · `HmiAvatar` (`AvatarSize`/`AvatarStatus`) · `HmiSpinner` (`SpinnerSize`)
        · `HmiProgress` (`value`, `indeterminate`)
  - [x] **Interactive** (behind the optional `web` feature, events wired via
        `src/event.rs`): `HmiInput` (two-way `value` + `on_change`), `HmiSwitch`
        and `HmiCheckbox` (`checked` + `on_change`)
- [x] **Demo consumes the crate**: `demo/src/main.rs` uses `HmiAssets` + the typed
      wrappers (no more `dangerous_inner_html`). Verified with `cargo test`,
      `cargo clippy --workspace --all-targets -- -D warnings`, `dx check`, and
      `dx build --release --platform web`.
- [x] **Repo restructured around the library**: `hmi-dioxus` at the workspace
      root, demo moved to `demo/`. Both crates share one repo-owned version via
      `[workspace.package].version`, deliberately decoupled from npm `5.0.1`
      (which is pinned only in `xtask`).

---

## Component surface (verified against vendored 4.2.0; unchanged through 5.0.1)

The bundle registers **85 custom elements** through a single helper
`tt("name", …)` that defines `hmi-<name>` — `customElements.define` is called in
exactly one place (inside `tt`), and `tt` is invoked for 85 distinct names.

> An earlier draft here split these into "54 web components / 31 React-only"; that
> does **not** hold for 4.2.0 — every name below is registered as a custom element.
> Caveat: chart/data-heavy elements (and some inputs) likely need JSON/data props,
> or a surrounding context, to render usefully — so wrap the simple presentational
> ones first and confirm each renders standalone before wrapping it.

Wrapping conventions (from the existing wrappers): complex props are passed as
JSON-encoded strings; upstream `onSelect`/`onClose`-style callbacks aren't
expressible as attributes — attach standard DOM event handlers to the rendered
element instead. Re-vendor with `cargo run -p xtask` and re-extract this list when
bumping the upstream version.

Rough grouping, to suggest order (**14 / 85 wrapped**):

**Presentational / leaf — wrap first**
- [x] badge · [x] button · [x] chip
- [ ] alert · [x] avatar · [ ] banner · [ ] blockquote · [ ] box · [ ] breadcrumbs
- [x] card · [x] code · [x] divider · [ ] flex · [ ] grid · [x] heading · [ ] image
- [ ] kbd · [ ] link · [ ] list · [ ] meter · [x] progress · [ ] skeleton · [ ] spacer
- [x] spinner · [ ] stat · [x] text · [ ] tooltip

**Inputs / form (verify standalone; need value/event plumbing)**
- [x] checkbox · [ ] combobox · [x] input · [ ] radio · [ ] select · [ ] slider
- [ ] stepper · [x] switch · [ ] tabs · [ ] textarea · [ ] form-control
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

- [x] **Confirm the upstream license.** `@ninoverse/hmi-components` is **MIT**
      (same author/copyright as this crate: "Copyright (c) 2026 Nicola"), so
      redistributing `hmi-components.iife.js` + the theme CSS inside the published
      `.crate` is permitted and MIT-compatible. Attribution + full upstream license
      text recorded in `THIRD-PARTY-NOTICES.md` and linked from the README.
- [x] Document the supported Dioxus range (`0.7`) and the MSRV (`1.83`, the Dioxus
      0.7 toolchain floor) — `rust-version` in `Cargo.toml` + a Compatibility
      section in the README.
- [x] Fill publish metadata in the root `Cargo.toml`: `repository`, `documentation`,
      `keywords`, `categories`, `readme`, and an `exclude` list that drops dev-only
      / sibling-crate paths (`license` + `description` were already set).

## Publish

- [ ] `cargo publish -p hmi-dioxus --dry-run`; confirm the packaged `.crate`
      includes `assets/vendor/` and excludes `demo/`.
- [ ] `cargo publish -p hmi-dioxus` (outward-facing, irreversible — confirm first).

---

## Open questions

- [x] **Event callbacks & value binding** — *implemented for
      input/switch/checkbox; see
      [`docs/event-binding-and-dataviz.md`](docs/event-binding-and-dataviz.md).*
      As of upstream `4.2.0` the form controls expose `onChange`, which the
      bridge turns into a bubbling `change` `CustomEvent` carrying the value in
      `detail`, plus `defaultValue`/`defaultChecked`. The wrappers render
      **uncontrolled** — seeding the initial state via `default-value`/
      `default-checked` and reading edits from `detail` via a `web-sys` listener
      (re-entering the Dioxus runtime and waking the scheduler). The `web`
      feature stays only for that read (proposal doc §6). Verified end-to-end in
      headless Chromium. Callbacks are `EventHandler<T>`.
- [x] **Data-viz standalone render** — *answered (see the same doc).* Charts are
      plain custom elements taking JSON props + explicit `width`/`height`; they need
      no React context. `hmi-responsive-container` is an optional ResizeObserver
      wrapper for fluid width only. The group can be wrapped with the existing
      attribute-only idiom.
- [ ] Once published, does the `demo/` stay in this repo (current plan) or move to
      a separate example, to keep library release history clean?
