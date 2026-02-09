use std::time::{Duration, Instant};

/// Tracks frame timing and provides pacing information for GPU upload
/// scheduling. Helps determine when to start preparing the next frame's
/// uploads based on historical frame times.
///
/// This is separate from the existing [`super::FramePacer`] which handles
/// vsync-aware frame pacing and sleep scheduling. `UploadPacer` focuses on
/// lightweight timing statistics for upload pipeline decisions.
pub struct UploadPacer {
    frame_times: [Duration; 8],
    frame_index: usize,
    last_frame_start: Instant,
}

impl Default for UploadPacer {
    fn default() -> Self {
        Self {
            frame_times: [Duration::from_millis(16); 8],
            frame_index: 0,
            last_frame_start: Instant::now(),
        }
    }
}

impl UploadPacer {
    /// Record the start of a new frame.
    pub fn begin_frame(&mut self) {
        let now = Instant::now();
        let frame_time = now.duration_since(self.last_frame_start);
        self.frame_times[self.frame_index % 8] = frame_time;
        self.frame_index += 1;
        self.last_frame_start = now;
    }

    /// Returns the average frame time over the last 8 frames.
    pub fn average_frame_time(&self) -> Duration {
        let total: Duration = self.frame_times.iter().sum();
        total / 8
    }

    /// Returns the estimated FPS based on average frame time.
    pub fn estimated_fps(&self) -> f32 {
        let avg = self.average_frame_time();
        if avg.as_secs_f32() > 0.0 {
            1.0 / avg.as_secs_f32()
        } else {
            0.0
        }
    }

    /// Returns the time elapsed since the current frame started.
    pub fn elapsed_this_frame(&self) -> Duration {
        Instant::now().duration_since(self.last_frame_start)
    }

    /// Returns how much frame budget remains (based on average frame time).
    pub fn remaining_budget(&self) -> Duration {
        let avg = self.average_frame_time();
        let elapsed = self.elapsed_this_frame();
        avg.saturating_sub(elapsed)
    }
}
