# Project Overview: Korangar

Korangar is a next-generation Ragnarok Online client implemented in Rust. It aims to modernize the experience while maintaining compatibility with original data.

## Key Features
- **Modern Rendering**: Built on `wgpu`, supporting real-time lighting, shadows, and high-performance graphics.
- **Cross-Platform**: Supports Windows, Linux, and macOS.
- **Modular Design**: Codebase split into specialized crates for audio, video, networking, and data parsing.
- **Scriptable**: Integrates Lua 5.1 via `mlua` for extensibility.
- **Improved UI**: Completely new, customizable user interface without official client limitations.

## Tech Stack
- **Language**: Rust (Edition 2024)
- **Graphics**: wgpu, winit, cgmath
- **Audio**: cpal, symphonia
- **Networking**: Custom protocol implementation for RO packets
- **Data Formats**: Supports GRF, MAP, RSM, SPR, ACT, etc.
- **Scripting**: Lua (mlua)
- **Async**: Tokio (used in networking)

## Repository Structure
The project follows a **Monorepo** structure using Rust Workspaces.

- **korangar**: The main client application.
- **korangar-***: Engine-specific libraries (audio, video, collision, interface, etc.).
- **ragnarok-***: Protocol and data format libraries independent of the main client.
