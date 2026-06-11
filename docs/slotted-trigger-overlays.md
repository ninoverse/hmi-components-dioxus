# Slotted triggers don't work for overlay components (tooltip, popover, hover-card, context-menu)

> **Audience:** maintainers of **`@ninoverse/hmi-components`** (the upstream npm
> package) and of this crate. This proposes a small upstream change that would
> let the trigger-based overlay elements work when consumed as **web
> components** (which is how every framework wrapper, including `hmi-dioxus`,
> uses them).
>
> **Method note:** the upstream *source* repo was not reachable from the session
> this was written in, so the analysis is against the **vendored compiled
> bundle** (`assets/vendor/hmi-components.iife.js`, v5.0.1) and runtime
> behaviour observed by driving the demo in a headless browser. The
> React→web-component bridge is
> [`@r2wc/react-to-web-component`](https://github.com/bitovi/react-to-web-component).

## TL;DR

Four registered elements position a floating layer relative to a **trigger that
the consumer supplies as children**:

| Element | Content prop | Trigger | Open on |
|---|---|---|---|
| `tooltip` | `label` (json) | **children** | hover / focus |
| `hover-card` | children | **children** *(or `trigger` json)* | hover |
| `popover` | children | **children** *(or `trigger` json)* | click |
| `context-menu` | `menu` (json) | **children** | right-click |

Each wires its trigger by cloning the child React element and injecting handlers
and a positioning `ref`:

```js
// tooltip / hover-card
cloneElement(child, { ref, onMouseEnter: e => { child.props.onMouseEnter?.(e); open() }, ... })
// popover
cloneElement(child, { ref, onClick: e => { child.props.onClick?.(e); toggle() } })
// context-menu
cloneElement(child, { onContextMenu: e => { e.preventDefault(); openAt(e) } })
```

`React.cloneElement` requires a **React element**. But when these tags are used
as web components, the bridge captures the slotted light-DOM trigger as an
**HTML string**, not an element:

```js
// inside tt(...) — the registration helper, once per element at connect:
const p = this.innerHTML.trim();
p && (this[childrenProp] = p);
```

So `cloneElement` has nothing real to clone: the handlers and the positioning
`ref` are never attached. The trigger renders (the slotted markup is still
shown), but **the overlay never opens and never positions** — there is no
`pointerenter`/`click`/`contextmenu` listener and no anchor `ref`.

### Reproduction (verified)

Rendering `<hmi-tooltip label="…"><button>Hover me</button></hmi-tooltip>` and
hovering — or even dispatching `pointerenter`/`mouseenter` directly on both the
host and the button — produces **no** `.tooltip` / `[role="tooltip"]` portal in
the document. The host's rendered subtree is just the slotted children; no
anchor wrapper, ref, or handler is present. The element is inert.

`hover-card` and `popover` also accept a `trigger` **json** prop, which is a
partial escape hatch (a serialisable trigger description), but the natural
"wrap my own element" path is broken, and `tooltip`/`context-menu` have no such
prop at all.

## Why this is a web-component problem, not a wrapper problem

A framework wrapper (Dioxus, Vue, Angular, plain HTML) can only hand the trigger
to a custom element as **slotted DOM**. There is no way for a wrapper to pass a
live React element across the custom-element boundary. So no amount of wrapper
cleverness fixes this — `cloneElement` of a slotted child is structurally
impossible. It must be fixed where the React tree is built.

## Proposal: render an anchor wrapper instead of cloning the child

For every trigger-based overlay, render the trigger handlers and the
positioning `ref` on a **wrapper element the component owns**, rather than
cloning the child:

```jsx
// before
return cloneElement(child, { ref: anchorRef, onPointerEnter: open, onPointerLeave: close });

// after — works whether `children` is a React element OR slotted HTML
return (
  <span
    class="tooltip-anchor"
    ref={anchorRef}
    onPointerEnter={open}
    onPointerLeave={close}
    onFocusCapture={open}
    onBlurCapture={close}
  >
    {children}
  </span>
);
```

The wrapper:

- works for the React-native consumer (children is an element) **and** the
  web-component consumer (children is slotted HTML rendered via the bridge);
- gives the floating layer a stable, real anchor element to measure against —
  removing the dependency on a forwarded `ref` the slotted child can't provide;
- is a `display: contents` span when no layout box is wanted, so it does not
  disturb the trigger's own styling/box. (Note `display: contents` elements do
  not receive pointer events, so the anchor needs a real box — e.g.
  `display: inline-block` / `inline-flex` — or the listeners must sit on the
  floating-UI reference element. This is the one design choice to confirm.)

If preserving the exact current DOM for React consumers matters, gate it:
clone when `isValidElement(children)`, otherwise render the wrapper. The
web-component bridge always hits the wrapper branch.

## Affected elements

`tooltip`, `hover-card`, `popover`, `context-menu`. The `open`-driven overlays
(`modal`, `drawer`, `confirm-dialog`, `command-palette`) are **not** affected —
they take an `open` boolean attribute plus json content props and do not depend
on a slotted element trigger, so they are wrappable today (modulo their
`onClose`/`onConfirm` callbacks, which the wrapper handles as DOM events).

## Impact on `hmi-dioxus`

Until this is addressed upstream, `HmiTooltip`/`HmiPopover`/`HmiHoverCard`/
`HmiContextMenu` cannot be shipped as thin wrappers — they would compile and
bind their attributes but never open. They are deferred in [`../TODO.md`](../TODO.md)
with a pointer to this document. Once an anchor-wrapper (or equivalent) lands
upstream, wrapping them is the usual attribute-mapping exercise.
