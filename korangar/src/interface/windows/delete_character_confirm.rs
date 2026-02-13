use korangar_interface::window::{CustomWindow, Window};
use ragnarok_packets::CharacterId;

use crate::input::InputEvent;
use crate::interface::windows::WindowClass;
use crate::state::ClientState;
use crate::state::theme::InterfaceThemeType;

pub struct DeleteCharacterConfirmWindow {
    character_id: CharacterId,
    character_name: String,
}

impl DeleteCharacterConfirmWindow {
    pub fn new(character_id: CharacterId, character_name: String) -> Self {
        Self {
            character_id,
            character_name,
        }
    }
}

impl CustomWindow<ClientState> for DeleteCharacterConfirmWindow {
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::DeleteCharacterConfirm)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        window! {
            title: "Delete Character",
            class: Some(WindowClass::DeleteCharacterConfirm),
            theme: InterfaceThemeType::Menu,
            closable: true,
            elements: (
                text! {
                    text: format!("Are you sure you want to delete ^FF0000{}^000000?", self.character_name),
                },
                split! {
                    gaps: theme().window().gaps(),
                    children: (
                        button! {
                            text: "Cancel",
                            event: InputEvent::CancelDeleteCharacter,
                        },
                        button! {
                            text: "Confirm",
                            event: InputEvent::DeleteCharacter {
                                character_id: self.character_id,
                            },
                        },
                    ),
                },
            ),
        }
    }
}
