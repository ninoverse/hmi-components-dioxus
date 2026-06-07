# Commit Conventions

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification.

## Format

```
<type>(<scope>): <description>

[optional body]
```

- Subject line: max 72 characters, lowercase, no trailing period
- Use imperative mood: "add feature" not "added feature"
- Body: wrap at 72 characters, explain *why* not *what*

## Types

| Type | When to use |
|------|-------------|
| `feat` | New feature or user-visible behaviour |
| `fix` | Bug fix |
| `refactor` | Code change with no behaviour change |
| `style` | Formatting, whitespace — no logic change |
| `docs` | Documentation only |
| `chore` | Build scripts, deps, tooling, CI |
| `perf` | Performance improvement |
| `revert` | Reverts a previous commit |

Append `!` after the type for breaking changes: `feat!: drop Node 16 support`.

## Scopes (optional but recommended)

Use the route or layer being changed: `auth`, `api`, `ui`, `db`, `config`, `infra`.

## Examples

```
feat(ui): add dropdown component with keyboard navigation
fix(auth): redirect loop when session cookie is expired
refactor(api): extract pagination logic into utility
chore: upgrade linter to latest
docs: update CLAUDE.md with branching strategy
feat!: replace session cookie with JWT-only flow
```

## What to avoid

- Vague messages: `fix stuff`, `update`, `wip`
- Mixing unrelated changes in one commit
- Committing secrets or credentials (they are gitignored for a reason)
