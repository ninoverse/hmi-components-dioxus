# TODO — publish `hmi-dioxus` to crates.io

**Goal:** ship `hmi-dioxus` — typed Dioxus 0.7 wrappers for the
`@ninoverse/hmi-components` web components — as a public crate consumable via
`cargo add hmi-dioxus`. The crate is the workspace root of this repo; the `demo/`
app (`publish = false`) is its first consumer and stays in-repo.

**Where things stand:** the wrappers were cleared to re-wrap against a new
upstream version — **0 wrappers ship** right now. The infrastructure remains:
the self-injecting `HmiAssets`, the optional `web` feature with its DOM
event-binding interop (`src/event.rs`), and the `json_string` attribute helper.
The license + publish-metadata prerequisites are done; what's left is the
wrapping itself and the eventual `cargo publish`.

---

## Component surface

Re-vendor the new upstream with `cargo run -p xtask`, then re-extract the list
of registered custom elements (the bundle defines `hmi-<name>` through a single
`customElements.define` call inside the `tt("name", …)` helper) and refresh the
inventory below before starting to wrap.

> Caveat: chart/data-heavy elements (and some inputs) likely need JSON/data
> props, or a surrounding context, to render usefully — so wrap the simple
> presentational ones first and confirm each renders standalone before wrapping
> it. Some elements may also turn out **structurally unwrappable** (e.g. slotted
> triggers wired with `React.cloneElement`, or `value`/`onChange` trafficking in
> native JS objects); see `.claude/skills/triage-unwrappable-component` for the
> protocol — mark those ⛔ with a fresh analysis doc under `docs/`.

Wrapping conventions: complex props are passed as JSON-encoded strings (see the
`json_string` helper in `src/components/mod.rs`); upstream
`onSelect`/`onClose`-style callbacks aren't expressible as attributes — attach
standard DOM event handlers to the rendered element instead, or use the `web`
feature interop in `src/event.rs` for `change`-style events.

Inventory (**0 wrapped** — repopulate after re-vendoring the new upstream):

_The previous version's component list was cleared. Run `cargo run -p xtask`,
extract the registered element names, and list them here grouped by kind
(presentational / inputs / containers / data-viz) to drive wrapping order._

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
