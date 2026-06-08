# Events, value binding & data-viz — spike findings

> **Status:** design spike (decision record). No library code changes accompany
> this document; the pattern below is the recommended approach for a **follow-up
> implementation PR**. It exists to de-risk the two `TODO.md` "Open questions"
> before the interactive component set is wrapped en masse.

## TL;DR

1. **Event callbacks & two-way binding are feasible and have a clean pattern.**
   The upstream components dispatch ordinary DOM `CustomEvent`s (`select`, `close`,
   `change`, …) whose `detail` carries the callback argument. Dioxus 0.7 can't name
   a custom event in `rsx!`, but `onmounted` + `web_sys`
   `add_event_listener_with_callback` attaches a listener on the element. Callbacks
   are exposed to consumers as `EventHandler<T>`; values bind two-way via the
   `value` attribute (in) + the `change` event's `detail` (out).
2. **Data-viz elements render standalone.** Charts are plain custom elements that
   take JSON props and default `width`/`height`. `hmi-responsive-container` is an
   optional ResizeObserver wrapper for fluid width — not a required context.

---

## 1. How the upstream components expose behaviour

`@ninoverse/hmi-components` (3.1.2) is a **React→web-component bridge**. Each
element is registered via a `tt("name", ReactComponent, { props })` helper, and
function-typed props are turned into DOM events by this bridge code (minified, in
`assets/vendor/hmi-components.iife.js`):

```js
// for each function-typed prop `b` (e.g. "onSelect"):
this[Ce][b] = A => {
  const N = b.replace(/^on/, "").toLowerCase();      // onSelect -> "select"
  this.dispatchEvent(new CustomEvent(N, { detail: A, ...v[b] }));
};
```

So a React-style `onXxx` callback becomes a DOM **`CustomEvent` named `xxx`** whose
`event.detail` is exactly the argument the callback would have received. **These
events do not bubble** (`bubbles` is unset/false).

Values are exposed as JS **properties** with getters/setters that mirror to the
matching **attribute** when the value is stringifiable (`value`, `checked`,
`selected`, `open`, …). That gives a symmetric model:

- **Push a value in:** set the attribute (`value: "{...}"`).
- **Read a value out:** listen to the `change` event and read `event.detail`
  (equivalently, read the element's `value` property).

### Event-name mapping

| React prop | DOM event | Emitting elements (examples) | `detail` payload |
|------------|-----------|------------------------------|------------------|
| `onSelect` | `select`  | chip, tree                   | `boolean` (chip = selected state) |
| `onClose`  | `close`   | chip, drawer, modal          | none |
| `onChange` | `change`  | input, textarea, select, combobox, checkbox, radio-group, switch, slider, stepper, tabs, color-picker, date-picker, number-input, segmented-control, value-scale-selector, file-upload, multi-input, pagination | value-typed (string/number/bool) |
| `onConfirm`| `confirm` | confirm-dialog               | dialog-defined |
| `onCancel` | `cancel`  | confirm-dialog               | none |
| `onNav`    | `nav`     | navbar, sidebar              | nav target |

> The exact `detail` shape for `json`-typed payloads (e.g. a select's option
> object) is not fully recoverable from the minified bundle. Confirm those against
> the upstream `.d.ts` (`npm pack @ninoverse/hmi-components@3.1.2`) when wrapping
> each one — `npm` + network are available in this environment.

### Per-interactive-component reference (verified from the bundle)

| Element | Value property | Event(s) | `detail` |
|---------|----------------|----------|----------|
| chip    | `selected: boolean` | `select`, `close` | `boolean` / — |
| input   | `value: string` (mirrors `value` attr) | `change` | `string` |
| textarea| `value: string` | `change` | `string` |
| checkbox / switch / radio | `checked: boolean`, `value: string` | `change` | `boolean`/`string` |
| select / combobox | `value: string`, `options: json` | `change` | `string` |
| slider  | `value: number` (`min`/`max`/`step`) | `change` | `number` |
| tabs    | `value: string` (`options: json`) | `change` | `string` |
| modal / drawer | `open: boolean` | `close` | — |

---

## 2. Dioxus 0.7 constraints

- `rsx!` only accepts **predefined** typed event handlers (`onclick`, `oninput`,
  `onchange`, `onmounted`, …). There is **no syntax for an arbitrary custom event
  name** like `onselect`. Dioxus-web's root delegation also only sees **bubbling**
  events — and these CustomEvents don't bubble — so a delegated handler wouldn't
  fire even for the `change`-named event.
- `onmounted` **is** available under the library's `features = ["lib"]` (it lives
  in `dioxus-html`, not the web renderer). On the web renderer,
  `MountedData::downcast::<web_sys::Element>()` returns the live element; from
  there `web_sys`'s `add_event_listener_with_callback` attaches a listener **on the
  element itself**, which is what non-bubbling CustomEvents require.

**Conclusion:** the listener must be wired in `onmounted` via `web-sys`, which is a
web-only dependency the renderer-agnostic library doesn't currently pull in.

---

## 3. Recommended pattern

### 3.1 Optional `web` cargo feature (library)

Keep the library renderer-agnostic by default; add a `web` feature that pulls the
web-only deps. The library must still build with the feature **off** (docs.rs and
non-web renderers).

```toml
# Cargo.toml — illustrative, lands in the implementation PR
[features]
web = ["dep:web-sys", "dep:wasm-bindgen"]

[dependencies]
dioxus = { version = "0.7", default-features = false, features = ["lib"] }
wasm-bindgen = { version = "0.2", optional = true }
web-sys = { version = "0.3", optional = true, features = [
  "Element", "EventTarget", "Event", "CustomEvent",
  "HtmlElement", "HtmlInputElement",
] }
```

The demo forwards it: `hmi-dioxus = { path = "..", features = ["web"] }`.

### 3.2 A cfg-gated interop helper

To avoid `#[cfg]` on an `rsx!` attribute (awkward), the wrappers always call one
helper whose body is real under `web` and a no-op otherwise — so the wrapper code
is identical across platforms.

```rust
// src/event.rs — illustrative, not yet in the crate

/// Attach a DOM listener for `event_name` on the just-mounted element and forward
/// `event.detail` to `handler`. No-op off the web renderer.
#[cfg(feature = "web")]
pub(crate) fn on_custom<T>(
    mounted: &MountedData,
    event_name: &'static str,
    handler: EventHandler<T>,
    decode: fn(wasm_bindgen::JsValue) -> Option<T>,
) {
    use wasm_bindgen::{closure::Closure, JsCast};
    let Some(el) = mounted.downcast::<web_sys::Element>() else { return };
    let cb = Closure::<dyn Fn(web_sys::Event)>::new(move |e: web_sys::Event| {
        let detail = e.unchecked_into::<web_sys::CustomEvent>().detail();
        if let Some(value) = decode(detail) {
            handler.call(value);
        }
    });
    let _ = el.add_event_listener_with_callback(event_name, cb.as_ref().unchecked_ref());
    cb.forget(); // spike: leak one closure per mount; store + drop for production
}

#[cfg(not(feature = "web"))]
pub(crate) fn on_custom<T>(
    _: &MountedData, _: &'static str, _: EventHandler<T>, _: fn(/*…*/),
) {}
```

`decode` turns the `JsValue` `detail` into the typed payload — e.g. `|d| d.as_bool()`
for chip `select`, `|d| d.as_string()` for input `change`.

### 3.3 Callback props use `EventHandler<T>`

`EventHandler<T>` is Copy/Clone (so it moves cleanly into the closure) and is the
idiomatic Dioxus callback type. Prefer it over `Box<dyn Fn>` (not Clone) and over
ad-hoc channels.

```rust
// src/components/chip.rs — illustrative upgrade
#[component]
pub fn HmiChip(
    #[props(default)] selected: bool,
    #[props(default)] icon: Option<String>,
    #[props(default)] on_select: EventHandler<bool>,
    #[props(default)] on_close: EventHandler<()>,
    children: Element,
) -> Element {
    rsx! {
        hmi-chip {
            "selected": if selected { "true" },
            "icon": icon,
            onmounted: move |m| {
                crate::event::on_custom(&m, "select", on_select, |d| d.as_bool());
                crate::event::on_custom(&m, "close",  on_close,  |_| Some(()));
            },
            {children}
        }
    }
}
```

When a consumer doesn't pass a callback, `EventHandler` defaults to a no-op, so
existing call sites keep working.

### 3.4 Two-way value binding

Push the value in via the attribute; read changes out via the `change` event and
drive a `Signal` in the consumer.

```rust
// src/components/input.rs — illustrative new wrapper
#[component]
pub fn HmiInput(
    #[props(default)] value: String,
    #[props(default)] placeholder: Option<String>,
    #[props(default)] on_change: EventHandler<String>,
) -> Element {
    rsx! {
        hmi-input {
            "value": value,
            "placeholder": placeholder,
            onmounted: move |m| {
                crate::event::on_custom(&m, "change", on_change, |d| d.as_string());
            },
        }
    }
}

// demo usage — the Signal is the single source of truth, fed back into `value`
let mut text = use_signal(String::new);
rsx! {
    HmiInput { value: "{text}", on_change: move |v| text.set(v) }
    p { "You typed: {text}" }
}
```

---

## 4. Data-viz: standalone render confirmed

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

**Implication for the roadmap:** the data-viz group can be wrapped with the same
attribute-only idiom as the presentational set — JSON in via a string prop, explicit
dimensions, optionally nested in an `HmiResponsiveContainer`. No special handling
is needed; the only open work is confirming each chart's exact JSON `detail`/prop
shapes against the upstream `.d.ts`.

---

## 5. Risks & footguns (for the implementation PR)

- **Non-bubbling events** → must attach the listener on the element in `onmounted`,
  not rely on Dioxus delegation. (Confirmed above.)
- **`Box<dyn Fn>` isn't Clone** → use `EventHandler<T>` so the callback can move
  into the `web_sys` `Closure`.
- **Closure lifetime** → `Closure::forget` leaks one closure per mount. Fine for a
  spike/demo; for production, store the `Closure` (e.g. in a `use_hook`) and drop it
  on unmount, or remove the listener.
- **`cfg` on `rsx!` attributes** is awkward → keep wrappers platform-identical by
  always calling the `on_custom` helper, whose body is gated, not the `onmounted`
  line.
- **`json` payload shapes** (select options, chart `detail`s) aren't fully visible
  in the minified bundle → cross-check the upstream `.d.ts` per component.
- **Feature hygiene** → verify the crate builds with the feature **off**
  (`cargo build`) and on (`cargo build --features web` / the demo), so docs.rs and
  non-web renderers stay green.

---

## 6. Follow-up implementation checklist

1. Add the `web` feature + optional `web-sys`/`wasm-bindgen` deps; demo enables it.
2. Add `src/event.rs` with the gated `on_custom` helper (+ `mod event;` in `lib.rs`).
3. Upgrade `HmiChip` with `on_select`/`on_close` and a `selected` attribute.
4. Add `HmiInput` (two-way `value` + `on_change`) and `HmiSparkline` (data-viz).
5. Demo: interactive chip, a `Signal`-bound input, and a sparkline with sample data.
6. Verify: `cargo build` (feature off) **and** the demo build (feature on),
   `cargo clippy --all-targets -- -D warnings`, `dx check`, `dx build --release
   --platform web`, plus a visual check via `dx serve`.
