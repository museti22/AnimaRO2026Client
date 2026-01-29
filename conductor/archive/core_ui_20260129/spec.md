# Track Specification: Core UI Systems Implementation

## 1. Overview
This track aims to implement the remaining core User Interface (UI) systems required for a functional Ragnarok Online client, specifically tailored for AnimaRO. This includes Character Progression (Skills/Stats), Economy (Shops/Storage), and Social (Chat/Party/Guild) interfaces. All new UI elements must strictly adhere to the project's "Gothic-Fantasy" aesthetic.

## 2. Goals
- Achieve feature parity with standard RO clients for core interaction windows.
- Implement a modular, data-driven UI architecture using `rust-state`.
- Ensure all UI assets align with the AnimaRO visual identity (Obsidian/Gold/Dark).

## 3. Scope
### In Scope
- **Character Progression UI:**
    - Skill Tree Window (Visualizing dependencies, learning skills).
    - Status/Attribute Window (Distributing stats, viewing parameters).
- **Economy UI:**
    - Buying/Selling Store Interfaces.
    - Storage/Warehouse Interface.
    - Equipment Window.
- **Social UI:**
    - Chat Window (Tabs, filters, private messaging).
    - Party/Guild Organization Windows.
    - Basic Emote selector.
- **Asset Creation:**
    - Creation/Integration of new UI textures/sprites fitting the Gothic-Fantasy theme.

### Out of Scope
- Server-side mechanics logic (Pre-Renewal vs Renewal calculations).
- Advanced/Custom server features not present in standard clients.
- 3D World rendering changes (focus is strictly 2D UI).

## 4. Functional Requirements
- **Theme Enforcement:** All windows must use the theme system defined in `korangar-interface` to allow for easy reskinning and strict adherence to the style guide.
- **State Synchronization:** UI state must be synchronized with the `ClientState` using `rust-state` selectors.
- **Responsiveness:** UI must scale correctly with resolution and support drag/drop where applicable.
- **Networking:** UI actions (clicking 'Buy', learning a skill) must send the correct packets using `korangar-networking`.

## 5. Non-Functional Requirements
- **Performance:** UI rendering should not significantly impact frame rates.
- **Maintainability:** Code should be split into independent components within `korangar/src/interface/windows`.

## 6. Acceptance Criteria
- All listed windows can be opened, interacted with, and closed.
- Visual style matches the "Gothic-Fantasy" guidelines (verified via screenshots).
- "Buy", "Sell", "Level Up Skill", and "Add Stat" actions trigger correct network packets.
- Code compiles without warnings and passes `clippy` checks.
