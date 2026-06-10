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
feature that powers DOM event binding. The license + publish-metadata
prerequisites are done; what's left is breadth (more wrappers) and the actual
`cargo publish`.

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

Rough grouping, to suggest order (**13 / 85 wrapped**):

**Presentational / leaf — wrap first**
- [x] badge · [x] button · [x] chip
- [ ] alert · [x] avatar · [ ] banner · [ ] blockquote · [ ] box · [ ] breadcrumbs
- [x] card · [ ] code · [x] divider · [ ] flex · [ ] grid · [x] heading · [ ] image
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

## Publish

- [ ] `cargo publish -p hmi-dioxus --dry-run`; confirm the packaged `.crate`
      includes `assets/vendor/` and excludes `demo/`.
- [ ] `cargo publish -p hmi-dioxus` (outward-facing, irreversible — confirm first).

---

## Future / low priority

- [ ] Publish the crate to a Cargo repository on **Google Cloud Artifact
      Registry** as an additional/private distribution channel alongside
      crates.io (configure the registry in `.cargo/config.toml`, authenticate
      with the GCP credential helper, then `cargo publish --registry <name>`).
