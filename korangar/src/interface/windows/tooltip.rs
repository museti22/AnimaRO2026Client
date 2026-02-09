use korangar_interface::window::{CustomWindow, Window};

use crate::graphics::Color;
use crate::interface::windows::WindowClass;
use crate::state::ClientState;
use crate::state::theme::InterfaceThemeType;

/// Floating tooltip window for entity/item details.
pub struct TooltipWindow {
    pub title: String,
    pub lines: Vec<String>,
}

impl TooltipWindow {
    pub fn new(title: String, lines: Vec<String>) -> Self {
        Self { title, lines }
    }

    pub fn entity_tooltip(name: String, entity_type: &str, level: Option<usize>) -> Self {
        let mut lines = vec![format!("Type: {entity_type}")];
        if let Some(lvl) = level {
            lines.push(format!("Level: {lvl}"));
        }
        Self { title: name, lines }
    }

    pub fn item_tooltip(name: String, item_type: &str, description: Option<String>) -> Self {
        let mut lines = vec![format!("Type: {item_type}")];
        if let Some(desc) = description {
            lines.push(desc);
        }
        Self { title: name, lines }
    }
}

impl CustomWindow<ClientState> for TooltipWindow {
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::Tooltip)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        let description = self.lines.join("\n");

        window! {
            title: self.title,
            class: Some(WindowClass::Tooltip),
            theme: InterfaceThemeType::InGame,
            closable: false,
            minimum_width: 200.0,
            elements: (
                text! {
                    text: description,
                    color: Color::rgb_u8(200, 200, 200),
                },
            ),
        }
    }
}
