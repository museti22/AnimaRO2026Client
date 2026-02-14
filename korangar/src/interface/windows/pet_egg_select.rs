use korangar_interface::window::{CustomWindow, Window};
use rust_state::Context;

use crate::input::InputEvent;
use crate::interface::windows::WindowClass;
use crate::state::ClientState;
use crate::state::theme::InterfaceThemeType;

/// Window displaying a list of pet eggs for the player to select and hatch.
pub struct PetEggSelectWindow {
    eggs: Vec<u16>,
}

impl PetEggSelectWindow {
    pub fn new(eggs: Vec<u16>) -> Self {
        Self { eggs }
    }
}

impl CustomWindow<ClientState> for PetEggSelectWindow {
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::PetEggSelect)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        let description = if self.eggs.is_empty() {
            "No pet eggs found.".to_string()
        } else {
            let egg_list: Vec<String> = self.eggs.iter().map(|idx| format!("Egg #{}", idx)).collect();
            format!("Available eggs: {}", egg_list.join(", "))
        };

        let first_egg = self.eggs.first().copied().unwrap_or(0);

        window! {
            title: "Select Pet Egg",
            class: Self::window_class(),
            theme: InterfaceThemeType::InGame,
            closable: true,
            minimum_width: 220.0,
            elements: (
                text! { text: description },
                button! {
                    text: format!("Hatch Egg #{}", first_egg),
                    event: move |_: &Context<ClientState>, queue: &mut EventQueue<ClientState>| {
                        queue.queue(InputEvent::SelectPetEgg { index: first_egg });
                    },
                },
            ),
        }
    }
}
