# Portal-based overlays' buttons are dead as web components (modal, drawer, confirm-dialog, command-palette)

> **Audience:** maintainers of **`@ninoverse/hmi-components`** (the upstream npm
> package) and of this crate. This corrects an earlier assumption (see the note
> at the end) and explains why the `open`-driven overlay elements can't be thin
> web-component wrappers today.
>
> **Method note:** the analysis is against the **vendored compiled bundle**
> (`assets/vendor/hmi-components.iife.js`, v5.0.1) and runtime behaviour observed
> by driving the demo in a headless browser. The React→web-component bridge is
> [`@r2wc/react-to-web-component`](https://github.com/bitovi/react-to-web-component).

## TL;DR

The bridge mounts each element's React tree **into the custom-element host** and
roots React's event delegation there. These overlays then render their content
through a **React portal to `document.body`** — *outside* the host. Two things
break as a result:

1. **Action/close callbacks never fire.** A click on a portaled button
   (Confirm, Cancel, the `×` close, the backdrop) bubbles up the **DOM** to
   `document.body`, never reaching the host where React's delegated listener
   lives. So React's `onClick` never runs, and the bridge never dispatches the
   `confirm` / `cancel` / `close` `CustomEvent`. The dialog renders but its
   buttons are inert.
2. **`actions` can't be passed.** `modal` / `drawer` render their `actions`
   prop **as a React child node**, not from data. Passing the serialisable
   value a web-component attribute can carry (e.g. `[{label, value}]`) throws
   *"Objects are not valid as a React child"* (React error #31) and the element
   fails to render at all.

| Element | Open prop | Callback(s) | Status through the bridge |
|---|---|---|---|
| `modal` | `open` | `onClose` | `actions` crash (#31); close button portaled → `close` never fires |
| `drawer` | `open` | `onClose` | same as modal |
| `confirm-dialog` | `open` | `onConfirm`, `onCancel` | buttons portaled → neither event fires |
| `command-palette` | `open` | `onOpenChange` | portaled input/list → selection callbacks don't fire |

### Reproduction (verified)

- `confirm-dialog`: render it `open`, then click **Delete** (the confirm
  button) with a real trusted mouse click. Patching `EventTarget.dispatchEvent`
  globally shows **no** `confirm` / `cancel` / `close` `CustomEvent` is ever
  dispatched. The host element has **zero** children — the whole dialog is
  portaled to `document.body`.
- `modal`: injecting `<hmi-modal open actions='[{"label":"OK","value":"ok"}]'>`
  logs React error #31 and renders nothing; the host has zero children.

## Why this is a boundary problem, not a wrapper problem

A wrapper can only set attributes and listen for `CustomEvent`s on the host. It
can't change where r2wc roots React, where the component portals, or make the
portaled DOM bubble to the host. And it can't hand `actions` a live React node
across the boundary. So this must be fixed upstream.

## Proposal

- **Event delegation:** give the overlays a way to work when React is rooted at
  the custom element — e.g. render the portal into a node **inside** the host
  (or the host's shadow root) so portaled clicks still reach React's listener,
  or attach the action handlers with native DOM listeners rather than relying on
  React's synthetic delegation.
- **`actions`:** accept a **serialisable** action descriptor
  (`[{ label, value, variant }]`) and emit the chosen `value` via the existing
  `onClose` / a new `onAction` event, instead of requiring a React node. The
  same pattern the wrapped data-driven elements already use.

With both, the `open`-driven overlays become the usual attribute-mapping +
`open`-boolean + event-listener exercise.

## Impact on `hmi-dioxus`

`HmiModal`, `HmiDrawer`, `HmiConfirmDialog`, and `HmiCommandPalette` can't ship
as thin wrappers: they render but their buttons/selection never call back, so a
consumer can neither confirm, cancel, nor close them programmatically through
the events. They are deferred in [`../TODO.md`](../TODO.md) with a pointer here.

> **Correction:** an earlier note in
> [`slotted-trigger-overlays.md`](slotted-trigger-overlays.md) stated these
> `open`-driven overlays were "not affected … wrappable today." That was
> asserted without testing their portaled-button behaviour and is **wrong** —
> they are affected by the distinct portal/event-delegation mechanism described
> here.
