# Code Review Guidelines

## What to check

<!-- TODO: Delete or adapt this whole "Styling" section to match the project's styling approach. -->
### Styling
- Component styles live in `{{STYLED_DIR}}` (e.g. `{{STYLED_FILE_EXAMPLE}}`), not inline `style={}`
- New color values use design tokens (`var(--{{TOKEN_EXAMPLE}})`) rather than hardcoded values
- Shape tokens: {{SHAPE_TOKENS}} — never hardcoded radii
- Elevation tokens: {{ELEVATION_TOKENS}} — never hardcoded box-shadows
- Sizing units: `{{SIZE_UNIT}}` relative to the `{{BASE_FONT_SIZE}}` base

### Code quality
- No `any` unless genuinely unavoidable; prefer narrowing the type
- {{LINTER}} passes without suppression comments: `{{LINT_CMD}}`
- No `console.log` left in production paths (API routes, layout guards)
