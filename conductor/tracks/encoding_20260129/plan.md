# Implementation Plan - Support non-Korean GRF Encodings

## Phase 1: Encoding Configuration [checkpoint: 1971e11]
- [x] Task: Add Encoding Setting
    - [ ] Update `korangar/src/settings/mod.rs` (or relevant settings file) to include an `archive_encoding` field.
    - [ ] Default it to "euc-kr" but allow "windows-1252".
- [x] Task: Implement Encoding Logic in NativeArchive 7872e21
    - [ ] Modify `korangar/src/loaders/archive/native/mod.rs` to use the configured encoding when decoding file names.
    - [ ] Use `encoding_rs` to handle the decoding.
- [ ] Task: Conductor - User Manual Verification 'Encoding Configuration' (Protocol in workflow.md)
