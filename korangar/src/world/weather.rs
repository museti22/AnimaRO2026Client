use rand_aes::tls::rand_f32;

use crate::graphics::{Color, ScreenPosition, ScreenSize};
use crate::loaders::FontSize;
use crate::renderer::GameInterfaceRenderer;

/// Weather type that can be active on a map.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WeatherType {
    None,
    Rain,
    Snow,
    Fog,
}

/// A single weather particle (raindrop or snowflake).
struct WeatherParticle {
    x: f32,
    y: f32,
    velocity_x: f32,
    velocity_y: f32,
    size: f32,
    alpha: f32,
}

impl WeatherParticle {
    fn new_rain(screen_width: f32, screen_height: f32) -> Self {
        Self {
            x: rand_f32() * screen_width,
            y: -rand_f32() * screen_height * 0.3,
            velocity_x: -30.0 - rand_f32() * 20.0,
            velocity_y: 600.0 + rand_f32() * 200.0,
            size: 1.0 + rand_f32() * 1.5,
            alpha: 0.3 + rand_f32() * 0.4,
        }
    }

    fn new_snow(screen_width: f32, screen_height: f32) -> Self {
        Self {
            x: rand_f32() * screen_width,
            y: -rand_f32() * screen_height * 0.1,
            velocity_x: -10.0 + rand_f32() * 20.0,
            velocity_y: 40.0 + rand_f32() * 60.0,
            size: 2.0 + rand_f32() * 3.0,
            alpha: 0.5 + rand_f32() * 0.5,
        }
    }
}

const MAX_RAIN_PARTICLES: usize = 300;
const MAX_SNOW_PARTICLES: usize = 200;

/// Manages weather effects (rain, snow, fog).
pub struct WeatherSystem {
    weather_type: WeatherType,
    particles: Vec<WeatherParticle>,
    fog_opacity: f32,
    spawn_timer: f32,
}

impl Default for WeatherSystem {
    fn default() -> Self {
        Self {
            weather_type: WeatherType::None,
            particles: Vec::new(),
            fog_opacity: 0.0,
            spawn_timer: 0.0,
        }
    }
}

impl WeatherSystem {
    pub fn set_weather(&mut self, weather_type: WeatherType) {
        if self.weather_type != weather_type {
            self.weather_type = weather_type;
            self.particles.clear();
            self.fog_opacity = 0.0;
        }
    }

    pub fn get_weather(&self) -> WeatherType {
        self.weather_type
    }

    pub fn clear(&mut self) {
        self.weather_type = WeatherType::None;
        self.particles.clear();
        self.fog_opacity = 0.0;
    }

    pub fn update(&mut self, delta_time: f32, window_size: ScreenSize) {
        match self.weather_type {
            WeatherType::None => {}
            WeatherType::Rain => self.update_rain(delta_time, window_size),
            WeatherType::Snow => self.update_snow(delta_time, window_size),
            WeatherType::Fog => self.update_fog(delta_time),
        }
    }

    fn update_rain(&mut self, delta_time: f32, window_size: ScreenSize) {
        // Spawn new particles.
        self.spawn_timer += delta_time;
        let spawn_interval = 1.0 / 200.0; // 200 particles per second.
        while self.spawn_timer >= spawn_interval && self.particles.len() < MAX_RAIN_PARTICLES {
            self.particles
                .push(WeatherParticle::new_rain(window_size.width, window_size.height));
            self.spawn_timer -= spawn_interval;
        }

        // Update existing particles.
        for particle in &mut self.particles {
            particle.x += particle.velocity_x * delta_time;
            particle.y += particle.velocity_y * delta_time;
        }

        // Remove off-screen particles.
        self.particles
            .retain(|p| p.y < window_size.height + 10.0 && p.x > -10.0);
    }

    fn update_snow(&mut self, delta_time: f32, window_size: ScreenSize) {
        self.spawn_timer += delta_time;
        let spawn_interval = 1.0 / 80.0; // 80 particles per second.
        while self.spawn_timer >= spawn_interval && self.particles.len() < MAX_SNOW_PARTICLES {
            self.particles
                .push(WeatherParticle::new_snow(window_size.width, window_size.height));
            self.spawn_timer -= spawn_interval;
        }

        for particle in &mut self.particles {
            // Snow has a gentle sway.
            particle.velocity_x = (particle.y * 0.02).sin() * 15.0;
            particle.x += particle.velocity_x * delta_time;
            particle.y += particle.velocity_y * delta_time;
        }

        self.particles
            .retain(|p| p.y < window_size.height + 10.0 && p.x > -10.0 && p.x < window_size.width + 10.0);
    }

    fn update_fog(&mut self, delta_time: f32) {
        // Fog fades in over 2 seconds.
        self.fog_opacity = (self.fog_opacity + delta_time * 0.5).min(0.25);
    }

    pub fn render(&self, renderer: &GameInterfaceRenderer, window_size: ScreenSize) {
        match self.weather_type {
            WeatherType::None => {}
            WeatherType::Rain => self.render_rain(renderer),
            WeatherType::Snow => self.render_snow(renderer),
            WeatherType::Fog => self.render_fog(renderer, window_size),
        }
    }

    fn render_rain(&self, renderer: &GameInterfaceRenderer) {
        for particle in &self.particles {
            let position = ScreenPosition {
                left: particle.x,
                top: particle.y,
            };
            let color = Color::rgba(0.7, 0.8, 1.0, particle.alpha);
            // Render rain as short vertical line via damage text ("|").
            renderer.render_damage_text("|", position, color, FontSize(particle.size * 6.0));
        }
    }

    fn render_snow(&self, renderer: &GameInterfaceRenderer) {
        for particle in &self.particles {
            let position = ScreenPosition {
                left: particle.x,
                top: particle.y,
            };
            let color = Color::rgba(1.0, 1.0, 1.0, particle.alpha);
            // Render snowflake as a dot.
            renderer.render_damage_text(".", position, color, FontSize(particle.size * 4.0));
        }
    }

    fn render_fog(&self, renderer: &GameInterfaceRenderer, window_size: ScreenSize) {
        if self.fog_opacity > 0.01 {
            let position = ScreenPosition { left: 0.0, top: 0.0 };
            let color = Color::rgba(0.7, 0.75, 0.8, self.fog_opacity);
            renderer.render_rectangle(position, window_size, color);
        }
    }
}
