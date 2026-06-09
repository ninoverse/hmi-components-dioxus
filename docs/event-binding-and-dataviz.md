# Events, value binding & data-viz

> **Status:** the **event-binding** half of this document is now a decision
> record describing the **shipped** implementation — the `web` cargo feature,
> `src/event.rs`, and the `HmiInput` / `HmiSwitch` / `HmiCheckbox` wrappers. It
> supersedes the original spike, whose `detail`-based model turned out to be
> wrong for these form controls (see §1). The **data-viz** half (§5) is still a
> forward-looking spike for a group that hasn't been wrapped yet.

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

So there is **no `change` CustomEvent and no `detail`**. What reaches the host
is the inner `<input>`'s **native** `input` (per keystroke) and `change` (on
commit / toggle) events, which bubble up. The value is read from the event
target: `event.target.value` (string) or `event.target.checked` (bool).

### Event reference (corrected, verified against the bundle + runtime)

| Element | Value source | Event to listen for | Read from |
|---------|--------------|---------------------|-----------|
| input | inner `<input>.value` | native `input` (bubbles) | `target.value` |
| switch / checkbox | inner `<input type=checkbox>.checked` | native `change` (bubbles) | `target.checked` |
| chip | — | `select` / `close` **CustomEvent** | `event.detail` (`bool` / —) |

> The `chip` path (a real `detail`-carrying CustomEvent) is **not** wired up by
> the current wrappers — it would need a second, `detail`-decoding helper
> alongside the target-decoding `on_input_event` described below.

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
  "Element", "EventTarget", "Event", "HtmlInputElement", "Window",
] }
```

The demo forwards it: `web = ["dioxus/web", "hmi-dioxus/web"]`.

### 3.2 `src/event.rs` — the interop helpers

All are `#[cfg]`-gated with no-op off-web bodies so wrapper code is identical
across renderers.

**`on_input_event`** — attach a listener and forward decoded values out:

- Downcasts the `onmounted` `MountedData` to `web_sys::Element` (the dioxus-web
  backing type) and attaches the listener **on the host**.
- Registers in the **capture phase** so it reads the user-intended value
  *before* React's bubble-phase handler reverts a controlled input.
- Wraps the body in `Runtime::wrap_closure` (re-enters the runtime captured at
  mount) and calls `schedule_update()` after `handler.call(value)` to wake the
  render loop.
- Returns a `ListenerGuard` that removes the listener on drop (no per-mount
  leak); the wrapper stores it in a `use_signal` for the component's lifetime.
- `decode: fn(&InputTarget) -> Option<T>` reads the value, e.g.
  `|el| Some(el.value())` (string) or `|el| Some(el.checked())` (bool).

**`ElementHandle`** (+ `host_element(&MountedData)`) — push values in:

- `set_attr(name, Option<&str>)` — used for the input `value` (an *attribute*,
  not the property Dioxus would set).
- `set_inner_checked(bool)` — sets the inner `<input>`'s `checked` **property**
  directly, keeping React uncontrolled (see §4). The inner input is rendered
  asynchronously after the host mounts, so this retries on animation frames
  (capped) until the input exists.

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
    let mut host = use_signal(|| Option::<ElementHandle>::None);

    // Push `checked` to the inner input whenever it (or the host) changes.
    use_effect(move || {
        if let Some(h) = host.read().as_ref() {
            h.set_inner_checked(checked);
        }
    });

    rsx! {
        hmi-switch {
            "label": label.as_deref().map(json_string), // JSON-encoded text
            "disabled": if disabled { "true" },
            "name": name,
            "value": value,
            onmounted: move |m| {
                host.set(host_element(&m));
                listener.set(on_input_event(&m, "change", on_change, |el| Some(el.checked())));
            },
        }
    }
}
```

`HmiInput` is the same shape with `on_input_event(&m, "input", …, |el| Some(el.value()))`
and `host.set_attr("value", Some(&value))` in its effect. Callbacks use
`EventHandler<T>` (Copy, default no-op) so omitting them keeps call sites working.

### 3.4 Controlled vs uncontrolled — the key decision

- **Text input** is bound **controlled**: `value` is pushed in as an attribute
  and read out per keystroke. React would revert each edit, but the
  capture-phase read feeds `on_change` the typed value and the `use_effect`
  pushes it back, so the controlled input converges.
- **Switch / checkbox** must stay **uncontrolled**: the bridge forwards a
  `checked` attribute to React as a controlled prop, and *with no `onChange`
  React refuses to let the user toggle it*. Driving the inner input's `checked`
  **property** instead leaves React uncontrolled, so it toggles freely, and the
  property write is re-applied from the signal via `use_effect`.

---

## 4. Footguns (what actually bit us)

- **`detail` vs `target`.** Only `CustomEvent`-based callbacks (chip) carry a
  `detail`; native form events carry the value on `event.target`.
- **`value`/`checked` are properties, not attributes, in Dioxus** → they never
  reach a custom element through `rsx!`. Sync them with `web-sys`.
- **A controlled checkbox with no `onChange` can't be toggled** → drive the
  inner `checked` property, don't set the `checked` attribute.
- **The inner `<input>` is rendered async** (a frame or more after the host
  mounts) → poll across animation frames before giving up.
- **Runtime + scheduler.** Re-enter the runtime (`Runtime::wrap_closure`) and
  call `schedule_update()`; otherwise the handler/signal write no-ops or never
  re-renders.
- **React reverts controlled edits** → listen in the **capture** phase to read
  the user value first.
- **Hmi\* elements snapshot their slot `innerHTML` at `connectedCallback`** →
  reactive text placed *inside* an `Hmi*` element won't update. Put dynamic
  text in a native element (the demo's readouts do this).
- **`json` attributes are `JSON.parse`d** → encode plain text as a JSON string
  (a shared `json_string` helper does this for labels).
- **Feature hygiene** → CI/locals must build both `cargo build` (feature off)
  and `cargo build --features web` (on), plus the demo.

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

- **Shipped:** `web` feature + `src/event.rs`; `HmiInput` (two-way `value` +
  `on_change`), `HmiSwitch` and `HmiCheckbox` (`checked` + `on_change`); demo
  binds all three to signals with native readouts.
- **Not done:** `HmiChip` `on_select`/`on_close` (would add a `detail`-decoding
  helper next to `on_input_event`); other `change`-emitting controls
  (textarea, select/combobox, slider, radio-group, …) follow the same
  target-reading pattern; data-viz wrappers (§5).
