# Implementation Plan - Core UI Systems Implementation

## Phase 1: Asset Preparation & Theming
- [x] Task: Define UI Theme Constants 69afe02
    - [ ] Create `GothicTheme` struct in `korangar-interface` implementing the theme traits.
    - [ ] Define color palette (Obsidian, Antique Gold, etc.) in `state/theme/interface.rs`.
- [x] Task: Create/Import Placeholder Assets 46b12f3
    - [ ] Create placeholder textures for window backgrounds, buttons, and icons if final assets are missing.
    - [ ] Register new assets in `TextureLoader`.
- [ ] Task: Conductor - User Manual Verification 'Asset Preparation & Theming' (Protocol in workflow.md)

## Phase 2: Character Progression UI
- [ ] Task: Implement Status Window
    - [ ] Create `interface/windows/status.rs`.
    - [ ] Bind stat values (Str, Agi, etc.) to `ClientState`.
    - [ ] Implement "Add Stat" buttons sending `IncreaseAttribute` packets.
- [ ] Task: Implement Skill Tree Window
    - [ ] Create `interface/windows/skill_tree.rs`.
    - [ ] Implement visual tree layout based on `SkillTreeLoader`.
    - [ ] Implement skill learning logic (packet sending).
- [ ] Task: Conductor - User Manual Verification 'Character Progression UI' (Protocol in workflow.md)

## Phase 3: Economy UI
- [ ] Task: Implement Inventory & Equipment Window
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
