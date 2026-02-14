use std::sync::Arc;

use korangar_interface::element::store::{ElementStore, ElementStoreMut};
use korangar_interface::element::Element;
use korangar_interface::layout::area::Area;
use korangar_interface::layout::{Resolver, WindowLayout};
use korangar_interface::window::{CustomWindow, Window};

use crate::graphics::{Color, Texture};
use crate::interface::windows::WindowClass;
use crate::renderer::LayoutExt;
use crate::state::ClientState;
use crate::state::theme::InterfaceThemeType;

const CUTIN_WIDTH: f32 = 256.0;
const CUTIN_HEIGHT: f32 = 256.0;

struct CutinLayoutInfo {
    area: Area,
}

struct CutinElement {
    texture: Arc<Texture>,
}

impl CutinElement {
    fn new(texture: Arc<Texture>) -> Self {
        Self { texture }
    }
}

impl Element<ClientState> for CutinElement {
    type LayoutInfo = CutinLayoutInfo;

    fn create_layout_info(
        &mut self,
        _state: &rust_state::Context<ClientState>,
        _: ElementStoreMut<'_>,
        resolver: &mut Resolver<'_, ClientState>,
    ) -> Self::LayoutInfo {
        let area = resolver.with_height(CUTIN_HEIGHT);
        CutinLayoutInfo { area }
    }

    fn lay_out<'a>(
        &'a self,
        _state: &'a rust_state::Context<ClientState>,
        _: ElementStore<'a>,
        layout_info: &'a Self::LayoutInfo,
        layout: &mut WindowLayout<'a, ClientState>,
    ) {
        layout.add_texture(layout_info.area, self.texture.clone(), Color::WHITE, true);
    }
}

/// Window displaying an NPC cutin image overlay.
pub struct CutinWindow {
    texture: Arc<Texture>,
    image_name: String,
}

impl CutinWindow {
    pub fn new(texture: Arc<Texture>, image_name: String) -> Self {
        Self { texture, image_name }
    }
}

impl CustomWindow<ClientState> for CutinWindow {
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::Cutin)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        window! {
            title: format!("Cutin - {}", self.image_name),
            class: Self::window_class(),
            theme: InterfaceThemeType::InGame,
            closable: true,
            minimum_width: CUTIN_WIDTH + 20.0,
            elements: (
                CutinElement::new(self.texture),
            ),
        }
    }
}
