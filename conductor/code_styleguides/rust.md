# Rust Code Style Guide

## Formatting Standards (rustfmt)
This project enforces strict formatting rules via `rustfmt`. Ensure your editor is configured to run `cargo fmt` on save.

- **Edition:** 2024
- **Max Width:** 140 characters
- **Imports:** Grouped by `StdExternalCrate` and merged at the `Module` level.
- **Hex Literals:** Upper case (e.g., `0xFF`).
- **Comments:** Wrapped to fit the width.

## Core Philosophy

1.  **Safety First**: `unsafe` is **forbidden** unless explicitly requested. If used, it MUST be wrapped in a `// SAFETY:` comment explaining why it is safe.
2.  **Expression-Oriented**: Leverage Rust as an expression language.
    - _Bad_: `let mut x = 0; if condition { x = 1; } else { x = 2; }`
    - _Good_: `let x = if condition { 1 } else { 2 };`
3.  **Type-Driven Design**: Make invalid states unrepresentable. Use `enum`s to encode state machines instead of booleans or loose strings.

## Idiomatic Patterns

### Error Handling
- **Libraries**: Use `thiserror` for library crates (`korangar-*`, `ragnarok-*`).
  ```rust
  #[derive(thiserror::Error, Debug)]
  pub enum MyError {
      #[error("IO failed: {0}")]
      Io(#[from] std::io::Error),
      #[error("Invalid data: {0}")]
      InvalidData(String),
  }
  ```
- **Applications**: Use `anyhow::Result` for the main application logic (`korangar`).

### Iterators vs Loops
- Prefer `Iterator` combinators (`map`, `filter`, `fold`) over explicit `for` loops for transformation and filtering tasks.
  ```rust
  // Good
  let results: Vec<_> = items.iter()
      .filter(|i| i.is_valid())
      .map(|i| i.process())
      .collect();
  ```

### Option & Result Combinators
- Use `map`, `and_then`, `unwrap_or_else` to handle `Option` and `Result` types.
- Avoid excessive nesting of `if let`.

## Project Strictness
- **Async/Await**: Use `tokio` as the default runtime.
- **Visibility**: All struct fields are private by default. Use `pub(crate)` for internal sharing and `pub` only for the public API surface.
- **Modules**: Keep `main.rs` minimal. Logic resides in `lib.rs` or specific submodules.
