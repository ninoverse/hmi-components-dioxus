# Controlled form controls — required behaviour

A spec for **`@ninoverse/hmi-components`**: how `<hmi-input>`, `<hmi-switch>`,
`<hmi-checkbox>` (and the other text/toggle controls — `textarea`, `radio`,
`password-input`, `search-input`) must behave so a host framework can bind them
**controlled**.

This crate needs controlled binding. It can't be done from the wrapper side —
the behaviour below has to live in the web components.

---

## The contract (one sentence)

> The control always displays exactly the value the host last gave it. The user
> can never change that value directly — a user action only *asks* the host to
> change it, by firing an event. The host updates the value, and the new value
> flows back down.

That one rule is what makes **clear the field**, **reset the form**, and
**uncheck on cancel** work: the host is always in charge of the value.

```
        host sets value/checked
host state ───────────────────────▶  <hmi-input> shows it, exactly
    ▲                                      │
    └──────── change event (detail) ◀──────┘
        "the user wants this value"
        (the host decides whether to apply it)
```

---

## Required behaviour

### 1. Display equals the prop, exactly

Whatever `value` / `checked` is set to is what renders. There is no separate
internal "current value" that can drift away from the prop.

### 2. The value can be set at any time — by attribute or property — and empty / false count

All of these must take effect immediately:

```js
el.setAttribute('value', 'hi');   // shows "hi"
el.setAttribute('value', '');     // shows ""  (cleared)        ← must work
el.value = 'hi';                   // same effect, via property
el.checked = false;                // unchecks                  ← must work
el.setAttribute('checked', 'false'); // unchecks
```

Two things are missing today and must be fixed:

- **Empty string and `false` are real values, not "no value."** Right now an
  empty or `false` update is *ignored*, so a controlled input can't be cleared
  and a controlled checkbox can't be unchecked. This is the main fix.
- **A value set before the element upgrades must not be lost.** If a framework
  sets `el.value` / `el.checked` *before* the component's script has defined the
  element, that value must still apply once it upgrades (the standard
  "upgrade properties" pattern). Frameworks like Dioxus set these as DOM
  *properties*, so the property path must be exactly as reliable as the
  attribute path.

### 3. A user action is reported, never self-applied

On a keystroke or toggle, dispatch a **bubbling** event carrying the value the
user intended — and do **not** change the displayed value as a result. The
control keeps showing the prop until the host sets a new one.

| Control | Event | `detail` |
|---|---|---|
| input · textarea · password-input · search-input | `change` | new text (`string`) |
| switch · checkbox · radio | `change` | new checked (`bool`) |

(For a checkbox: when the user clicks, fire `change` with `detail: true/false`,
but leave the box at its controlled state — the host flips it by updating
`checked`.)

### 4. Uncontrolled mode stays available

If the host sets only `default-value` / `default-checked` and never the
controlled `value` / `checked`, the control owns its own value (as it does
today). The host picks the mode by choosing which prop it sets — controlled
(`value`/`checked`) or uncontrolled (`default-*`).

---

## What 4.2.0 already does, and what's left

- ✅ Fires a bubbling `change` event with the value in `detail` (rule 3).
- ✅ Has both `value`/`checked` and `default-value`/`default-checked` (rule 4).
- ❌ **Ignores empty and `false`** updates — the attribute handler only applies a
  change when the new value is truthy, so `value=""` and `checked="false"`
  (or removing the attribute) are dropped. Breaks rule 2.
- ❌ A `value` / `checked` **property** set before upgrade is shadowed and lost.
  Breaks rule 2 for property-driven frameworks.

Fixing those two points is the whole job; the event side is done.

---

## Acceptance checklist

A control is controlled-ready when every line holds:

- [ ] Set `value="x"`, then `value=""` → shows `x`, then empty.
- [ ] Set `checked="true"`, then `checked="false"` → checks, then unchecks.
- [ ] Setting the value via the JS **property** has the same effect as the attribute.
- [ ] Setting the value **before** the element's script loads still shows once upgraded.
- [ ] Typing / toggling fires a bubbling `change` with the new value in `detail`,
      and the displayed value does **not** change until the host sets it.

When these pass, this crate can bind `value`/`checked` two-way and drop the
uncontrolled `default-*` workaround.
