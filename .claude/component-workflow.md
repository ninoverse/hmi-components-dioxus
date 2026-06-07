# Component Workflow

The exact procedure for building or modifying a single Dioxus component.
Follow every step in order; do not skip or reorder.

---

## Pre-flight

Before writing any code:

1. **Ask for confirmation.** State which component you are about to build and wait
   for explicit approval. Do not start on your own initiative.

2. **Check if the file already exists:**
   ```bash
   ls src/components/<name>.rs 2>/dev/null && echo EXISTS || echo MISSING
   ```
   If it exists, report the finding and ask: skip / overwrite / modify.
   Never silently overwrite.

---

## 6-step checklist (one component, one commit)

Complete all six steps before committing. Never commit a partial component.

### 1. `src/components/<name>.rs`

- Filename: `snake_case.rs`.
- Define a `PascalCase` function annotated with `#[component]`, returning
  `Element` via `rsx!`. Mark it `pub`.
- Props: declare inline as function arguments. Move a `Props` struct to
  `src/models/<name>.rs` only if the type is shared across multiple components.
- Component-scoped CSS: declare `const <NAME>_CSS: Asset = asset!("/assets/styling/<name>.css");`
  and inject `document::Stylesheet { href: <NAME>_CSS }` as the first node in `rsx!`.
- All values via design tokens (`var(--color-primary)`) — no hardcoded colors, radii, or shadows.
- Sizing: `rem` units. Remember `1rem = 16px` at the project base.

### 2. `assets/styling/<name>.css`

- Use only project design tokens (`var(--color-*)`, `var(--radius-*)`, `var(--elevation-*)`).
- Shape tokens: `var(--radius-sm|md|lg)` — never hardcoded radii.
- Elevation tokens: `var(--elevation-1|2)` — never hardcoded box-shadows.
- Class names: BEM-style, scoped to the component:
  `.<name>`, `.<name>__part`, `.<name>--modifier`.

### 3. `src/components/mod.rs`

Add `mod <name>;` and a `pub use <name>::<ComponentName>;` re-export.
Keep both lists **alphabetical**.

### 4. `src/main.rs`

Render the component in `App` (or the relevant parent) with **at least one
variant per meaningful prop combination**. Every prop that changes visual
output must be exercised — type errors and render failures surface at
`cargo check` / `dx serve`.

```rust
Hero { title: "Primary", subtitle: "Default state" }
```

### 5. Visual check

Start the dev server (`dx serve --platform web`) and confirm the component
renders correctly. Take a screenshot using the method below and present it for
review before committing. For desktop/mobile, swap `--platform`.

### 6. Commit, push, PR

```
feat(ui): add <ComponentName> component
```

One commit per component. Never batch multiple components in one commit.

- Push the commit to the current group branch.
- If this is the **group's first commit**: open a draft PR immediately.
- If the draft PR already exists: just push to it.
- **Stop.** Ask before starting the next component.

---

## Group verification gate

Run before marking any group PR ready for review:

```bash
cargo clippy --all-targets -- -D warnings   # zero warnings, zero errors
dx check                                     # rsx! macros validate
dx build --release --platform web            # production build succeeds
```

Confirm that `src/components/mod.rs` `mod` and `pub use` lines are
**alphabetically sorted and consistent** with the files on disk.

---

## Screenshot method

<!-- One working example for sandboxed Linux containers. Adapt to your environment. -->

Works without widening the container's egress policy. `/tmp` is wiped on
container reset — re-run the install at the start of each session.

```bash
mkdir -p /tmp/shot && cd /tmp/shot && npm init -y
npm i @sparticuz/chromium puppeteer-core
```

Driver `/tmp/shot/shot.js`:

```js
const chromium = require('@sparticuz/chromium');
const puppeteer = require('puppeteer-core');

(async () => {
    const browser = await puppeteer.launch({
        args: [...chromium.args, '--no-sandbox', '--disable-dev-shm-usage'],
        executablePath: await chromium.executablePath(),
        headless: true,
        defaultViewport: { width: 1400, height: 1000, deviceScaleFactor: 2 },
    });
    const page = await browser.newPage();
    await page.goto('http://localhost:8080/', { waitUntil: 'networkidle0' });
    await page.screenshot({ path: 'out.png', fullPage: true });
    await browser.close();
})();
```

With `dx serve --platform web` running: `node /tmp/shot/shot.js`
