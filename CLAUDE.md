# CLAUDE.md

This file provides strict guidance and architectural rules for Claude Code (claude.ai/code) when working in this repository.

## Commands & Tooling

- **Package Manager:** You MUST strictly use `{{PACKAGE_MANAGER}}`. Never use other package managers in this repo.
- **Maintain the Build:** Never leave the codebase in a state where build or linting fails. Run the relevant commands below to verify your work before concluding a task.

<!-- TODO: Replace this code block with the commands actually used in this repo. -->
```bash
{{INSTALL_CMD}}     # Install dependencies
{{DEV_CMD}}         # Start dev server
{{BUILD_CMD}}       # Production build
{{LINT_CMD}}        # Lint check
{{FORMAT_CMD}}      # Format with auto-write
{{TEST_CMD}}        # Run tests (delete this line if not applicable)
```

## Architecture & Framework Rules

<!-- TODO: Describe the framework / runtime constraints for this project. -->
**Framework:** This project strictly uses {{FRAMEWORK}}.

<!-- TODO: Delete this whole "Styling Conventions" block if the project has no design system. -->
### Styling Conventions

- **Design Tokens:** CSS custom properties follow the `{{DESIGN_TOKEN_PREFIX}}` naming scheme (e.g. `var(--{{TOKEN_EXAMPLE}})`). Active theme CSS lives in `{{THEME_CSS_PATH}}`.
- **Typography & Scaling:** Base font size is `{{BASE_FONT_SIZE}}`; `rem` units scale from this base. Available font tokens: {{FONT_TOKENS}}.

## Behavioral Guidelines

**Tradeoff:** Bias toward caution over speed. For trivial tasks, use judgment.

### 1. Think Before Coding

**Don't assume. Don't hide confusion. Surface tradeoffs.**

- State your assumptions explicitly. If uncertain, stop and ask.
- If multiple interpretations exist, present them - don't pick silently.
- If a simpler approach exists, propose it. Push back when warranted.

### 2. Simplicity First

**Minimum code that solves the problem. Nothing speculative.**

- No features beyond what was asked. No abstractions for single-use code.
- No "flexibility" or error handling for impossible scenarios.
- If you write 200 lines and it could be 50, rewrite it.

### 3. Surgical Changes

**Touch only what you must. Clean up only your own mess.**

- Don't "improve" adjacent code, comments, or formatting.
- Match existing style exactly.
- Remove imports/variables/functions that YOUR changes made unused. Don't remove pre-existing dead code unless asked.

### 4. Goal-Driven Execution

**Define success criteria. Loop until verified.**

- Transform tasks into verifiable goals (e.g., "Add validation" → "Write tests for invalid inputs, then make them pass").
- For multi-step tasks, state a brief plan and verify each step independently.

---

## Extended Rules (Read Before Acting)

Use your file-reading capabilities to read the exact rules in the `.claude/` directory **before** executing any of the following tasks:

- **Committing code:** Read `.claude/commit-conventions.md`
- **Creating branches:** Read `.claude/branch-naming.md`
- **Reviewing PRs:** Read `.claude/code-review.md`
- **Testing/Verifying:** Read `.claude/testing-requirements.md`
- **Opening PRs:** Read `.claude/pr-guidelines.md`
- **Creating new files:** Read `.claude/file-naming.md`
- **Building a component:** Read `.claude/component-workflow.md`
- **Deciding what to build next / branching strategy:** Read `.claude/execution-order.md`
