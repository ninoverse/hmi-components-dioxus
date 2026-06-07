# Testing Requirements

There is currently no test suite configured in this repository. Until one is added, the following manual verification steps apply.

## Before merging any change

- [ ] `{{BUILD_CMD}}` succeeds (catches import boundary violations at compile time)

## If/when a test framework is added

<!-- TODO: Pick a test stack. -->
Recommended stack: **{{UNIT_TEST_FRAMEWORK}}** (unit) + **{{E2E_TEST_FRAMEWORK}}** (e2e).

- Unit tests belong in a `__tests__/` sibling to the file under test, or colocated as `*.test.ts`
- E2e tests go in `e2e/`
<!-- TODO: Document auth / integration testing strategy if applicable. -->
