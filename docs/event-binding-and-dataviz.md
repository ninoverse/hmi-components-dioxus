# Events, value binding & data-viz

> **Status:** the **event-binding** half of this document is a decision record
> for `src/event.rs` and the `HmiInput` / `HmiSwitch` / `HmiCheckbox` wrappers.
>
> **Updated for upstream 4.2.0.** The form controls now expose `onChange`
> (wired onto the inner `<input>`), `defaultValue`/`defaultChecked`, and emit a
> **bubbling `change` `CustomEvent` whose `detail` carries the value**. That
> made the original workarounds unnecessary, so the implementation is now much
> smaller (see §3). The history below — the `detail`-vs-`target` saga (§1) and
> the controlled/uncontrolled fight (§4) — is kept to explain why the pre-4.2.0
> code looked the way it did; **§3 reflects the current code**. The **data-viz**
> half (§5) is still a forward-looking spike.
>
> **Re-verified against upstream 5.0.0** (bundle diff): the event contract is
> byte-for-byte identical for every wrapped element — same registrations, same
> bubbling `change` `CustomEvent` with the value in `detail` — so §3 stands
> as-is. 5.0.0's bridge additionally fixes two host-element gaps: attribute
> *removal* now resets the prop to its type default (the `attributeChangedCallback`
> limitation cited in §3.4 is gone), and properties set on the host *before*
> the registration script runs are re-applied at upgrade. Neither changes the
> wrappers, but the removal fix means a future controlled-attribute binding is
> no longer ruled out by the bridge — only by React ignoring
> `defaultValue`/`defaultChecked` updates (still true) and Dioxus's
> `value`/`checked` property special-case (still true).

## TL;DR

1. **Value binding works, but not the way the spike predicted.** The spike
   assumed every interactive element dispatches a non-bubbling `CustomEvent`
   whose `detail` carries the value. That is true for `chip` (`onSelect` →
   `select`), but **input, switch and checkbox have no callback props** — they
   render a plain inner `<input>` and emit ordinary **native, bubbling**
   `input`/`change` events. The value lives on `event.target`, not in `detail`.
2. **Reading values out** needs a `web-sys` listener attached on the mounted
   host (Dioxus delegation can't see these events), invoked **inside** the
   Dioxus runtime, and followed by an explicit scheduler wake.
3. **Pushing values in** can't go through `rsx!`: Dioxus sends `value`/`checked`
   as DOM *properties* that custom elements ignore, and the upstream turns a
   forwarded `checked`/`value` into a React-*controlled* input with no
   `onChange`. The wrappers sync these imperatively via `web-sys` instead.
4. **Data-viz elements render standalone** (unchanged from the spike).

---

## 1. How the upstream components actually expose behaviour

`@ninoverse/hmi-components` (3.1.2) is a **React→web-component bridge**. Each
element is registered via `tt("name", ReactComponent, { schema })` (minified, in
`assets/vendor/hmi-components.iife.js`), which defines `hmi-<name>` and observes
the schema's attributes. Attribute strings are coerced by type:

| Schema type | Parse (attribute → prop) |
|-------------|--------------------------|
| `string`    | identity |
| `number`    | `parseFloat` |
| `boolean`   | `/^[ty1-9]/i.test(s)` — i.e. `"true"`, `"1"`, `"yes"` are truthy |
| `json`      | `JSON.parse` (so a text value must be JSON-encoded, e.g. `"\"Label\""`) |

**Function-typed props** are turned into DOM events by the bridge: a React
`onXxx` prop becomes a non-bubbling `CustomEvent` named `xxx` whose `detail` is
the callback argument. This is real — it's how `chip`'s `onSelect`/`onClose`
work.

**The catch the spike missed:** input, switch and checkbox declare **no**
function props. Their React components simply spread attributes onto a native
inner element:

```js
// input:    <span class="input"><input class="input__field" {...props}/></span>
// switch:   <label class="switch"><input type="checkbox" {...props}/>…</label>
// checkbox: <label class="checkbox"><input type="checkbox" {...props}/>…</label>
```

In **3.1.2** there was **no `change` CustomEvent and no `detail`** for these
controls: only the inner `<input>`'s native `input`/`change` events, read off
`event.target`. **4.2.0 fixed this** — adding an `onChange` prop means the
bridge dispatches a `change` `CustomEvent` whose `detail` is the value, so all
three controls now match the `chip` model.

### Event reference (4.2.0)

| Element | Event | `detail` |
|---------|-------|----------|
| input | `change` **CustomEvent** (bubbles, per keystroke) | value (`string`) |
| switch / checkbox | `change` **CustomEvent** (bubbles, on toggle) | checked (`bool`) |
| chip | `select` / `close` **CustomEvent** | `bool` / — |

> All four now share one `detail`-decoding path, so the single `on_input_event`
> helper would also serve `chip`'s `select`/`close` when those get wired up.

---

## 2. Dioxus 0.7 constraints (what forced the design)

- **`rsx!` can't name a custom event** (`onselect`, or even `oninput` on a
  custom element won't fire). Dioxus-web's delegated dispatch keys off the
  event target's `data-dioxus-id`; the inner `<input>` is **React-rendered and
  has no Dioxus id**, so a delegated `oninput` on the host never fires. The
  listener must be attached directly on the host element via `web-sys`.
- **Dioxus special-cases `value`/`checked`/`selected` as DOM *properties*.**
  In dioxus-interpreter-js's `setAttributeInner`, `case "checked": node.checked
  = …` / `case "value": node.value = …`. Setting them in `rsx!` writes a JS
  property on the `<hmi-*>` host, which the custom element does **not** observe
  as an attribute — so the value never reaches React. (`disabled`, `name`,
  `placeholder`, `type`, `error`, `label` take the default `setAttribute` path
  and work normally.)
- **Raw `web-sys` callbacks fire outside the Dioxus runtime.** Calling an
  `EventHandler` (or any signal write) from one silently no-ops unless the
  runtime is re-entered — see the panic help in `dioxus_core::Runtime::current`.
- **dioxus-web only flushes renders after its *own* events.** A signal write
  from an external callback queues a `SchedulerMsg` but doesn't get picked up
  until the scheduler is woken explicitly.

---

## 3. The shipped pattern

### 3.1 Optional `web` cargo feature (library)

The library stays renderer-agnostic by default; the `web` feature pulls the
web-only deps. It must still build with the feature **off** (docs.rs,
non-web renderers), so every web-only item is `#[cfg(feature = "web")]` and has
a no-op counterpart.

```toml
# Cargo.toml
[features]
web = ["dep:web-sys", "dep:wasm-bindgen"]

[dependencies]
dioxus = { version = "0.7", default-features = false, features = ["lib"] }
wasm-bindgen = { version = "0.2", optional = true }
web-sys = { version = "0.3", optional = true, features = [
  "Element", "EventTarget", "Event", "CustomEvent",
] }
```

The demo forwards it: `web = ["dioxus/web", "hmi-dioxus/web"]`.

### 3.2 `src/event.rs` — the one interop helper

`on_input_event` is `#[cfg]`-gated with a no-op off-web body so wrapper code is
identical across renderers. It **only reads values out** — the wrappers are
uncontrolled (§3.4), so there is nothing to push in.

- Downcasts the `onmounted` `MountedData` to `web_sys::Element` (the dioxus-web
  backing type) and attaches a listener for the bridge's `change` `CustomEvent`
  **on the host**.
- Wraps the body in `Runtime::wrap_closure` (re-enters the runtime captured at
  mount) and calls `schedule_update()` after `handler.call(value)` to wake the
  render loop — a raw `web-sys` callback fires outside the Dioxus runtime.
- Returns a `ListenerGuard` that removes the listener on drop (no per-mount
  leak); the wrapper stores it in a `use_signal` for the component's lifetime.
- `decode: fn(&EventValue) -> Option<T>` reads `event.detail`, e.g. `|d|
  d.string()` (input) or `|d| d.bool()` (switch/checkbox).

### 3.3 Wrapper shape

```rust
#[component]
pub fn HmiSwitch(
    #[props(default)] checked: bool,
    label: Option<String>,
    #[props(default)] disabled: bool,
    name: Option<String>,
    value: Option<String>,
    #[props(default)] on_change: EventHandler<bool>,
) -> Element {
    let mut listener = use_signal(|| Option::<ListenerGuard>::None);

    rsx! {
        hmi-switch {
            // Seed the initial state; the control is uncontrolled thereafter.
            "default-checked": if checked { "true" } else { "false" },
            "label": label.as_deref().map(json_string), // JSON-encoded text
            "disabled": if disabled { "true" },
            "name": name,
            "value": value,
            onmounted: move |m| {
                listener.set(on_input_event(&m, "change", on_change, |d| d.bool()));
            },
        }
    }
}
```

`HmiInput` is the same shape with `"default-value": if !value.is_empty() { value }`
and `on_input_event(&m, "change", …, |d| d.string())`. Callbacks use
`EventHandler<T>` (Copy, default no-op) so omitting them keeps call sites working.

### 3.4 Uncontrolled binding (uniform across the three)

All three are **uncontrolled**: `value`/`checked` seed the initial state via the
upstream `default-value` / `default-checked` attributes (new in 4.2.0), and edits
are read back out of the `change` `CustomEvent`'s `detail`. The DOM owns the live
value, so the user can type and toggle freely with no React revert and nothing to
push back in.

> **Why not controlled?** Two-way controlled binding doesn't work cleanly here.
> Dioxus' `use_effect` re-runs on *signal* reads inside its closure, not on
> *prop* changes, so an effect that pushes `value`/`checked` back never re-fires
> when the prop updates (verified in-browser: the host attribute stays at its
> mount value). On top of that the bridge's `attributeChangedCallback` ignores
> attribute *removals* and empty values (`(newValue || type==="method")`), so a
> controlled checkbox can't be set back to `false` and a controlled input can't
> be cleared. Uncontrolled sidesteps both — and matches how the pre-4.2.0 code
> actually behaved at runtime.

This implementation was verified end-to-end with headless Chromium
(`.claude/component-workflow.md` screenshot method): typing into the input,
toggling the switch both directions, and checking the box all update their
signal readouts and the live DOM state.

---

## 4. Footguns

Still live:

- **Uncontrolled, not two-way.** `value`/`checked` are *initial* state only;
  changing the prop after mount does **not** update the control (React ignores
  `defaultValue`/`defaultChecked` changes). Drive UI from `on_change` instead.
- **Runtime + scheduler.** A raw `web-sys` callback fires outside the runtime →
  re-enter it (`Runtime::wrap_closure`) and call `schedule_update()`; otherwise
  the handler/signal write no-ops or never re-renders.
- **Hmi\* elements snapshot their slot `innerHTML` at `connectedCallback`** →
  reactive text placed *inside* an `Hmi*` element won't update. Put dynamic
  text in a native element (the demo's readouts do this).
- **`json` attributes are `JSON.parse`d** → encode plain text as a JSON string
  (a shared `json_string` helper does this for labels).
- **Feature hygiene** → CI/locals must build both `cargo build` (feature off)
  and `cargo build --features web` (on), plus the demo.

Retired by 4.2.0 (kept for history): the `detail`-vs-`target` split (all
controls now carry `detail`), the *capture-phase* read to beat React's revert
(an `onChange` means there's nothing to beat), the inner-`checked`-property +
animation-frame retry dance, and the imperative `value`/`checked` push-in
(`ElementHandle`/`set_attr`) — uncontrolled binding needs none of it.

---

## 5. Data-viz: standalone render (spike — not yet implemented)

> This section is unchanged from the original spike; no data-viz wrappers ship
> yet, so treat it as the recommended approach when that group is wrapped.

Charts are ordinary custom elements that take JSON props plus numeric
`width`/`height` (defaults exist, e.g. sparkline `520×260`). They render without
any React context. `hmi-responsive-container` is a **separate, optional** custom
element wrapping a `ResizeObserver` that feeds a measured width to width-aware
charts (bar, line, area, scatter); fixed-size charts (donut, gauge, heatmap) use
absolute size props and need no container.

```rust
// src/components/sparkline.rs — illustrative; pure attributes, no callbacks
#[component]
pub fn HmiSparkline(
    data: String,                      // JSON array, e.g. "[3,7,4,9,5]"
    #[props(default)] width: Option<u32>,
    #[props(default)] height: Option<u32>,
) -> Element {
    rsx! {
        hmi-sparkline {
            "data": data,
            "width": width.map(|w| w.to_string()),
            "height": height.map(|h| h.to_string()),
        }
    }
}
```

The data-viz group can be wrapped with the attribute-only idiom (JSON in via a
string prop, explicit dimensions, optionally nested in an
`HmiResponsiveContainer`). The only open work is confirming each chart's exact
JSON prop/`detail` shapes against the upstream `.d.ts`.

---

## 6. Status & follow-ups

- **Shipped & runtime-verified (headless Chromium):** `web` feature +
  `src/event.rs`; `HmiInput`, `HmiSwitch`, `HmiCheckbox` (initial `value`/
  `checked` + `on_change`, uncontrolled); demo binds all three to signals with
  native readouts.
- **Not done:** `HmiChip` `on_select`/`on_close` — now a one-liner, since
  `on_input_event` already decodes `CustomEvent` `detail` (just pass `"select"`
  / `"close"`); other `change`-emitting controls (textarea, select/combobox,
  slider, radio-group, …) reuse the same `detail`-reading helper; data-viz
  wrappers (§5).
- **Runtime check still pending in CI-less envs:** the 4.2.0 simplification
  (controlled switch/checkbox via the `checked` attribute, `detail` reads)
  compiles and the demo builds; confirm toggles/edits end-to-end with
  `cd demo && dx serve --platform web`.
