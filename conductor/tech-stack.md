# Technology Stack

## Core Technologies
- **Language:** Rust (Latest Stable)
    - Utilizes a workspace architecture for high modularity.
    - Heavily relies on Rust's memory safety and concurrency features.
- **Graphics API:** `wgpu` (WebGPU)
    - Provides a cross-platform, high-performance abstraction over Vulkan, Metal, DX12, and WebGL.
- **Async Runtime:** `tokio`
    - Powers the networking and asynchronous resource loading tasks.
- **Windowing & Input:** `winit`
    - Handles window creation, resizing, and low-level input events.

## Application Architecture
- **Rendering Engine:** Custom 3D renderer built on `wgpu`.
    - Features real-time lighting, dynamic shadows, and support for complex Ragnarok Online assets (.rsm, .rsw). UI uses SDF-based icons.
- **UI System:** Custom framework (`korangar-interface`).
    - Procedural UI rendering with focus on high customizability and theme support.
- **Networking:** Custom protocol stack (`korangar-networking`).
    - Implements the Ragnarok Online packet protocol using `ragnarok-packets` and `ragnarok-bytes`. Includes support for inventory, equipment, storage, and status updates.
- **Audio:** Custom engine (`korangar-audio`).
    - Built on `cpal` for low-level audio output and `symphonia` for decoding.
- **Collision & Physics:** Custom implementation (`korangar-collision`).
    - Specialized in handling Ragnarok's .gat (ground accessibility table) and 3D AABB collisions.

## Data & Configuration
- **Asset Formats:**
    - Custom parsers for official Ragnarok data: `.grf` (archives), `.rsm` (3D models), `.rsw` (world), `.gat` (walkability).
- **Serialization:**
    - `serde`: Primary serialization framework.
    - `ron` (Rusty Object Notation): Used for game settings and UI configurations.
- **Scripting:**
    - `mlua`: Integration with Lua for game logic and data handling compatible with official client files.

## Development & Tooling
- **Build System:** `cargo`
- **Environment:** Nix (via `flake.nix` and `.envrc`) for reproducible development environments.
- **CI/CD:** GitHub Actions for building, testing, and linting.
