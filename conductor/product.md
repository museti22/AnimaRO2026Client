# Initial Concept
Korangar is a next-generation Ragnarok Online client written in Rust, designed to provide a modern, high-performance, and cross-platform experience while maintaining compatibility with existing game data.

# Product Definition

## Vision
To modernize the Ragnarok Online experience by replacing the legacy client with a robust, modular, and visually advanced implementation that leverages the safety and performance of the Rust programming language.

## Core Purpose
Provide players and developers with a client that overcomes the technical debt and limitations of the original engine (Aegis/Granny), such as fixed aspect ratios, limited lighting, and platform restrictions.

## Target Audience
- **Players:** Seeking a modern, stable, and visually enhanced way to play Ragnarok Online on Windows, Linux, or MacOS.
- **Server Owners:** Looking for a customizable and secure client to offer their communities.
- **Developers:** Interested in a modular codebase for creating new features or tools for Ragnarok Online.

## Key Features
- **Modern Renderer:** Built on `wgpu`, featuring real-time lighting, dynamic shadows, and support for high resolutions and ultra-wide aspect ratios.
- **Cross-Platform:** Native support for Windows, Linux, and MacOS.
- **Customizable UI:** A completely new and flexible user interface system.
- **High Performance:** Multi-threaded architecture utilizing Rust's concurrency model and `tokio`.
- **Modular Design:** Functionality split into independent crates (audio, collision, networking, etc.) for better maintainability and reuse.
- **Asset Compatibility:** Direct loading and rendering of official Ragnarok Online data formats (.grf, .rsm, .rsw, .gat).

## Success Criteria
- Stable gameplay on all target platforms.
- Visual parity or improvement over the original client.
- Low latency networking and smooth input handling.
- Ease of contribution for new developers through clean, documented code.
