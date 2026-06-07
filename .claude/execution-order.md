# Execution Order & Branching Strategy

Defines the priority order for building components and the branch / PR
structure that maps onto it.

---

## Branching and PR strategy

See `.claude/branch-naming.md` for the branch name format.

| Work type | Branch prefix | One PR per |
|-----------|--------------|-----------|
| Foundation scaffold | `chore/` | whole scaffold |
| Token update / re-theme | `chore/` | one PR |
| Component group (Phase N) | `feat/` | group (e.g. `feat/phase1-layout`) |
| Single isolated component | `feat/` | component |
| Rename / refactor | `refactor/` | logical rename unit |
| Docs / rules | `docs/` | one PR |

**Draft PR rule:** open a draft PR at the group's **first commit**. Push every
subsequent component commit to that same PR. Mark ready for review only when
`{{LINT_CMD}}` and `{{BUILD_CMD}}` both pass cleanly.

---

<!-- TODO: If this project plans phased builds, populate the phase table below.
     Otherwise delete this entire "Phase execution order" section and the
     "Current gap list" section that follows. -->

## Phase execution order

Build phases in ascending order. Do not start a later phase until all earlier
phases are merged to `main`.

| Phase | Category | Components | Dependency |
|-------|----------|-----------|------------|
| 0 | Reconciliation | Renames, moves, structural fixes | Do first — before any new builds |
| 1 | {{PHASE_1_CATEGORY}} | {{PHASE_1_COMPONENTS}} | Foundational |
| 2 | {{PHASE_2_CATEGORY}} | {{PHASE_2_COMPONENTS}} | After Phase 1 |

### Within each phase

- Build **one component at a time**.
- Follow the 9-step checklist in `.claude/component-workflow.md` for each.
- Stop and confirm with the user after each component before starting the next.
- Existing components in a phase get an **audit-pass** (review + screenshot);
  only commit if a real defect is found.

### Audit-pass checklist (existing components)

1. Open the component source file — check for hardcoded colors, radii, or shadows.
2. Confirm the component is imported and rendered in `{{APP_ENTRY_FILE}}`.
3. Take a screenshot (`{{DEV_CMD}}` + screenshot method in `component-workflow.md`).
4. Surface anything broken. Only commit if a fix is needed; use an isolated commit.

---

## Current gap list

<!-- TODO: Track outstanding work here, or delete this section. -->

### Adding new components

New components are no longer part of a phased roadmap — build each as a single
isolated component on its own `feat/<name>` branch off `main`, following the
9-step workflow in `.claude/component-workflow.md`.

Update this table if a new category of work is planned.
