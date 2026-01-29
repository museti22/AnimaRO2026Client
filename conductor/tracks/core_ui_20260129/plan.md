# Implementation Plan - Core UI Systems Implementation

## Phase 1: Asset Preparation & Theming [checkpoint: bd26162]
- [x] Task: Define UI Theme Constants 69afe02
    - [ ] Create `GothicTheme` struct in `korangar-interface` implementing the theme traits.
    - [ ] Define color palette (Obsidian, Antique Gold, etc.) in `state/theme/interface.rs`.
- [x] Task: Create/Import Placeholder Assets 46b12f3
    - [ ] Create placeholder textures for window backgrounds, buttons, and icons if final assets are missing.
    - [ ] Register new assets in `TextureLoader`.
- [ ] Task: Conductor - User Manual Verification 'Asset Preparation & Theming' (Protocol in workflow.md)

## Phase 2: Character Progression & Storage UI [checkpoint: a7c6642]
- [x] Task: Audit Status Window 69afe02
- [x] Task: Improve Skill Tree Window 50944f4
    - [ ] Update `interface/windows/skill_tree.rs` to use `row` and `col` positions from `LocalSkillEntry`.
    - [ ] Implement visual connectors between skill dependencies.
- [x] Task: Implement Storage (Warehouse) Window 5cf92af
    - [ ] Create `interface/windows/storage.rs`.
    - [ ] Bind storage items to `ClientState`.
    - [ ] Implement drag/drop between inventory and storage.
- [ ] Task: Conductor - User Manual Verification 'Character Progression & Storage UI' (Protocol in workflow.md)

## Phase 3: Economy UI
- [x] Task: Implement Inventory & Equipment Window db1738a
    - [ ] Refactor existing inventory if necessary to support new drag/drop features.
    - [ ] Implement Equipment slots with visual feedback.
- [ ] Task: Implement Shop Interface
    - [ ] Create `interface/windows/shop.rs`.
    - [ ] Implement Buy/Sell logic and packet handling.
- [ ] Task: Conductor - User Manual Verification 'Economy UI' (Protocol in workflow.md)

## Phase 4: Social UI & Finalization
- [ ] Task: Implement Chat Window Enhancements
    - [ ] Add support for tabs and channel filtering.
    - [ ] Implement private messaging UI.
- [ ] Task: Implement Party/Guild Windows
    - [ ] Create basic list views for party/guild members.
- [ ] Task: Conductor - User Manual Verification 'Social UI & Finalization' (Protocol in workflow.md)
