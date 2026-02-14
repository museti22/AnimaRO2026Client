use std::sync::Arc;

use korangar_interface::element::store::{ElementStore, ElementStoreMut};
use korangar_interface::element::Element;
use korangar_interface::layout::area::Area;
use korangar_interface::layout::{Resolver, WindowLayout};
use korangar_interface::prelude::{HorizontalAlignment, VerticalAlignment};
use korangar_interface::window::{CustomWindow, Window};

use crate::graphics::{Color, Texture};
use crate::interface::windows::WindowClass;
use crate::loaders::{FontSize, OverflowBehavior};
use crate::renderer::LayoutExt;
use crate::state::theme::InterfaceThemeType;
use crate::state::{ClientState, ClientStatePathExt};
use crate::state::client_state;
use crate::world::EntityType;

const MINIMAP_SIZE: f32 = 256.0;

struct MinimapLayoutInfo {
    area: Area,
}

struct MinimapElement {
    map_width: u16,
    map_height: u16,
    texture: Option<Arc<Texture>>,
}

impl MinimapElement {
    fn new(map_width: u16, map_height: u16, texture: Option<Arc<Texture>>) -> Self {
        Self { map_width, map_height, texture }
    }
}

impl Element<ClientState> for MinimapElement {
    type LayoutInfo = MinimapLayoutInfo;

    fn create_layout_info(
        &mut self,
        _state: &rust_state::Context<ClientState>,
        _: ElementStoreMut<'_>,
        resolver: &mut Resolver<'_, ClientState>,
    ) -> Self::LayoutInfo {
        let area = resolver.with_height(MINIMAP_SIZE);
        MinimapLayoutInfo { area }
    }

    fn lay_out<'a>(
        &'a self,
        state: &'a rust_state::Context<ClientState>,
        _: ElementStore<'a>,
        layout_info: &'a Self::LayoutInfo,
        layout: &mut WindowLayout<'a, ClientState>,
    ) {
        let entities_path = client_state().entities();
        let entities = state.get(&entities_path);
        let player_entity_id = entities.first().map(|e| e.get_entity_id());
        let map_w = self.map_width as f32;
        let map_h = self.map_height as f32;

        if map_w == 0.0 || map_h == 0.0 {
            return;
        }

        let bg_area = layout_info.area;

        // Render the minimap texture as background if available,
        // otherwise fall back to a dark semi-transparent rectangle.
        if let Some(texture) = &self.texture {
            layout.add_texture(bg_area, texture.clone(), Color::WHITE, true);
        } else {
            layout.add_rectangle(
                bg_area,
                Default::default(),
                Color::rgba(0.0, 0.0, 0.0, 0.5),
                Color::rgba(0.0, 0.0, 0.0, 0.0),
                Default::default(),
            );
        }

        // Render entity dots as small text markers on top of the minimap
        for entity in entities.iter() {
            let entity_type = entity.get_entity_type();

            // Skip hidden and warp entities
            if matches!(entity_type, EntityType::Hidden | EntityType::Warp) {
                continue;
            }

            let position = entity.get_tile_position();
            let x_ratio = position.x as f32 / map_w;
            let y_ratio = 1.0 - (position.y as f32 / map_h);

            let dot_x = bg_area.left + x_ratio * bg_area.width;
            let dot_y = bg_area.top + y_ratio * bg_area.height;

            let color = match entity_type {
                EntityType::Player => {
                    let is_self = player_entity_id
                        .map(|pid| pid == entity.get_entity_id())
                        .unwrap_or(false);
                    if is_self {
                        Color::rgb_u8(255, 255, 0) // Yellow = self
                    } else {
                        Color::rgb_u8(0, 255, 0) // Green = other player
                    }
                }
                EntityType::Monster => Color::rgb_u8(255, 0, 0),   // Red
                EntityType::Npc => Color::rgb_u8(0, 100, 255),     // Blue
                _ => continue,
            };

            let dot_area = Area {
                left: dot_x - 3.0,
                top: dot_y - 3.0,
                width: 8.0,
                height: 8.0,
            };

            layout.add_text(
                dot_area,
                "\u{25CF}",
                FontSize(8.0),
                color,
                color,
                HorizontalAlignment::Left { offset: 0.0, border: 0.0 },
                VerticalAlignment::Center { offset: 0.0 },
                OverflowBehavior::Shrink,
            );
        }

        // Render party member positions as cyan dots
        let party_path = client_state().party_members();
        let party_members = state.get(&party_path);
        for member in party_members.iter() {
            if member.x == 0 && member.y == 0 {
                continue;
            }

            let x_ratio = member.x as f32 / map_w;
            let y_ratio = 1.0 - (member.y as f32 / map_h);

            let dot_x = bg_area.left + x_ratio * bg_area.width;
            let dot_y = bg_area.top + y_ratio * bg_area.height;

            let dot_area = Area {
                left: dot_x - 3.0,
                top: dot_y - 3.0,
                width: 8.0,
                height: 8.0,
            };

            layout.add_text(
                dot_area,
                "\u{25CF}",
                FontSize(8.0),
                Color::rgb_u8(0, 255, 255), // Cyan = party member
                Color::rgb_u8(0, 255, 255),
                HorizontalAlignment::Left { offset: 0.0, border: 0.0 },
                VerticalAlignment::Center { offset: 0.0 },
                OverflowBehavior::Shrink,
            );
        }
    }
}

/// Window displaying a minimap of the current map with player position
/// and nearby entities.
pub struct MinimapWindow {
    map_name: String,
    map_width: u16,
    map_height: u16,
    texture: Option<Arc<Texture>>,
}

impl MinimapWindow {
    pub fn new(map_name: String, map_width: u16, map_height: u16, texture: Option<Arc<Texture>>) -> Self {
        Self {
            map_name,
            map_width,
            map_height,
            texture,
        }
    }
}

impl CustomWindow<ClientState> for MinimapWindow {
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::Minimap)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        window! {
            title: format!("Minimap - {}", self.map_name),
            class: Self::window_class(),
            theme: InterfaceThemeType::InGame,
            closable: true,
            minimum_width: 270.0,
            elements: (
                MinimapElement::new(self.map_width, self.map_height, self.texture),
                text! {
                    text: "^ffff00You^000000 | ^00ff00Players^000000 | ^00ffffParty^000000 | ^ff0000Monsters^000000 | ^0064ffNPCs^000000",
                },
            ),
        }
    }
}
