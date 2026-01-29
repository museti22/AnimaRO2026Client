# Track Specification: Fix Debug Build Hang & Optimize Startup

## 1. Overview
The client currently hangs or takes an excessive amount of time to start in `debug` mode due to the `calculate_hash` function processing all game assets (which can be gigabytes in size). This tracks aims to resolve this by optimizing or skipping the hash calculation during development.

## 2. Goals
- Reduce startup time in debug mode to under 5 seconds.
- Prevent the "stuck" state reported during `create game file loader`.

## 3. Scope
- **In Scope:**
    - Modify `korangar/src/loaders/gamefile/mod.rs` to optimize `calculate_hash`.
    - Implement a caching mechanism or a debug-only skip for hashing.
- **Out of Scope:**
    - Changes to the asset format itself.

## 4. Proposed Solution
- Introduce a `fast_hash` or `skip_hash` mechanism when the `debug` feature is active.
- Ideally, only hash the file headers or metadata in debug mode, or rely on file modification times.

## 5. Acceptance Criteria
- `cargo run --release --features debug` starts the client successfully and quickly.
- `cargo run --release` (production) still performs necessary integrity checks.
