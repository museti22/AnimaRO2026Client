# Development Guide

## Prerequisites
- **Rust**: Latest stable version (Edition 2024).
- **GPU Drivers**: Vulkan, Metal, or DX12 compatible drivers (for `wgpu`).
- **Official Data**: `data.grf` and other official Ragnarok files are required in the client folder.

## Setup
1. Clone the repository.
2. Place your official `.grf` files in the root or `korangar/` directory as specified in the configuration.
3. Install dependencies: `cargo fetch`.

## Common Commands
### Run the Client
```bash
cargo run -p korangar
```

### Build (Release)
```bash
cargo build -p korangar --release
```

### Run Tests
```bash
cargo test --workspace
```

## Contribution
Refer to `wiki/Contributing.md` for detailed coding standards and PR processes.
Current aesthetics: **Gothic-Fantasy** (Anima: Beyond Fantasy style). Avoid chibi/flat corporate designs.
