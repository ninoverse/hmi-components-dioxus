# Claude Code Template

Reusable Claude Code configuration scaffolding. Copy `CLAUDE.md` and `.claude/`
into the root of any new repository, then fill in the placeholders.

## Usage

1. Copy the template files into your repo root:

   ```bash
   cp /path/to/claude-template/CLAUDE.md  /path/to/your-repo/
   cp -r /path/to/claude-template/.claude /path/to/your-repo/
   ```

2. Find every placeholder and replace it:

   ```bash
   grep -rn "{{" CLAUDE.md .claude/
   grep -rn "<!-- TODO" CLAUDE.md .claude/
   ```

3. Delete sections that do not apply (for example, the Styling Conventions
   block if you have no design system, or the execution-order phase table if
   you don't run phased work).

## What each rule file covers

| File | Purpose |
|------|---------|
| `branch-naming.md` | Branch prefix and format conventions |
| `commit-conventions.md` | Conventional Commits rules |
| `pr-guidelines.md` | PR title, description template, size guidance |
| `testing-requirements.md` | Manual and automated test gates |
| `file-naming.md` | Directory layout and file naming conventions |
| `code-review.md` | Review checklist (styling + code quality) |
| `component-workflow.md` | Step-by-step procedure to add a component |
| `execution-order.md` | Branching strategy and (optional) phased build order |

## Placeholder Reference

Fill in any of these that appear in the template files. Delete `{{...}}` and
adjacent prose when a placeholder is irrelevant (for example, library-only
placeholders in app projects).

### Commands and tooling

| Placeholder | Meaning | Example |
|---|---|---|
| `{{PROJECT_NAME}}` | Human-readable project name | `my-design-system` |
| `{{PACKAGE_MANAGER}}` | Package manager binary | `pnpm`, `npm`, `yarn`, `bun` |
| `{{INSTALL_CMD}}` | Install dependencies | `pnpm install` |
| `{{DEV_CMD}}` | Start dev server | `pnpm dev` |
| `{{BUILD_CMD}}` | Production build | `pnpm build` |
| `{{LINT_CMD}}` | Lint check | `pnpm lint` |
| `{{FORMAT_CMD}}` | Format with auto-write | `pnpm format` |
| `{{TEST_CMD}}` | Run tests | `pnpm test` |
| `{{LINTER}}` | Linter name | `Biome`, `ESLint` |
| `{{FRAMEWORK}}` | Primary framework | `React 19`, `Vue 3`, `SvelteKit` |
| `{{DEV_PORT}}` | Local dev server port | `5173`, `3000` |

### Design system

| Placeholder | Meaning | Example |
|---|---|---|
| `{{DESIGN_TOKEN_SYSTEM}}` | Token system name | `Material Design 3`, `Tailwind`, `custom` |
| `{{DESIGN_TOKEN_PREFIX}}` | Token naming pattern | `MD3 short-name` |
| `{{TOKEN_EXAMPLE}}` | Example token | `primary`, `color-bg-primary` |
| `{{SHAPE_TOKENS}}` | Shape / radii token list | `` `var(--corner-tl)`, `var(--corner-tr)` `` |
| `{{ELEVATION_TOKENS}}` | Elevation token list | `` `var(--elevation-N)` `` |
| `{{FONT_TOKENS}}` | Font CSS variable list | `--font-inter`, `--font-mono` |
| `{{BASE_FONT_SIZE}}` | Root font-size for rem | `8px`, `16px` |
| `{{SIZE_UNIT}}` | Preferred sizing unit | `rem`, `px` |
| `{{THEME_CSS_PATH}}` | Where theme CSS lives | `public/css/themes/` |

### File layout

| Placeholder | Meaning | Example |
|---|---|---|
| `{{COMPONENT_DIR}}` | Folder for components | `src/components` |
| `{{STYLED_DIR}}` | Folder for component styles | `src/components/styled` |
| `{{STYLED_FILE_EXAMPLE}}` | Example styled file path | `src/components/styled/button.styled.css` |
| `{{MODELS_DIR}}` | Folder for shared types | `src/models` |
| `{{COMPONENT_EXT}}` | Component file extension | `tsx`, `vue`, `svelte` |
| `{{FILENAME_CONVENTION}}` | Filename casing rule | `camelCase.tsx` |
| `{{COMPONENT_NAMING}}` | Component identifier casing | `PascalCase name` |
| `{{INDEX_FILE}}` | Barrel export entry | `src/index.ts` |
| `{{BUILD_CONFIG_FILE}}` | Build config path | `vite.config.ts`, `rollup.config.ts` |
| `{{APP_ENTRY_FILE}}` | Demo / host app file | `src/App.tsx` |
| `{{DIST_DIR}}` | Build output directory | `dist` |

### Testing and phases

| Placeholder | Meaning | Example |
|---|---|---|
| `{{UNIT_TEST_FRAMEWORK}}` | Unit test framework | `Vitest`, `Jest` |
| `{{E2E_TEST_FRAMEWORK}}` | E2E framework | `Playwright`, `Cypress` |
| `{{PHASE_N_CATEGORY}}` (numbered per row, e.g. `{{PHASE_1_CATEGORY}}`) | Phase row category | `Layout`, `Form primitives` |
| `{{PHASE_N_COMPONENTS}}` (numbered per row, e.g. `{{PHASE_1_COMPONENTS}}`) | Phase row components | `box, flex, grid` |
