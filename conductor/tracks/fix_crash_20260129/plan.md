# Implementation Plan - Fix Map Loading Crash

## Phase 1: Diagnostic Logging
- [x] Task: Instrument NativeArchive with Debug Logging
    - [ ] Add `print_debug!` statements in `korangar/src/loaders/archive/native/mod.rs` inside `get_file_by_path`.
    - [ ] Log `asset_path`, `file_information.offset`, `file_information.compressed_size`, and `file_information.uncompressed_size`.
    - [ ] Log the first 16 bytes of the `compressed_file_buffer` after decryption.
- [ ] Task: Verify File Identity
    - [ ] Cross-reference the logged offsets/sizes with a GRF tool (like GRF Editor) for the `prontera.rsw` or other map files.
- [ ] Task: Conductor - User Manual Verification 'Diagnostic Logging' (Protocol in workflow.md)

## Phase 2: Fix & Verification [checkpoint: 0f9431c]
- [x] Task: Implement Robust Decryption/Decompression 0f9431c
    - [ ] Based on Phase 1 logs, determine if `mixcrypt` is failing or if the offset is wrong.
    - [ ] If offset is wrong: Fix path normalization/encoding in `from_path`.
    - [ ] If decryption is failing: Refactor `mixcrypt.rs` to support the specific variant used in the GRF.
- [ ] Task: Conductor - User Manual Verification 'Fix & Verification' (Protocol in workflow.md)
