# Elements whose value is a native JS object can't be thin-wrapped (date-picker, file-upload)

> **Audience:** maintainers of **`@ninoverse/hmi-components`** (the upstream npm
> package) and of this crate. This explains why two form elements can't be
> consumed as thin web-component wrappers today, and what an upstream change
> would need to look like.
>
> **Method note:** the analysis is against the **vendored compiled bundle**
> (`assets/vendor/hmi-components.iife.js`, v5.0.1) and runtime behaviour observed
> by driving the demo in a headless browser. The React→web-component bridge is
> [`@r2wc/react-to-web-component`](https://github.com/bitovi/react-to-web-component).

## TL;DR

A framework wrapper can only hand a custom element **strings** — through
attributes, or through the `value` DOM property the bridge forwards. The bridge
`JSON.parse`s `json`-typed props. So a wrapper can pass anything that is
JSON-serialisable, but **not** a live native object.

Two elements take their `value` (and emit their `onChange`) as native JS objects
that have no JSON representation the component will accept:

| Element | `value` type | Result through the web-component boundary |
|---|---|---|
| `date-picker` | `Date` (or `{ start: Date, end: Date }`) | **crashes** — see below |
| `file-upload` | `File[]` | impossible — `File`s can't be constructed from data |

### date-picker (verified)

The component calls `Date` methods directly on its `value`:

```js
const N = value ?? range?.start ?? new Date();
useState(new Date(N.getFullYear(), N.getMonth(), 1));   // N must be a real Date
```

Passing a JSON value — e.g. setting the `value` attribute to `"2024-06-15"`,
which the bridge `JSON.parse`s to a string — makes `N` a string, and the element
throws on render:

```
TypeError: N.getFullYear is not a function
```

(verified by injecting `<hmi-date-picker>` and setting a serialised `value`: the
element's rendered subtree is wiped and the page logs the error.) `min`, `max`
and `defaultValue` are all `Date`-`json` too, so they fail the same way. The
element renders **only** with no value at all — i.e. it can't be controlled,
bounded, or given an initial date through the boundary.

### file-upload (verified)

`value` is an array of `File` objects, keyed by name+size internally. `File`s
**cannot be constructed from serialised data** — the browser forbids
programmatically setting an `<input type=file>`'s files for security. So a
controlled `value` is impossible by the web platform, not merely by this design.
The element renders and the drop-zone works uncontrolled, but the selection can
only be read back out as live `File` objects in `onChange` — which the
attribute/`CustomEvent`-string model used by every other wrapper here can't
carry.

## Why this is a boundary problem, not a wrapper problem

No wrapper (Dioxus, Vue, Angular, plain HTML) can pass a live `Date` or `File`
across the custom-element boundary — only strings. So this must be fixed where
the React props are consumed.

## Proposal

- **date-picker:** accept and emit **ISO-8601 strings** (`"2024-06-15"`, or
  `{ "start": "...", "end": "..." }`) instead of `Date` objects — parse to
  `Date` internally, serialise on `onChange`. That makes `value`, `min`, `max`,
  and `defaultValue` cross the JSON boundary cleanly and the element fully
  wrappable.
- **file-upload:** a controlled `value` of real `File`s is not achievable across
  the boundary (browser security). The element is inherently uncontrolled; to be
  consumable it should emit a **serialisable description** of the selection in
  `onChange` (e.g. `[{ name, size, type }]`) rather than raw `File`s, so a
  wrapper can report what was chosen. Passing files *in* remains out of scope by
  platform rule.

## Impact on `hmi-dioxus`

Until these land upstream, `HmiDatePicker` and `HmiFileUpload` can't ship as
thin wrappers — `date-picker` crashes the moment a value is supplied, and
`file-upload`'s selection can't be carried back out. Both are deferred in
[`../TODO.md`](../TODO.md) with a pointer here. Once values are exchanged as
strings / serialisable descriptions, wrapping them is the usual
attribute-mapping exercise.
