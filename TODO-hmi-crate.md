# TODO — Extract HMI components into a publishable cargo crate

Goal: turn the typed HMI component wrappers into a standalone, **public crates.io**
crate (`hmi-dioxus`) that multiple Dioxus apps can consume via `cargo add`.

Context: the HMI components are standard **web components (custom elements)** from
the npm package `@ninoverse/hmi-components` (v3.1.2), vendored at build time by
`build.rs`. Today the app renders them via `dangerous_inner_html` raw strings —
no type safety. The crate replaces that with typed Dioxus wrappers + enums and a
single self-injecting assets component.

---

## Phase 0 — Prerequisites (blocking for a public publish)

- [ ] **Confirm the license of `@ninoverse/hmi-components`.** A public crate
      redistributes the vendored `hmi-components.iife.js` + theme CSS inside the
      published `.crate`. This is only legal if that package's license permits
      redistribution and is compatible with the crate's MIT license.
      **Cannot be verified from the repo — owner must confirm/set it.**
- [ ] Decide the crate name (proposed: `hmi-dioxus`).
- [ ] Pin the supported Dioxus range (currently `0.7`) and document MSRV.

## Phase 1 — Scaffold as a workspace member (de-risk before going public)

Build it inside this repo first, with this app as the first consumer. Lift it
into its own repo only once the API feels right.

- [ ] Create the crate skeleton:
      ```
      hmi-dioxus/
      ├── Cargo.toml                 # version mirrors JS pkg: 3.1.x wraps 3.1.2
      ├── assets/vendor/             # dist files, COMMITTED (no longer gitignored)
      ├── src/
      │   ├── lib.rs                 # re-exports + HmiAssets
      │   ├── models/variant.rs      # shared Variant enum
      │   ├── assets.rs              # HmiAssets: inject all CSS + JS once
      │   └── components/
      │       ├── badge.rs           # HmiBadge
      │       ├── button.rs          # HmiButton
      │       ├── spinner.rs         # HmiSpinner
      │       └── alert.rs           # HmiAlert
      └── xtask/                     # maintainer-only: refresh vendored dist from npm
      ```
- [ ] Add `Variant` enum (`primary | secondary | success | danger | info`) with
      `as_str()`; promote to `models/` since badge/button/alert share it.
- [ ] Implement `HmiAssets` — emits the four theme stylesheets + components CSS +
      the IIFE script via `asset!()`, so consumers add it **once** at app root.
- [ ] Implement the four typed wrappers, each rendering its native custom element
      (Dioxus 0.7 `rsx!` supports dashed tags, e.g. `hmi-badge { "variant": ..., {children} }`).
      - [ ] `HmiBadge { variant: Variant, children }`
      - [ ] `HmiButton { variant: Variant, children }`
      - [ ] `HmiSpinner { size: SpinnerSize }`  (size enum local to the file)
      - [ ] `HmiAlert { variant: Variant, title: Option<String>, children }`

## Phase 2 — Self-containment (required for public/no-network builds)

- [ ] **Commit** the vendored dist files into the crate (`assets/vendor/`).
      docs.rs and most downstream CI build with **no network**, so the assets
      must ship in the crate — they cannot be fetched at build time.
- [ ] Move the `npm pack` logic out of `build.rs` into a maintainer-only
      `xtask refresh` (runs only when bumping the JS version), so no consumer
      build ever shells out to npm.

## Phase 3 — Switch this app to consume the crate

- [ ] Replace the `dangerous_inner_html` block in `src/main.rs` with `HmiAssets {}`
      + typed wrapper calls.
- [ ] Remove the now-unused per-app HMI `asset!` consts and `build.rs` npm fetch.
- [ ] Verify: `cargo clippy --all-targets -- -D warnings`, `dx check`,
      `dx build --release --platform web`.

## Phase 4 — Publish (only after Phase 0 license is cleared)

- [ ] Move the crate into its **own repo** (this repo stays an app template;
      keep library churn out of app history).
      Note: current GitHub scope is this template repo only — new repo needs
      owner action / added scope.
- [ ] Set crate metadata: `description`, `license`, `repository`, `keywords`,
      `categories`, `readme`.
- [ ] Versioning policy: mirror the JS package (`3.1.x` ⇒ JS `3.1.2`); JS major
      bump ⇒ crate major bump.
- [ ] `cargo publish --dry-run`, verify the `.crate` includes `assets/vendor/`.
- [ ] `cargo publish` (outward-facing, irreversible — confirm before running).

---

## Open decisions

- [ ] Crate name confirmed?
- [ ] Enum (closed set, safest) vs. validated newtype for `variant` if the design
      system adds variants frequently. Default: enum.
- [ ] Next step now: scaffold the workspace crate, or write the crate
      `README` / API design doc first for review?
