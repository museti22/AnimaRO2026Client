# Track Specification: Support non-Korean GRF Encodings

## 1. Overview
The current implementation of `korangar` assumes that file paths within GRF archives are encoded in EUC-KR (Korean). AnimaRO, like many international private servers, uses Windows-1252 (Western European) or other encodings for its assets. This mismatch leads to file path resolution failures and potential crashes.

## 2. Goals
- Allow the client to read GRF files encoded in Windows-1252 (and potentially others).
- Provide a configuration option or auto-detection to select the correct encoding.

## 3. Scope
- **In Scope:**
    - `korangar/src/loaders/archive` module.
    - `korangar/src/settings` to add the configuration.
- **Out of Scope:**
    - Changing the encoding of the GRF files themselves.

## 4. Proposed Solution
- Add an `encoding` option to `sclientinfo.xml` or `game_settings.ron`.
- Update `NativeArchive` to use this encoding when decoding file names from the GRF table.
- Default to `Windows-1252` for AnimaRO if `EUC-KR` fails, or make it configurable.

## 5. Acceptance Criteria
- The client can successfully load `prontera` and other resources from the AnimaRO `data.grf` without crashing.
- Characters like `¾ÆÀÌÅÛ` (Mojibake for Korean in Latin1) are handled correctly or mapped to the correct paths.
