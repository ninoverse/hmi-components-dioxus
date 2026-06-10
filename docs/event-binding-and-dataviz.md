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
> bubbling `change` `CustomEvent` with the value in `detail`. 5.0.0's bridge
> additionally fixes the two host-element gaps that `controlled-form-controls.md`
> required: empty/`false` updates and attribute *removal* now apply (the
> `attributeChangedCallback` limitation cited in §3.4's history is gone), and
> properties set on the host *before* the registration script runs are
> re-applied at upgrade. Those fixes made Dioxus's `value`/`checked`
> property writes land, so the wrappers were **migrated to controlled
> binding** — §3.4 describes the current model, with the old uncontrolled
> rationale kept as history.
>
> **Upgraded to 5.0.1** (now vendored): a patch that fixes the **mid-string
> caret jump** the controlled binding exposed (the old §4 footgun). The React
> text components now capture the inner element's selection in their `onChange`
> (guarded against IME composition via `isComposing`) and restore it in a
> `useLayoutEffect` keyed on the `value` prop — but only while the element is
> focused, the new value matches the user's edit, and the type is
> selection-capable (`text`/`search`/`password`/`tel`/`url` or `textarea`).
> Because the fix lives entirely in the upstream React layer and rides the
> existing echo path, **no wrapper code changed**; re-running the headless
> caret measurement against the demo confirms mid-string edits now keep their
> caret (see §6). The event contract and registrations are still identical.

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
identical across renderers. It **only reads values out** — pushing values in
needs no interop at all, because Dioxus's `value`/`checked` property writes
reach React through the 5.0.0 host (§3.4).

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
    /// Controlled state: the switch always shows exactly this value.
    checked: Option<bool>,
    label: Option<String>,
    #[props(default)] disabled: bool,
    name: Option<String>,
    value: Option<String>,
    #[props(default)] on_change: EventHandler<bool>,
) -> Element {
    let mut listener = use_signal(|| Option::<ListenerGuard>::None);

    rsx! {
        hmi-switch {
            // Dioxus writes `checked` as a DOM *property*; the 5.0.0 host
            // forwards it to the React prop, even pre-upgrade.
            "checked": checked.map(|c| if c { "true" } else { "false" }),
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

`HmiInput` is the same shape with `"value": value` (`Option<String>`) and
`on_input_event(&m, "change", …, |d| d.string())`. Callbacks use
`EventHandler<T>` (Copy, default no-op) so omitting them keeps call sites working.

### 3.4 Controlled binding (uniform across the three)

All three are **controlled** when `value`/`checked` is passed (per
`controlled-form-controls.md`): the control always displays exactly the prop, a
user action only *requests* a change via the `change` `CustomEvent`'s `detail`,
and the new value appears when the caller feeds it back into the prop. Omitting
`value`/`checked` leaves the control uncontrolled (it owns its own state and
still reports edits through `on_change`).

Push-in needs no interop code: Dioxus special-cases `value`/`checked` in
`setAttributeInner` as DOM *property* writes, and the 5.0.0 host maps host
properties to React props — via prototype accessors after upgrade, and by
re-applying pre-upgrade own properties in `connectedCallback`.

> **History — why this was uncontrolled until 5.0.0.** Two bridge gaps blocked
> controlled binding: the host's `attributeChangedCallback` ignored attribute
> *removals* and empty/`false` values (`(newValue || type==="method")`), so a
> controlled checkbox couldn't be unchecked and a controlled input couldn't be
> cleared; and a `value`/`checked` *property* set before the registration
> script ran was shadowed and lost — which is exactly where Dioxus's initial
> property write lands. 5.0.0 fixed both (they were the two ❌ items in
> `controlled-form-controls.md`), so the wrappers dropped the
> `default-value`/`default-checked` seeding workaround.

---

## 4. Footguns

Still live:

- **Controlled means you must echo.** When `value`/`checked` is passed, an edit
  only sticks once `on_change` feeds it back into the prop — bind it to a
  signal or the control freezes at the prop value. Omit the prop entirely for
  fire-and-forget usage.
- **The echo is asynchronous** — the keystroke→`on_change`→prop→DOM round-trip
  crosses the Dioxus and React schedulers. This *used* to bounce a controlled
  input's caret to the end on mid-string edits (the classic
  controlled-over-a-bridge artifact); **upstream 5.0.1 fixed it** by
  capturing/restoring the selection across the echo, so mid-string typing now
  keeps its caret. The async gap remains (don't assume the DOM reflects a new
  `value` synchronously), but it's no longer user-visible for text entry.
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
(`ElementHandle`/`set_attr`) — controlled binding rides Dioxus's own property
writes and needs none of it.

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

- **Shipped & runtime-verified (headless Chromium, against 5.0.1):** `web`
  feature + `src/event.rs`; `HmiInput`, `HmiSwitch`, `HmiCheckbox` (controlled
  `value`/`checked` + `on_change`); demo binds all three to signals with native
  readouts plus a reset button. Verified end-to-end: typing echoes through the
  signal and back into the input, an un-echoed controlled input stays frozen at
  its prop, toggles round-trip both directions, and the reset button clears the
  input / re-checks the switch / unchecks the box — the full
  `controlled-form-controls.md` acceptance checklist. Mid-string editing now
  preserves the caret (5.0.1 fix): inserting `X` at position 2 of `abcdef`
  yields `abXcdef` with the caret at 3, and a fast `XY` insert yields
  `abXYcdef` — both regressed before 5.0.1.
- **Not done:** `HmiChip` `on_select`/`on_close` — now a one-liner, since
  `on_input_event` already decodes `CustomEvent` `detail` (just pass `"select"`
  / `"close"`); other `change`-emitting controls (textarea, select/combobox,
  slider, radio-group, …) reuse the same `detail`-reading helper; data-viz
  wrappers (§5).
- **Resolved in 5.0.1:** the mid-string caret jump (the controlled echo crosses
  two schedulers) — upstream now preserves the selection across the echo (§4).
