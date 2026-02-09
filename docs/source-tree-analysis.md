# Source Tree Analysis

## Root Directory
- `Cargo.toml`: Workspace configuration.
- `docs/`: Project documentation (Project Knowledge).
- `korangar/`: Main client source.
- `korangar-*/`: Specialized engine crates.
- `ragnarok-*/`: Data format and protocol crates.
- `wiki/`: Community and setup documentation.

## Main Client (`korangar/src`)
- `main.rs`: Entry point.
- `graphics/`: Low-level graphics abstractions.
- `input/`: Mouse and keyboard handling.
- `interface/`: UI logic and state.
- `loaders/`: High-level data loading orchestrators.
- `networking/`: Connection management and packet dispatching.
- `renderer/`: Game-specific rendering pipelines (Map, Entity, UI).
- `settings/`: Client configuration management.
- `state/`: Application state machines (Loading, Login, InGame).
- `system/`: OS-level integrations and utilities.
- `world/`: Map loading, entity management, and pathfinding.

## Sub-Projects (Crates)
- **ragnarok-packets**: Definition of binary packets for RO protocol.
- **ragnarok-formats**: Parsers for official Ragnarok data files.
- **korangar-interface**: Core UI framework and component library.
- **korangar-audio**: Audio engine using `cpal` and `symphonia`.
- **korangar-networking**: Low-level networking primitives.
