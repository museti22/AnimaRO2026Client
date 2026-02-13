use korangar_interface::window::{CustomWindow, Window};
use rust_state::Path;

use crate::interface::windows::WindowClass;
use crate::loaders::OverflowBehavior;
use crate::settings::{KeybindingCapabilities, KeybindingCapabilitiesPathExt, KeybindingSettings, KeybindingSettingsPathExt};
use crate::state::theme::InterfaceThemeType;
use crate::state::ClientState;

pub struct KeybindingSettingsWindow<A, B> {
    settings_path: A,
    capabilities_path: B,
}

impl<A, B> KeybindingSettingsWindow<A, B> {
    pub fn new(settings_path: A, capabilities_path: B) -> Self {
        Self {
            settings_path,
            capabilities_path,
        }
    }
}

impl<A, B> CustomWindow<ClientState> for KeybindingSettingsWindow<A, B>
where
    A: Path<ClientState, KeybindingSettings>,
    B: Path<ClientState, KeybindingCapabilities>,
{
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::KeybindingSettings)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        let elements = (
            text! {
                text: "--- Hotbar ---",
                overflow_behavior: OverflowBehavior::Shrink,
            },
            split! {
                children: (
                    text! {
                        text: "Slot 1",
                        overflow_behavior: OverflowBehavior::Shrink,
                    },
                    drop_down! {
                        selected: self.settings_path.hotbar_slot_1(),
                        options: self.capabilities_path.available_keys(),
                    }
                )
            },
            split! {
                children: (
                    text! {
                        text: "Slot 2",
                        overflow_behavior: OverflowBehavior::Shrink,
                    },
                    drop_down! {
                        selected: self.settings_path.hotbar_slot_2(),
                        options: self.capabilities_path.available_keys(),
                    }
                )
            },
            split! {
                children: (
                    text! {
                        text: "Slot 3",
                        overflow_behavior: OverflowBehavior::Shrink,
                    },
                    drop_down! {
                        selected: self.settings_path.hotbar_slot_3(),
                        options: self.capabilities_path.available_keys(),
                    }
                )
            },
            split! {
                children: (
                    text! {
                        text: "Slot 4",
                        overflow_behavior: OverflowBehavior::Shrink,
                    },
                    drop_down! {
                        selected: self.settings_path.hotbar_slot_4(),
                        options: self.capabilities_path.available_keys(),
                    }
                )
            },
            split! {
                children: (
                    text! {
                        text: "Slot 5",
                        overflow_behavior: OverflowBehavior::Shrink,
                    },
                    drop_down! {
                        selected: self.settings_path.hotbar_slot_5(),
                        options: self.capabilities_path.available_keys(),
                    }
                )
            },
            split! {
                children: (
                    text! {
                        text: "Slot 6",
                        overflow_behavior: OverflowBehavior::Shrink,
                    },
                    drop_down! {
                        selected: self.settings_path.hotbar_slot_6(),
                        options: self.capabilities_path.available_keys(),
                    }
                )
            },
            split! {
                children: (
                    text! {
                        text: "Slot 7",
                        overflow_behavior: OverflowBehavior::Shrink,
                    },
                    drop_down! {
                        selected: self.settings_path.hotbar_slot_7(),
                        options: self.capabilities_path.available_keys(),
                    }
                )
            },
            split! {
                children: (
                    text! {
                        text: "Slot 8",
                        overflow_behavior: OverflowBehavior::Shrink,
                    },
                    drop_down! {
                        selected: self.settings_path.hotbar_slot_8(),
                        options: self.capabilities_path.available_keys(),
                    }
                )
            },
            split! {
                children: (
                    text! {
                        text: "Slot 9",
                        overflow_behavior: OverflowBehavior::Shrink,
                    },
                    drop_down! {
                        selected: self.settings_path.hotbar_slot_9(),
                        options: self.capabilities_path.available_keys(),
                    }
                )
            },
            split! {
                children: (
                    text! {
                        text: "Slot 10",
                        overflow_behavior: OverflowBehavior::Shrink,
                    },
                    drop_down! {
                        selected: self.settings_path.hotbar_slot_10(),
                        options: self.capabilities_path.available_keys(),
                    }
                )
            },
            text! {
                text: "--- Windows (Alt+Key) ---",
                overflow_behavior: OverflowBehavior::Shrink,
            },
            split! {
                children: (
                    text! {
                        text: "Inventory",
                        overflow_behavior: OverflowBehavior::Shrink,
                    },
                    drop_down! {
                        selected: self.settings_path.toggle_inventory(),
                        options: self.capabilities_path.available_keys(),
                    }
                )
            },
            split! {
                children: (
                    text! {
                        text: "Equipment",
                        overflow_behavior: OverflowBehavior::Shrink,
                    },
                    drop_down! {
                        selected: self.settings_path.toggle_equipment(),
                        options: self.capabilities_path.available_keys(),
                    }
                )
            },
            split! {
                children: (
                    text! {
                        text: "Skill Tree",
                        overflow_behavior: OverflowBehavior::Shrink,
                    },
                    drop_down! {
                        selected: self.settings_path.toggle_skill_tree(),
                        options: self.capabilities_path.available_keys(),
                    }
                )
            },
            split! {
                children: (
                    text! {
                        text: "Stats",
                        overflow_behavior: OverflowBehavior::Shrink,
                    },
                    drop_down! {
                        selected: self.settings_path.toggle_stats(),
                        options: self.capabilities_path.available_keys(),
                    }
                )
            },
            split! {
                children: (
                    text! {
                        text: "Friend List",
                        overflow_behavior: OverflowBehavior::Shrink,
                    },
                    drop_down! {
                        selected: self.settings_path.toggle_friend_list(),
                        options: self.capabilities_path.available_keys(),
                    }
                )
            },
            split! {
                children: (
                    text! {
                        text: "Party",
                        overflow_behavior: OverflowBehavior::Shrink,
                    },
                    drop_down! {
                        selected: self.settings_path.toggle_party(),
                        options: self.capabilities_path.available_keys(),
                    }
                )
            },
            split! {
                children: (
                    text! {
                        text: "Guild",
                        overflow_behavior: OverflowBehavior::Shrink,
                    },
                    drop_down! {
                        selected: self.settings_path.toggle_guild(),
                        options: self.capabilities_path.available_keys(),
                    }
                )
            },
            split! {
                children: (
                    text! {
                        text: "Quest",
                        overflow_behavior: OverflowBehavior::Shrink,
                    },
                    drop_down! {
                        selected: self.settings_path.toggle_quest(),
                        options: self.capabilities_path.available_keys(),
                    }
                )
            },
            split! {
                children: (
                    text! {
                        text: "Minimap",
                        overflow_behavior: OverflowBehavior::Shrink,
                    },
                    drop_down! {
                        selected: self.settings_path.toggle_minimap(),
                        options: self.capabilities_path.available_keys(),
                    }
                )
            },
        );

        window! {
            title: "Keybinding Settings",
            class: Self::window_class(),
            theme: InterfaceThemeType::InGame,
            closable: true,
            elements,
        }
    }
}
