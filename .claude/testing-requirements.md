# Testing Requirements

## Before merging any change

- [ ] `cargo clippy --all-targets -- -D warnings` is clean (warnings are errors)
- [ ] `dx check` passes (catches `rsx!` macro errors)
- [ ] `cargo test` passes
- [ ] `dx build --release --platform web` succeeds (catches asset/compile failures)
- [ ] For UI changes: verified visually via `dx serve` (screenshot in the PR)

## Writing tests

`cargo test` is the test runner; no extra framework is required.

- **Unit tests** for pure logic (utilities, model methods) go in a `#[cfg(test)] mod tests`
  block at the bottom of the file under test.
- **Integration tests** that exercise the public crate surface go in a top-level
  `tests/` directory, one file per area.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_empty_input() {
        assert_eq!(format_label(""), "—");
    }
}
```

Component render/interaction testing is not wired up in this template; rely on
`dx check` + the manual visual check until a renderer-level harness is added.
