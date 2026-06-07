# Component Workflow

The exact procedure for building or modifying a single component.
Follow every step in order; do not skip or reorder.

---

## Pre-flight

Before writing any code:

1. **Ask for confirmation.** State which component you are about to build and wait
   for explicit approval. Do not start on your own initiative.

2. **Check if the file already exists:**
   ```bash
   ls {{COMPONENT_DIR}}/<name>.{{COMPONENT_EXT}} 2>/dev/null && echo EXISTS || echo MISSING
   ```
   If it exists, report the finding and ask: skip / overwrite / modify.
   Never silently overwrite.

---

## 9-step checklist (one component, one commit)

Complete all nine steps before committing. Never commit a partial component.

### 1. `{{COMPONENT_DIR}}/<name>.{{COMPONENT_EXT}}`

- Filename: `{{FILENAME_CONVENTION}}`.
- Export a **named** function component ({{COMPONENT_NAMING}}).
- Props type: define in the same file; move to `{{MODELS_DIR}}/<name>.model.ts` only
  if the type is shared across multiple components.
- First import: side-effect CSS — `import './styled/<name>.styled.css';`  <!-- TODO: adapt or delete if the project uses CSS-in-JS / no CSS files. -->
- All values via design tokens (`var(--{{TOKEN_EXAMPLE}})`) — no hardcoded colors, radii, or shadows.
- Sizing: `{{SIZE_UNIT}}` units. Remember `1{{SIZE_UNIT}} = {{BASE_FONT_SIZE}}` at the project base.

### 2. `{{STYLED_DIR}}/<name>.styled.css`

<!-- TODO: Adapt token names to this project's design system, or delete if no design system. -->
- Use only project design tokens (`var(--{{TOKEN_EXAMPLE}})`, …).
- Shape tokens: {{SHAPE_TOKENS}}.
- Elevation tokens: {{ELEVATION_TOKENS}}.
- Class names: BEM-style, scoped to the component:
  `.<name>`, `.<name>__part`, `.<name>--modifier`.

### 3. `{{INDEX_FILE}}`

Add a named re-export. Keep exports **alphabetical**.

<!-- TODO: Delete this entire step if the project is not a library / has no build entry map. -->
### 4. `{{BUILD_CONFIG_FILE}}`

Add the component to the build entry map. Keep entries **alphabetical**.

```ts
<name>: resolve(dirname, '{{COMPONENT_DIR}}/<name>.{{COMPONENT_EXT}}'),
```

<!-- TODO: Delete this entire step if the project is not published as a library. -->
### 5. `package.json` — `"exports"`

Add a subpath entry. Key: kebab-case. Keep entries **alphabetical**.

```json
"./<kebab-name>": {
    "types": "./{{DIST_DIR}}/<name>.d.ts",
    "import": "./{{DIST_DIR}}/<name>.js"
}
```

### 6. `{{APP_ENTRY_FILE}}`

Import the component from `'./index'` and render **at least one variant per
meaningful prop combination**. Every prop that changes visual output must be
exercised — type errors, missing CSS, and render failures surface here.

```tsx
import { ExampleComponent } from './index';
// inside return:
<ExampleComponent variant="primary">Primary</ExampleComponent>
<ExampleComponent variant="ghost" size="small">Ghost SM</ExampleComponent>
```

### 7. Visual check

Start the dev server (`{{DEV_CMD}}`) and confirm the component renders correctly in
the browser. Take a screenshot using the method below. Present it for review
before committing.

### 8. Commit

```
feat(ui): add <ComponentName> component
```

One commit per component. Never batch multiple components in one commit.

### 9. Push + PR

- Push the commit to the current group branch.
- If this is the **group's first commit**: open a draft PR immediately.
- If the draft PR already exists: just push to it.
- **Stop.** Ask before starting the next component.

---

## Group verification gate

Run before marking any group PR ready for review:

```bash
{{LINT_CMD}}    # zero warnings, zero errors
{{BUILD_CMD}}   # build emits all new entries; no stale references
```

<!-- TODO: Delete this dist spot-check if not a library build. -->
Spot-check `{{DIST_DIR}}/` for every component added in the group:

```bash
ls {{DIST_DIR}}/<name>.js {{DIST_DIR}}/<name>.d.ts
```

Confirm that `{{INDEX_FILE}}` re-exports, `{{BUILD_CONFIG_FILE}}` entries, and
`package.json` exports are all **alphabetically sorted and consistent** with each
other.

---

## Screenshot method

<!-- TODO: Replace or delete this entire "Screenshot method" section to match your environment. The block below is one working example for sandboxed Linux containers. -->

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
    await page.goto('http://localhost:{{DEV_PORT}}/', { waitUntil: 'networkidle0' });
    await page.screenshot({ path: 'out.png', fullPage: true });
    await browser.close();
})();
```

With `{{DEV_CMD}}` running: `node /tmp/shot/shot.js`
