# Implementation Plan - Fix Debug Build Hang

## Phase 1: Optimization
- [x] Task: Modify GameFileLoader
    - [ ] Update `calculate_hash` in `korangar/src/loaders/gamefile/mod.rs` to check for `cfg!(feature = "debug")`.
    - [ ] In debug mode, return a constant dummy hash or a fast hash of just the file names/sizes to avoid reading the entire GRF content.
- [ ] Task: Conductor - User Manual Verification 'Optimization' (Protocol in workflow.md)
