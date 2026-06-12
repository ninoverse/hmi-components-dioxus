# Only 7 elements dispatch their callbacks — every other `onChange`/`onSelect`/… is dead

> **Audience:** maintainers of **`@ninoverse/hmi-components`** (the upstream npm
> package) and of this crate. This is the single most impactful web-component
> gap found so far: the **majority of interactive elements can't report user
> input** when consumed as web components, because their callbacks are never
> wired to a DOM event.
>
> **Method note:** the analysis is against the **vendored compiled bundle**
> (`assets/vendor/hmi-components.iife.js`, v5.0.1) and runtime behaviour observed
> by driving the demo in a headless browser. The React→web-component bridge is
> [`@r2wc/react-to-web-component`](https://github.com/bitovi/react-to-web-component).

## TL;DR

The registration helper (`tt`) takes an optional **events** argument that tells
the bridge to turn a React callback prop into a bubbling `CustomEvent`:

```js
tt("input", InputImpl,
   { /* props */ onChange: "function", … },
   { onChange: { bubbles: true } });   // ← the events config
```

The bridge only creates a dispatcher for callbacks listed **in that events
config**:

```js
// in the generated element class:
m = Array.isArray(r.events) ? r.events.slice() : Object.keys(r.events); // events config keys
…
for (const x of m)            // ← ONLY the configured callbacks
  this[props][x] = (T) => this.dispatchEvent(
    new CustomEvent(x.replace(/^on/, "").toLowerCase(), { detail: T }));
```

A callback declared in **props** but **absent from the events config** is never
turned into a dispatcher — calling it from inside the component is a no-op
through the bridge, and **no event is ever fired**.

Across all **85** registered elements, exactly **7** pass an events config:

| Element | Events config |
|---|---|
| `checkbox`, `input`, `password-input`, `radio`, `search-input`, `switch`, `textarea` | `{ onChange: { bubbles: true } }` |

Every other element with a callback — `select`, `combobox`, `tabs`,
`segmented-control`, `radio-group`, `slider`, `number-input`, `multi-input`,
`value-scale-selector`, `stepper`, `color-picker`, `pagination`, `navbar`,
`sidebar`, `tree`, `table`, `accordion`, `carousel`, `list` (reorder), the
overlays, … — declares its callback only in **props**, so the callback is
**dead** through the web-component boundary.

### Reproduction (verified)

Injecting a raw `<hmi-tabs options='[…]' value="x">` (no framework involved) and
clicking a tab dispatches **no** `change` `CustomEvent` — patching
`EventTarget.dispatchEvent` globally shows nothing. The same element rendered by
Dioxus behaves identically. By contrast, typing into `<hmi-input>` *does* fire a
`change` event, because `input` is one of the 7 with an events config.

## Why this is a boundary problem, not a wrapper problem

A web-component consumer can only observe a callback as a DOM event. If the
bridge never dispatches one, no wrapper (Dioxus, Vue, Angular, plain HTML) can
read the user's selection back out. So this must be fixed at registration.

## Proposal

Add the missing callbacks to each element's events config when calling `tt`.
For example:

```js
tt("tabs", TabsImpl, { …props, onChange: "function" }, { onChange: { bubbles: true } });
tt("select", SelectImpl, { …props, onChange: "function" }, { onChange: { bubbles: true } });
tt("confirm-dialog", …, …, { onConfirm: { bubbles: true }, onCancel: { bubbles: true } });
```

i.e. **every** `on*` callback an element exposes should appear in its events
config. This is a one-line-per-callback change at each registration site and
makes the elements reportable as web components without touching their React
internals.

## Impact on `hmi-dioxus`

Only the 7 elements above can be shipped as interactive wrappers today
(`HmiInput`, `HmiTextarea`, `HmiCheckbox`, `HmiSwitch`, `HmiRadio`,
`HmiPasswordInput`, `HmiSearchInput`). Wrappers for the rest render and display
controlled values, but their `on_change` never fires — a controlled wrapper is
therefore *frozen*. They are deferred in [`../TODO.md`](../TODO.md) with a
pointer here. Once the events configs are added upstream, wrapping them is the
usual attribute-mapping + event-listener exercise (the wrapper code already
written for them is essentially ready).
