---
name: triage-unwrappable-component
description: Use when an hmi-components wrapper compiles and binds its attributes but doesn't actually work at runtime — the element renders but an interaction (hover/click/open) does nothing, or content never appears. Covers detecting structural web-component limitations (e.g. React.cloneElement triggers across slotted children), confirming the element is inert, and the protocol for deferring it: remove the wrapper, write an upstream proposal, mark it ⛔ in TODO.md. Triggers: "the wrapper renders but the tooltip/popover/menu never opens", "compiles but doesn't do anything", "is this component even wrappable".
---

# Triaging an hmi-component that can't be thin-wrapped

A wrapper that passes `cargo build` + `clippy` + `dx check` can still be **inert
at runtime**: the custom element mounts, attributes bind, but the behaviour
never happens. Always verify in the running demo (see
`.claude/component-workflow.md` step 5) — a green build is not proof a component
works. This skill is the protocol for when that verification shows it does not,
and the cause is structural rather than a bug you can fix in the wrapper.

## 1. Recognise the smell

Suspect a structural limitation when the element:

- needs a **slotted child as its trigger** (you pass the trigger via
  `{children}`) and the behaviour is interaction-driven (hover, click,
  right-click, focus); or
- needs a **live framework callback / render-prop child** that can't cross the
  custom-element boundary.

The web-component bridge (`@r2wc/react-to-web-component`) captures slotted
children as an **HTML string** (`this.innerHTML` at connect), not a React
element. Any upstream component that does
`React.cloneElement(child, { onX, ref })` on its trigger therefore silently
gets nothing wired — the clone has no real element to attach to.

## 2. Confirm against the bundle (cheap, do this first)

Grep the vendored bundle for the trigger pattern:

```bash
grep -oE 'cloneElement\([a-zA-Z],\{[^}]{0,60}' assets/vendor/hmi-components.iife.js
grep -oE 'tt\("<name>",[A-Za-z0-9_$]+,\{[^}]*\}' assets/vendor/hmi-components.iife.js
```

If the element's content/trigger comes from children (no `open` boolean, no
content json prop for the part you need) **and** a `cloneElement(child, {on…})`
exists, it is almost certainly inert through the bridge. Contrast: elements
driven by an `open:"boolean"` attribute + json content props (`modal`, `drawer`,
`confirm-dialog`, `command-palette`) do **not** have this problem.

## 3. Confirm at runtime (decisive)

Drive the demo headless and assert the behaviour's DOM result is absent. For an
overlay, hover/click the trigger and check no portal node appears — and dispatch
the event directly so a headless-hover miss can't be mistaken for a real
failure:

```js
// after page load + waitForSelector('<host> button'):
await page.evaluate(() => {
  const host = document.querySelector('hmi-<name>');
  const fire = el => ['pointerenter','mouseenter','click','contextmenu']
    .forEach(t => el.dispatchEvent(new Event(t, { bubbles: true })));
  fire(host); fire(host.querySelector('button') || host);
});
// then assert: document.querySelector('.<name>, [role="tooltip"], [role="menu"]') === null
```

If the result node never appears after both real and synthetic events, and the
host's rendered subtree is just the slotted children (no wired anchor/ref), it
is inert. (Use `waitUntil: 'domcontentloaded'` + a settle delay, not
`networkidle0` — the dev server's hot-reload socket never goes idle.)

## 4. Handle it — defer, don't ship broken

Do **not** commit a wrapper that doesn't work, and don't ship it with a "may not
work" caveat. Instead:

1. **Remove the wrapper.** Delete `src/components/<name>.rs` and revert its
   `mod.rs` / `lib.rs` / demo wiring so the tree is clean.
2. **Write an upstream proposal** in `docs/` (match the style of the existing
   ones, e.g. `docs/slotted-trigger-overlays.md`): audience + method note,
   TL;DR table of affected elements, the exact bundle evidence, a reproduction,
   and a concrete minimal upstream fix. List **every** element with the same
   root cause, not just the one you hit.
3. **Mark it blocked in `TODO.md`** with the `⛔` marker (not `[ ]`, which means
   "not done yet") and a link to the proposal. Keep the wrapped count honest —
   blocked ≠ wrapped.
4. **Commit** the proposal + TODO together, e.g.
   `docs: defer <name> (blocked on upstream slotted-trigger support)`.

## 5. Reuse the proposal

If a later component shares an already-documented root cause, extend the
existing proposal's affected-elements table and add the `⛔` marker — don't write
a second document. When upstream ships the fix, the proposal becomes the
implementation checklist: wrapping the deferred elements is then the usual
attribute-mapping exercise.
