/// Camera shake effect triggered by impacts.
pub struct CameraShake {
    intensity: f32,
    timer: f32,
    offset_x: f32,
    offset_y: f32,
}

impl Default for CameraShake {
    fn default() -> Self {
        Self {
            intensity: 0.0,
            timer: 0.0,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }
}

impl CameraShake {
    /// Trigger a camera shake with the given intensity (0.0-1.0).
    pub fn trigger(&mut self, intensity: f32) {
        self.intensity = intensity.clamp(0.0, 1.0);
        self.timer = 0.3; // 300ms shake duration
    }

    /// Update the shake effect. Returns the current offset.
    pub fn update(&mut self, delta_time: f32) -> (f32, f32) {
        if self.timer <= 0.0 {
            self.offset_x = 0.0;
            self.offset_y = 0.0;
            return (0.0, 0.0);
        }

        self.timer -= delta_time;
        let decay = (self.timer / 0.3).max(0.0);
        let magnitude = self.intensity * decay * 5.0;

        // Use a simple oscillation for shake.
        self.offset_x = (self.timer * 47.0).sin() * magnitude;
        self.offset_y = (self.timer * 53.0).cos() * magnitude;

        (self.offset_x, self.offset_y)
    }

    pub fn is_active(&self) -> bool {
        self.timer > 0.0
    }

    pub fn get_offset(&self) -> (f32, f32) {
        (self.offset_x, self.offset_y)
    }
}
