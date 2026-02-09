# Architecture Documentation: Korangar

Korangar is designed as a modular game engine tailored for Ragnarok Online. It separates concerns into distinct crates to ensure maintainability and reusability.

## System Architecture

### 1. Main Client (`korangar`)
The core orchestrator. It manages:
- **Game State**: Handling login, character selection, and world exploration.
- **Renderer**: Orchestrating the rendering pipeline using engine libraries.
- **World Simulation**: Managing entities, maps, and local physics.
- **Input & Interface**: Bridging user input with the UI and game world.

### 2. Interface System (`korangar-interface`)
A custom UI framework built for Korangar. It handles:
- **Layout**: Dynamic UI positioning.
- **Theming**: "Gothic-Fantasy" aesthetic (Obsidian/Gold).
- **Interactions**: Event handling for buttons, windows, and menus.

### 3. Graphics & Rendering (`wgpu`)
Korangar uses `wgpu` for a modern, hardware-agnostic rendering pipeline.
- **Shaders**: Custom WGSL shaders for lighting and effects.
- **Models**: RSM/RSW format loaders and renderers.

### 4. Networking (`korangar-networking` & `ragnarok-packets`)
- **Packet Handling**: Parsing and building binary packets for RO servers.
- **State Sync**: Keeping the client in sync with server-side game logic.

### 5. Loaders & Data (`korangar-loaders` & `ragnarok-formats`)
- **GRF Archive**: Efficiently reading assets from official data files.
- **Format Support**: Parsers for SPR (Sprites), ACT (Actions), MAP (Maps), and RSM (Models).

## Data Flow
1. **Input**: `winit` captures events → `korangar/src/input` processes them.
2. **Logic**: `korangar/src/state` updates the game world or UI state.
3. **Network**: `korangar-networking` sends/receives packets → `ragnarok-packets` decodes them.
4. **Rendering**: `korangar/src/renderer` sends commands to `wgpu` based on current state.
