# Nested interactive children lose their bindings (form-control, and any control-wrapping element)

> **Audience:** maintainers of **`@ninoverse/hmi-components`** (the upstream npm
> package) and of this crate. This proposes a small upstream change that would
> let elements which **wrap another control as children** work when consumed as
> **web components**.
>
> **Method note:** the analysis is against the **vendored compiled bundle**
> (`assets/vendor/hmi-components.iife.js`, v5.0.1) and runtime behaviour observed
> by driving the demo in a headless browser. The React→web-component bridge is
> [`@r2wc/react-to-web-component`](https://github.com/bitovi/react-to-web-component).

## TL;DR

The bridge captures an element's slotted light-DOM children **once at connect**
and re-renders them through React's `dangerouslySetInnerHTML`:

```js
// o0(...) — builds the children passed to the wrapped React component:
const y = container ? container[childrenProp] : void 0;            // captured innerHTML string
const v = y ? React.createElement("span", {
  style: { display: "contents" },
  dangerouslySetInnerHTML: { __html: y },                          // re-parsed from a string
}) : void 0;
return React.createElement(Component, props, v);
```

Because the children are **re-parsed from an HTML string**, any nested
*interactive* `hmi-*` control is destroyed and recreated as a fresh copy. The
copy keeps the attributes that were present in the serialized HTML (e.g.
`placeholder`, `error`), but loses everything a framework wrapper sets **after**
mount and **off-attribute**:

- the **`value` / `checked` DOM properties** (Dioxus sets these as properties,
  not attributes — they are never in the captured HTML), and
- the **event listeners** the wrapper attached to the original node (the
  `change` `CustomEvent` listener that reads the control's value back out).

So a controlled control nested inside such a wrapper renders, but **its value
binding and `on_change` are dead** — the parent can neither push a value in nor
read edits out.

### Affected element

| Element | Content prop(s) | Trigger | Problem |
|---|---|---|---|
| `form-control` | `label`, `hint`, `error` (json) | **children** (the control) | nested control loses `value`/`checked` + `on_change` |

`form-control`'s entire purpose is to wrap a form control with a label, hint and
error message, so this limitation removes its reason to exist as a thin wrapper.

> Presentational wrappers that take children (`card`, `box`, `banner`, `alert`,
> `blockquote`, …) are **not** affected in practice: their children are static
> markup, which `dangerouslySetInnerHTML` round-trips faithfully. The problem is
> specific to wrapping **interactive** custom elements that depend on live
> properties and listeners.

### Reproduction (verified)

Rendering
`<hmi-form-control label="Username" error="taken"><hmi-input value="admin" error></hmi-input></hmi-form-control>`
and inspecting the DOM after mount: the inner `<hmi-input>` exists and shows its
`error` border, but its native `<input>` `value` is **empty** — the `value`
property set by the framework was lost when the child was re-parsed from the
captured HTML string. No `change` listener survives on the recreated node.

## Why this is a web-component problem, not a wrapper problem

A framework wrapper (Dioxus, Vue, Angular, plain HTML) can only hand children to
a custom element as **slotted DOM**. It cannot prevent the bridge from
serialising them to a string and re-parsing them. So no wrapper change fixes
this — it must be fixed where the bridge renders children.

## Proposal: render children through a real slot, not `dangerouslySetInnerHTML`

Render the captured light-DOM children by **moving the real nodes** into the
React tree (or projecting them through a `<slot>` in a shadow root), instead of
serialising and re-parsing them:

- preserves the identity of nested custom elements, so their live properties
  (`value`/`checked`) and event listeners survive;
- still works for static markup children (the common case today);
- matches how native `<slot>` projection behaves.

A minimal version: keep a `ref` to a host node and `appendChild` the original
child nodes once, rather than `dangerouslySetInnerHTML`. The nested controls
then keep the bindings their framework wrapper gave them.

## Impact on `hmi-dioxus`

Until this is addressed upstream, `HmiFormControl` cannot be shipped as a thin
wrapper — it would render its label/hint/error chrome but silently break the
binding of any control placed inside it. It is deferred in
[`../TODO.md`](../TODO.md) with a pointer to this document. Once children are
projected as real nodes upstream, wrapping it is the usual
attribute-mapping-plus-children exercise.
