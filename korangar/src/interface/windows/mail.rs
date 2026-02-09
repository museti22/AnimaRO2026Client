use korangar_interface::window::{CustomWindow, Window};

use crate::interface::windows::WindowClass;
use crate::state::ClientState;
use crate::state::theme::InterfaceThemeType;

/// Window displayed for the mail / RODEX inbox.
pub struct MailWindow;

impl CustomWindow<ClientState> for MailWindow {
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::Mail)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        window! {
            title: "Mailbox",
            class: Some(WindowClass::Mail),
            theme: InterfaceThemeType::InGame,
            closable: true,
            minimum_width: 300.0,
            elements: (
                text! {
                    text: "Your mail inbox will be displayed here.",
                },
                text! {
                    text: "New mail notifications will appear in chat.",
                },
            ),
        }
    }
}
