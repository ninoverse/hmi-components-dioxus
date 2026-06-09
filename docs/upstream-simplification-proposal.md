# Upstream changes that would simplify this crate

> **✅ Resolved in upstream 4.2.0.** All three proposals below shipped: `input`,
> `switch`, `checkbox` (and `textarea`/`radio`/`password-input`/`search-input`)
> now expose `onChange` wired onto the inner `<input>` (§3.1), `defaultValue` /
> `defaultChecked` (§3.2), and register their callback with `{bubbles: true}`
> (§3.3). This crate was simplified accordingly — see the "after" column in §4
> and the updated [`event-binding-and-dataviz.md`](event-binding-and-dataviz.md).
> The original proposal is kept below as the record of why.

> **Audience:** maintainers of **`@ninoverse/hmi-components`** (the upstream npm
> package) and of this crate. This is a review of the event-binding approach in
> `src/event.rs` and a proposal for small upstream changes that would let us
> delete most of it.
>
> **Method note:** the upstream *source* repo was not reachable from the session
> this review was written in, so the analysis is against the **vendored compiled
> bundle** (`assets/vendor/hmi-components.iife.js`, v3.1.2) — the authoritative
> record of every element's prop schema and of the React→web-component bridge
> (which is [`@r2wc/react-to-web-component`](https://github.com/bitovi/react-to-web-component);
> the `Symbol.for("r2wc.*")` keys in the bundle confirm it). Everything below is
> grounded in that bundle plus the runtime behaviour already recorded in
> [`event-binding-and-dataviz.md`](event-binding-and-dataviz.md).

## TL;DR

All of the complexity in `src/event.rs` (a capture-phase `web-sys` listener,
runtime re-entry, manual scheduler wake, an `ElementHandle` that pushes
`value`/`checked` imperatively, and an animation-frame retry loop) exists for
**one reason**: the `input`, `switch` and `checkbox` elements are the only
interactive controls upstream that expose **no callback prop and no
`defaultValue`**. They take `value`/`checked` as one-way display attributes and
emit only the inner native `input`/`change` events.

The other interactive elements upstream — `combobox`, `select`, `slider`,
`number-input`, `segmented-control`, `radio-group`, `color-picker`,
`date-picker`, `tabs`, … — already follow the **controlled/uncontrolled React
pattern**: `value` + `onChange` + `defaultValue`. If the three controls this
crate wraps adopted that same shape, `src/event.rs` could shrink to a small
`detail`-decoding helper (or disappear entirely).

## 1. The root cause, in the schemas

Each element is registered by `tt("name", ReactComponent, schema)`. The schema
is the contract this crate codes against. Grouping the interactive elements by
whether they expose a callback:

**Have `onChange` (or equivalent) + `defaultValue` — clean to wrap:**

| Element | Relevant schema keys |
|---|---|
| `combobox` | `value`, `defaultValue`, `onChange`, `options`, … |
| `number-input` | `value`, `defaultValue`, `onChange`, `min`, `max`, … |
| `slider` | `value`, `defaultValue`, `onChange`, … |
| `select` | `value`, `defaultValue`, `onChange`, `options`, … |
| `segmented-control` | `value`, `defaultValue`, `onChange`, … |
| `radio-group` | `name`, `value`, `defaultValue`, `options`, `onChange` |
| `color-picker` / `date-picker` / `file-upload` | `value`/`defaultValue` + `onChange` |
| `tabs` | `value`, `onChange`, … |
| `chip` | `selected`, `onSelect`, `onClose` |

**No callback prop, no `defaultValue` — the hard ones:**

| Element | Full schema | Wrapped here? |
|---|---|---|
| `input` | `leftIcon`, `rightIcon`, `error`, `value`, `placeholder`, `disabled`, `type`, `name` | ✅ |
| `switch` | `label`, `checked`, `disabled`, `name`, `value` | ✅ |
| `checkbox` | `label`, `checked`, `disabled`, `name`, `value` | ✅ |
| `radio` | `label`, `checked`, `disabled`, `name`, `value` | — |
| `textarea` | `error`, `value`, `placeholder`, `disabled`, `rows`, `name` | — |
| `password-input` | `leftIcon`, `error`, `value`, `placeholder`, `disabled` | — |
| `search-input` | `error`, `value`, `placeholder`, `disabled` | — |

The three controls this crate ships interactivity for are **exactly** the
outliers, and the same gap blocks the four un-wrapped text controls. The crate
isn't doing anything idiosyncratic — it's compensating for a missing prop.

## 2. Why the missing prop forces every line of `src/event.rs`

The r2wc bridge turns a React `onXxx` function prop into a
`dispatchEvent(new CustomEvent("xxx", { detail: arg }))` (verified in the
bundle; it's how `chip`'s `onSelect`→`select` already works). It also installs a
property accessor for every schema prop whose setter coerces the value, reflects
it to the observed attribute, and re-renders React.

Because `input`/`switch`/`checkbox` declare **no** `onChange`:

1. **Reading values out** has no event to subscribe to but the inner native
   `input`/`change`. Those fire on the React-rendered inner `<input>`, which has
   no `data-dioxus-id`, so Dioxus's delegated `oninput`/`onchange` never sees
   them. The crate attaches a raw `web-sys` listener on the host — and because
   the element is React-*controlled* with no `onChange`, **React reverts every
   edit**, so the listener must run in the **capture phase** to read the value
   first, then re-enter the Dioxus runtime and wake the scheduler by hand.
   (`on_input_event`, ~60 lines + the `ListenerGuard`.)

2. **Pushing values in** can't go through `rsx!` (Dioxus special-cases
   `value`/`checked` as DOM *properties*, set pre-upgrade where they shadow the
   bridge's prototype accessor and never reach React). So the crate keeps an
   `ElementHandle`, sets `value` via `setAttribute`, and — because a controlled
   checkbox with no `onChange` *cannot be toggled by the user* — drives the
   inner checkbox's `checked` **property** directly, retrying across animation
   frames until the async-rendered inner input exists. (`ElementHandle`,
   `set_inner_checked`, `sync_checked`, the per-wrapper `use_effect`.)

Both problems are downstream of the same missing callback.

## 3. Proposed upstream changes

### 3.1 (Primary) Add `onChange` to `input`, `switch`, `checkbox` (+ `textarea`, `radio`, `password-input`, `search-input`)

Add the prop to the schema **and** wire it through the React component onto the
inner `<input>` (`onChange={e => props.onChange?.(e.target.value)}` for text;
`(e.target.checked)` for switch/checkbox), matching what `combobox`/`select`
already do. Then:

- The bridge emits a `change` `CustomEvent` whose `detail` is the value, so this
  crate reads `event.detail` through **one** small helper — the same
  `detail`-decoding path `chip` needs — instead of the bespoke
  target-reading/capture-phase listener.
- With an `onChange` present the React control is *properly* controlled, so it
  stops reverting user edits: the capture-phase race, the runtime/scheduler
  dance, and the "uncontrolled checkbox" property hack all become unnecessary.

### 3.2 (Recommended) Add `defaultValue` / `defaultChecked`

With `onChange` in place, also exposing `defaultValue` (text) and
`defaultChecked` (switch/checkbox) lets a consumer render the control
**uncontrolled** and never push state back in. That deletes the entire
write-in side for this crate — `ElementHandle`, `set_inner_checked`,
`sync_checked`, and the per-wrapper `use_effect` — leaving wrappers that only
*read* `onChange`. This mirrors `combobox`/`number-input`/`slider`, which
already expose both `value` and `defaultValue`.

### 3.3 (Nice-to-have) Dispatch the callback events with `bubbles: true`

r2wc accepts per-event options; the bundle spreads them into the `CustomEvent`
init (`new CustomEvent(name, { detail, ...opts })`). Registering the form-control
events as **bubbling** would let Dioxus's *delegated* `onchange`/`oninput` catch
them at the document root — potentially letting this crate bind events straight
from `rsx!` with **zero** `web-sys`. (Lower priority: even non-bubbling, a host
listener reading `detail` is far simpler than today.)

## 4. Net effect on this crate

| Today (`src/event.rs`, ~230 lines) | After 3.1 + 3.2 |
|---|---|
| `on_input_event` capture-phase listener, `Runtime::wrap_closure`, `schedule_update` | one `detail`-decoding host listener shared with `chip` (or nothing, with 3.3) |
| `ElementHandle`, `set_attr`, `set_inner_checked`, `sync_checked` animation-frame retry | removed (controls rendered uncontrolled) |
| Per-wrapper `host`/`listener` signals + `use_effect` push-in | wrappers just forward `on_change` |
| Controlled-vs-uncontrolled workaround prose in the design doc | a single sentence: "these emit `change` like everything else" |

The `web` cargo feature and its `web-sys`/`wasm-bindgen` deps were expected to
become droppable. **In practice (4.2.0) they did not** — see §6.

## 5. If upstream can't change

Until then, the current `src/event.rs` approach is the correct workaround and
should stay. The only crate-side simplification available **without** upstream
changes is to fold `chip`'s (already-correct) `detail`-based callback and the
form controls behind a single helper module — but the form controls still need
the capture-phase/property machinery described in §2, so the saving is modest.
The high-leverage fix is the upstream prop additions in §3.

## 6. After 4.2.0: what the `web` feature still needs

4.2.0 shipped §3.1–§3.3 and the crate was simplified accordingly: an
**uncontrolled**, `detail`-reading implementation (seed via
`default-value`/`default-checked`, read via the `change` `CustomEvent`), verified
end-to-end in headless Chromium. See
[`event-binding-and-dataviz.md`](event-binding-and-dataviz.md) §3.

The optimistic §4 prediction that the `web` feature could be **dropped** still
didn't pan out — but for **one** remaining reason, not two:

1. **Reading the value still needs `web-sys`.** The bubbling `change` event
   (§3.3) lets Dioxus's delegated `onchange` *fire*, but it hands the wrapper an
   `Event<FormData>` whose value is derived from the host's `value` **property**,
   not the user's just-emitted `detail`. Getting `detail` means downcasting to
   `web_sys::CustomEvent`, so binding from `rsx!` saves nothing.
2. ~~Writing `value`/`checked` needs `web-sys`.~~ **No longer applies** — the
   wrappers are uncontrolled, so they never push `value`/`checked` in; the
   `default-value`/`default-checked` seeds go through `rsx!` as ordinary
   attributes (not Dioxus-special-cased), needing no `web-sys`.

**Follow-up for a future upstream version** — to let this crate drop the `web`
feature entirely, the control would have to **reflect** the live value to a place
Dioxus's typed `onchange` already reads (e.g. keep the host's `value`/`checked`
property in sync with the inner input so `FormData`/the event target yields the
user value without touching `detail`). Then the wrapper could bind `onchange`
straight from `rsx!`. That doesn't hold today, so the `web` feature stays.

The realistic near-term win is **breadth**, not dep removal: `textarea`, `radio`,
`password-input` and `search-input` now expose the same
`onChange`/`defaultValue` surface and can be wrapped by reusing `on_input_event`
unchanged.

## 7. Next request: controlled binding

The shipped binding is **uncontrolled** (initial state + `on_change`). To support
**controlled** two-way binding — where the host owns the value and can clear or
reset it — the web components need two small behaviour fixes (honour empty/`false`
updates; keep the property path reliable through upgrade). The required behaviour
is specified in [`controlled-form-controls.md`](controlled-form-controls.md).
