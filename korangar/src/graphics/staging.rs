use std::num::NonZeroU64;

use wgpu::util::StagingBelt;
use wgpu::{CommandEncoder, Device};

/// Manages GPU staging buffers for efficient upload pipelining.
/// Uses double-buffered staging belts to overlap CPU writes with GPU reads.
pub struct StagingBeltManager {
    belt: StagingBelt,
    frame_index: u64,
}

impl StagingBeltManager {
    /// Create a new staging belt manager with the specified chunk size.
    /// The `device` is cloned internally by the staging belt for buffer
    /// allocation.
    pub fn new(device: Device, chunk_size: u64) -> Self {
        Self {
            belt: StagingBelt::new(device, chunk_size),
            frame_index: 0,
        }
    }

    /// Write data to a GPU buffer using the staging belt.
    /// This avoids blocking the main thread on buffer mapping.
    pub fn write_buffer(
        &mut self,
        encoder: &mut CommandEncoder,
        target: &wgpu::Buffer,
        offset: u64,
        data: &[u8],
    ) {
        if data.is_empty() {
            return;
        }
        let size = NonZeroU64::new(data.len() as u64).expect("data must not be empty");
        let mut view = self.belt.write_buffer(encoder, target, offset, size);
        view.copy_from_slice(data);
    }

    /// Call after encoding all commands for the frame.
    /// Submits the staging belt's copy commands.
    pub fn finish(&mut self) {
        self.belt.finish();
        self.frame_index += 1;
    }

    /// Call after the GPU finishes the frame (typically after present).
    /// Reclaims staging memory.
    pub fn recall(&mut self) {
        self.belt.recall();
    }

    /// Returns the current frame index (useful for double-buffering logic).
    pub fn frame_index(&self) -> u64 {
        self.frame_index
    }
}
