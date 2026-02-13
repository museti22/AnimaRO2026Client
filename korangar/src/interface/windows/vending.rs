use korangar_interface::window::{CustomWindow, Window};

use crate::interface::windows::WindowClass;
use crate::state::ClientState;
use crate::state::theme::InterfaceThemeType;

/// Window displayed when viewing a player vendor's shop.
#[allow(dead_code)]
pub struct VendingWindow;

impl CustomWindow<ClientState> for VendingWindow {
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::Vending)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        window! {
            title: "Vending Shop",
            class: Some(WindowClass::Vending),
            theme: InterfaceThemeType::InGame,
            closable: true,
            minimum_width: 300.0,
            elements: (
                text! {
                    text: "Vendor shop items will appear here.",
                },
                text! {
                    text: "Click an item to purchase it.",
                },
            ),
        }
    }
}
