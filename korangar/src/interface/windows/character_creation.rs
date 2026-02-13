use korangar_interface::components::text_box::DefaultHandler;
use korangar_interface::window::{CustomWindow, Window};
use rust_state::{Context, Path};

use crate::input::InputEvent;
use crate::interface::windows::WindowClass;
use crate::loaders::OverflowBehavior;
use crate::state::localization::LocalizationPathExt;
use crate::state::theme::InterfaceThemeType;
use crate::state::{ClientState, ClientStatePathExt, client_state};

const MINIMUM_NAME_LENGTH: usize = 4;
const MAXIMUM_NAME_LENGTH: usize = 24;
const MAXIMUM_HAIR_STYLE: u16 = 25;
const MAXIMUM_HAIR_COLOR: u16 = 8;

pub struct CharacterCreationWindow<A, B, C> {
    character_name_path: A,
    hair_style_path: B,
    hair_color_path: C,
    slot: usize,
}

impl<A, B, C> CharacterCreationWindow<A, B, C> {
    pub fn new(character_name_path: A, hair_style_path: B, hair_color_path: C, slot: usize) -> Self {
        Self { character_name_path, hair_style_path, hair_color_path, slot }
    }
}

impl<A, B, C> CustomWindow<ClientState> for CharacterCreationWindow<A, B, C>
where
    A: Path<ClientState, String>,
    B: Path<ClientState, u16>,
    C: Path<ClientState, u16>,
{
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::CharacterCreation)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        struct CharacterName;

        let disabled = ComputedSelector::new_default(move |state: &ClientState| {
            self.character_name_path.follow(state).unwrap().len() < MINIMUM_NAME_LENGTH
        });

        let create_action = move |state: &Context<ClientState>, queue: &mut EventQueue<ClientState>| {
            let name = state.get(&self.character_name_path).clone();
            let hair_style = *state.get(&self.hair_style_path);
            let hair_color = *state.get(&self.hair_color_path);
            queue.queue(InputEvent::CreateCharacter {
                slot: self.slot,
                name,
                hair_style,
                hair_color,
            });
        };

        let hair_style_dec = move |state: &Context<ClientState>, _queue: &mut EventQueue<ClientState>| {
            state.update_value_with(self.hair_style_path, |value| {
                *value = value.checked_sub(1).unwrap_or(MAXIMUM_HAIR_STYLE);
            });
        };

        let hair_style_inc = move |state: &Context<ClientState>, _queue: &mut EventQueue<ClientState>| {
            state.update_value_with(self.hair_style_path, |value| {
                if *value >= MAXIMUM_HAIR_STYLE {
                    *value = 0;
                } else {
                    *value += 1;
                }
            });
        };

        let hair_color_dec = move |state: &Context<ClientState>, _queue: &mut EventQueue<ClientState>| {
            state.update_value_with(self.hair_color_path, |value| {
                *value = value.checked_sub(1).unwrap_or(MAXIMUM_HAIR_COLOR);
            });
        };

        let hair_color_inc = move |state: &Context<ClientState>, _queue: &mut EventQueue<ClientState>| {
            state.update_value_with(self.hair_color_path, |value| {
                if *value >= MAXIMUM_HAIR_COLOR {
                    *value = 0;
                } else {
                    *value += 1;
                }
            });
        };

        window! {
            title: client_state().localization().create_character_window_title(),
            class: Self::window_class(),
            theme: InterfaceThemeType::Menu,
            closable: true,
            elements: (
                text_box! {
                    ghost_text: client_state().localization().character_name_text(),
                    state: self.character_name_path,
                    input_handler: DefaultHandler::<_, _, MAXIMUM_NAME_LENGTH>::new(self.character_name_path, create_action),
                    focus_id: CharacterName,
                    overflow_behavior: OverflowBehavior::Shrink,
                },
                split! {
                    children: (
                        text! {
                            text: "Hair Style",
                            overflow_behavior: OverflowBehavior::Shrink,
                        },
                        button! {
                            text: "<",
                            event: hair_style_dec,
                        },
                        text! {
                            text: PartialEqDisplaySelector::new(self.hair_style_path),
                            overflow_behavior: OverflowBehavior::Shrink,
                        },
                        button! {
                            text: ">",
                            event: hair_style_inc,
                        },
                    ),
                },
                split! {
                    children: (
                        text! {
                            text: "Hair Color",
                            overflow_behavior: OverflowBehavior::Shrink,
                        },
                        button! {
                            text: "<",
                            event: hair_color_dec,
                        },
                        text! {
                            text: PartialEqDisplaySelector::new(self.hair_color_path),
                            overflow_behavior: OverflowBehavior::Shrink,
                        },
                        button! {
                            text: ">",
                            event: hair_color_inc,
                        },
                    ),
                },
                button! {
                    text: client_state().localization().create_character_button_text(),
                    disabled,
                    disabled_tooltip: client_state().localization().create_character_button_tooltip(),
                    event: create_action,
                }
            ),
        }
    }
}
