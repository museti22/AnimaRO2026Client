use std::fmt;

#[cfg(feature = "debug")]
use korangar_debug::logging::{Colorize, print_debug};
use korangar_interface::components::drop_down::DropDownItem;
use korangar_interface::element::StateElement;
use ron::ser::PrettyConfig;
use rust_state::RustState;
use serde::{Deserialize, Serialize};
use winit::keyboard::KeyCode;

use crate::input::InputEvent;

/// Represents a key that can be bound to an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, RustState)]
pub enum BoundKey {
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
}

impl BoundKey {
    /// Convert a `BoundKey` to the corresponding `winit::keyboard::KeyCode`.
    pub fn to_key_code(self) -> KeyCode {
        match self {
            BoundKey::F1 => KeyCode::F1,
            BoundKey::F2 => KeyCode::F2,
            BoundKey::F3 => KeyCode::F3,
            BoundKey::F4 => KeyCode::F4,
            BoundKey::F5 => KeyCode::F5,
            BoundKey::F6 => KeyCode::F6,
            BoundKey::F7 => KeyCode::F7,
            BoundKey::F8 => KeyCode::F8,
            BoundKey::F9 => KeyCode::F9,
            BoundKey::F10 => KeyCode::F10,
            BoundKey::F11 => KeyCode::F11,
            BoundKey::F12 => KeyCode::F12,
            BoundKey::A => KeyCode::KeyA,
            BoundKey::B => KeyCode::KeyB,
            BoundKey::C => KeyCode::KeyC,
            BoundKey::D => KeyCode::KeyD,
            BoundKey::E => KeyCode::KeyE,
            BoundKey::F => KeyCode::KeyF,
            BoundKey::G => KeyCode::KeyG,
            BoundKey::H => KeyCode::KeyH,
            BoundKey::I => KeyCode::KeyI,
            BoundKey::J => KeyCode::KeyJ,
            BoundKey::K => KeyCode::KeyK,
            BoundKey::L => KeyCode::KeyL,
            BoundKey::M => KeyCode::KeyM,
            BoundKey::N => KeyCode::KeyN,
            BoundKey::O => KeyCode::KeyO,
            BoundKey::P => KeyCode::KeyP,
            BoundKey::Q => KeyCode::KeyQ,
            BoundKey::R => KeyCode::KeyR,
            BoundKey::S => KeyCode::KeyS,
            BoundKey::T => KeyCode::KeyT,
            BoundKey::U => KeyCode::KeyU,
            BoundKey::V => KeyCode::KeyV,
            BoundKey::W => KeyCode::KeyW,
            BoundKey::X => KeyCode::KeyX,
            BoundKey::Y => KeyCode::KeyY,
            BoundKey::Z => KeyCode::KeyZ,
            BoundKey::Num0 => KeyCode::Digit0,
            BoundKey::Num1 => KeyCode::Digit1,
            BoundKey::Num2 => KeyCode::Digit2,
            BoundKey::Num3 => KeyCode::Digit3,
            BoundKey::Num4 => KeyCode::Digit4,
            BoundKey::Num5 => KeyCode::Digit5,
            BoundKey::Num6 => KeyCode::Digit6,
            BoundKey::Num7 => KeyCode::Digit7,
            BoundKey::Num8 => KeyCode::Digit8,
            BoundKey::Num9 => KeyCode::Digit9,
        }
    }

    /// Try to match a `KeyCode` to a `BoundKey`.
    pub fn from_key_code(key_code: KeyCode) -> Option<Self> {
        match key_code {
            KeyCode::F1 => Some(BoundKey::F1),
            KeyCode::F2 => Some(BoundKey::F2),
            KeyCode::F3 => Some(BoundKey::F3),
            KeyCode::F4 => Some(BoundKey::F4),
            KeyCode::F5 => Some(BoundKey::F5),
            KeyCode::F6 => Some(BoundKey::F6),
            KeyCode::F7 => Some(BoundKey::F7),
            KeyCode::F8 => Some(BoundKey::F8),
            KeyCode::F9 => Some(BoundKey::F9),
            KeyCode::F10 => Some(BoundKey::F10),
            KeyCode::F11 => Some(BoundKey::F11),
            KeyCode::F12 => Some(BoundKey::F12),
            KeyCode::KeyA => Some(BoundKey::A),
            KeyCode::KeyB => Some(BoundKey::B),
            KeyCode::KeyC => Some(BoundKey::C),
            KeyCode::KeyD => Some(BoundKey::D),
            KeyCode::KeyE => Some(BoundKey::E),
            KeyCode::KeyF => Some(BoundKey::F),
            KeyCode::KeyG => Some(BoundKey::G),
            KeyCode::KeyH => Some(BoundKey::H),
            KeyCode::KeyI => Some(BoundKey::I),
            KeyCode::KeyJ => Some(BoundKey::J),
            KeyCode::KeyK => Some(BoundKey::K),
            KeyCode::KeyL => Some(BoundKey::L),
            KeyCode::KeyM => Some(BoundKey::M),
            KeyCode::KeyN => Some(BoundKey::N),
            KeyCode::KeyO => Some(BoundKey::O),
            KeyCode::KeyP => Some(BoundKey::P),
            KeyCode::KeyQ => Some(BoundKey::Q),
            KeyCode::KeyR => Some(BoundKey::R),
            KeyCode::KeyS => Some(BoundKey::S),
            KeyCode::KeyT => Some(BoundKey::T),
            KeyCode::KeyU => Some(BoundKey::U),
            KeyCode::KeyV => Some(BoundKey::V),
            KeyCode::KeyW => Some(BoundKey::W),
            KeyCode::KeyX => Some(BoundKey::X),
            KeyCode::KeyY => Some(BoundKey::Y),
            KeyCode::KeyZ => Some(BoundKey::Z),
            KeyCode::Digit0 => Some(BoundKey::Num0),
            KeyCode::Digit1 => Some(BoundKey::Num1),
            KeyCode::Digit2 => Some(BoundKey::Num2),
            KeyCode::Digit3 => Some(BoundKey::Num3),
            KeyCode::Digit4 => Some(BoundKey::Num4),
            KeyCode::Digit5 => Some(BoundKey::Num5),
            KeyCode::Digit6 => Some(BoundKey::Num6),
            KeyCode::Digit7 => Some(BoundKey::Num7),
            KeyCode::Digit8 => Some(BoundKey::Num8),
            KeyCode::Digit9 => Some(BoundKey::Num9),
            _ => None,
        }
    }
}

impl fmt::Display for BoundKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BoundKey::F1 => write!(f, "F1"),
            BoundKey::F2 => write!(f, "F2"),
            BoundKey::F3 => write!(f, "F3"),
            BoundKey::F4 => write!(f, "F4"),
            BoundKey::F5 => write!(f, "F5"),
            BoundKey::F6 => write!(f, "F6"),
            BoundKey::F7 => write!(f, "F7"),
            BoundKey::F8 => write!(f, "F8"),
            BoundKey::F9 => write!(f, "F9"),
            BoundKey::F10 => write!(f, "F10"),
            BoundKey::F11 => write!(f, "F11"),
            BoundKey::F12 => write!(f, "F12"),
            BoundKey::A => write!(f, "A"),
            BoundKey::B => write!(f, "B"),
            BoundKey::C => write!(f, "C"),
            BoundKey::D => write!(f, "D"),
            BoundKey::E => write!(f, "E"),
            BoundKey::F => write!(f, "F"),
            BoundKey::G => write!(f, "G"),
            BoundKey::H => write!(f, "H"),
            BoundKey::I => write!(f, "I"),
            BoundKey::J => write!(f, "J"),
            BoundKey::K => write!(f, "K"),
            BoundKey::L => write!(f, "L"),
            BoundKey::M => write!(f, "M"),
            BoundKey::N => write!(f, "N"),
            BoundKey::O => write!(f, "O"),
            BoundKey::P => write!(f, "P"),
            BoundKey::Q => write!(f, "Q"),
            BoundKey::R => write!(f, "R"),
            BoundKey::S => write!(f, "S"),
            BoundKey::T => write!(f, "T"),
            BoundKey::U => write!(f, "U"),
            BoundKey::V => write!(f, "V"),
            BoundKey::W => write!(f, "W"),
            BoundKey::X => write!(f, "X"),
            BoundKey::Y => write!(f, "Y"),
            BoundKey::Z => write!(f, "Z"),
            BoundKey::Num0 => write!(f, "0"),
            BoundKey::Num1 => write!(f, "1"),
            BoundKey::Num2 => write!(f, "2"),
            BoundKey::Num3 => write!(f, "3"),
            BoundKey::Num4 => write!(f, "4"),
            BoundKey::Num5 => write!(f, "5"),
            BoundKey::Num6 => write!(f, "6"),
            BoundKey::Num7 => write!(f, "7"),
            BoundKey::Num8 => write!(f, "8"),
            BoundKey::Num9 => write!(f, "9"),
        }
    }
}

impl DropDownItem<BoundKey> for BoundKey {
    fn text(&self) -> &str {
        match self {
            BoundKey::F1 => "F1",
            BoundKey::F2 => "F2",
            BoundKey::F3 => "F3",
            BoundKey::F4 => "F4",
            BoundKey::F5 => "F5",
            BoundKey::F6 => "F6",
            BoundKey::F7 => "F7",
            BoundKey::F8 => "F8",
            BoundKey::F9 => "F9",
            BoundKey::F10 => "F10",
            BoundKey::F11 => "F11",
            BoundKey::F12 => "F12",
            BoundKey::A => "A",
            BoundKey::B => "B",
            BoundKey::C => "C",
            BoundKey::D => "D",
            BoundKey::E => "E",
            BoundKey::F => "F",
            BoundKey::G => "G",
            BoundKey::H => "H",
            BoundKey::I => "I",
            BoundKey::J => "J",
            BoundKey::K => "K",
            BoundKey::L => "L",
            BoundKey::M => "M",
            BoundKey::N => "N",
            BoundKey::O => "O",
            BoundKey::P => "P",
            BoundKey::Q => "Q",
            BoundKey::R => "R",
            BoundKey::S => "S",
            BoundKey::T => "T",
            BoundKey::U => "U",
            BoundKey::V => "V",
            BoundKey::W => "W",
            BoundKey::X => "X",
            BoundKey::Y => "Y",
            BoundKey::Z => "Z",
            BoundKey::Num0 => "0",
            BoundKey::Num1 => "1",
            BoundKey::Num2 => "2",
            BoundKey::Num3 => "3",
            BoundKey::Num4 => "4",
            BoundKey::Num5 => "5",
            BoundKey::Num6 => "6",
            BoundKey::Num7 => "7",
            BoundKey::Num8 => "8",
            BoundKey::Num9 => "9",
        }
    }

    fn value(&self) -> BoundKey {
        *self
    }
}

/// Configurable keybinding settings with named fields for each hotbar slot.
#[derive(Clone, Serialize, Deserialize, RustState, StateElement)]
pub struct KeybindingSettings {
    pub hotbar_slot_1: BoundKey,
    pub hotbar_slot_2: BoundKey,
    pub hotbar_slot_3: BoundKey,
    pub hotbar_slot_4: BoundKey,
    pub hotbar_slot_5: BoundKey,
    pub hotbar_slot_6: BoundKey,
    pub hotbar_slot_7: BoundKey,
    pub hotbar_slot_8: BoundKey,
    pub hotbar_slot_9: BoundKey,
    pub hotbar_slot_10: BoundKey,
    pub toggle_inventory: BoundKey,
    pub toggle_equipment: BoundKey,
    pub toggle_skill_tree: BoundKey,
    pub toggle_stats: BoundKey,
    pub toggle_friend_list: BoundKey,
    pub toggle_party: BoundKey,
    pub toggle_guild: BoundKey,
    pub toggle_quest: BoundKey,
    pub toggle_minimap: BoundKey,
}

impl Default for KeybindingSettings {
    fn default() -> Self {
        Self {
            hotbar_slot_1: BoundKey::F1,
            hotbar_slot_2: BoundKey::F2,
            hotbar_slot_3: BoundKey::F3,
            hotbar_slot_4: BoundKey::F4,
            hotbar_slot_5: BoundKey::F5,
            hotbar_slot_6: BoundKey::F6,
            hotbar_slot_7: BoundKey::F7,
            hotbar_slot_8: BoundKey::F8,
            hotbar_slot_9: BoundKey::F9,
            hotbar_slot_10: BoundKey::F10,
            toggle_inventory: BoundKey::E,
            toggle_equipment: BoundKey::Q,
            toggle_skill_tree: BoundKey::T,
            toggle_stats: BoundKey::A,
            toggle_friend_list: BoundKey::Z,
            toggle_party: BoundKey::P,
            toggle_guild: BoundKey::G,
            toggle_quest: BoundKey::U,
            toggle_minimap: BoundKey::M,
        }
    }
}

impl KeybindingSettings {
    const FILE_NAME: &'static str = "client/keybinding_settings.ron";

    pub fn new() -> Self {
        Self::load().unwrap_or_else(|| {
            #[cfg(feature = "debug")]
            print_debug!("failed to load keybinding settings from {}", Self::FILE_NAME.magenta());
            Default::default()
        })
    }

    pub fn load() -> Option<Self> {
        #[cfg(feature = "debug")]
        print_debug!("loading keybinding settings from {}", Self::FILE_NAME.magenta());
        std::fs::read_to_string(Self::FILE_NAME)
            .ok()
            .and_then(|data| ron::from_str(&data).ok())
    }

    pub fn save(&self) {
        #[cfg(feature = "debug")]
        print_debug!("saving keybinding settings to {}", Self::FILE_NAME.magenta());

        let data = ron::ser::to_string_pretty(self, PrettyConfig::new()).unwrap();

        if let Err(_error) = std::fs::write(Self::FILE_NAME, data) {
            #[cfg(feature = "debug")]
            print_debug!(
                "failed to save keybinding settings to {}: {:?}",
                Self::FILE_NAME.magenta(),
                _error.red()
            );
        }
    }

    /// Returns all hotbar slot bindings as an array for iteration.
    pub fn hotbar_slots(&self) -> [BoundKey; 10] {
        [
            self.hotbar_slot_1,
            self.hotbar_slot_2,
            self.hotbar_slot_3,
            self.hotbar_slot_4,
            self.hotbar_slot_5,
            self.hotbar_slot_6,
            self.hotbar_slot_7,
            self.hotbar_slot_8,
            self.hotbar_slot_9,
            self.hotbar_slot_10,
        ]
    }

    /// Find if the given key code triggers a window toggle event (using Alt
    /// modifier).
    pub fn find_window_toggle(&self, key_code: KeyCode) -> Option<InputEvent> {
        if let Some(bound_key) = BoundKey::from_key_code(key_code) {
            if self.toggle_inventory == bound_key {
                return Some(InputEvent::ToggleInventoryWindow);
            }
            if self.toggle_equipment == bound_key {
                return Some(InputEvent::ToggleEquipmentWindow);
            }
            if self.toggle_skill_tree == bound_key {
                return Some(InputEvent::ToggleSkillTreeWindow);
            }
            if self.toggle_stats == bound_key {
                return Some(InputEvent::ToggleStatsWindow);
            }
            if self.toggle_friend_list == bound_key {
                return Some(InputEvent::ToggleFriendListWindow);
            }
            if self.toggle_party == bound_key {
                return Some(InputEvent::TogglePartyWindow);
            }
            if self.toggle_guild == bound_key {
                return Some(InputEvent::ToggleGuildWindow);
            }
            if self.toggle_quest == bound_key {
                return Some(InputEvent::ToggleQuestWindow);
            }
            if self.toggle_minimap == bound_key {
                return Some(InputEvent::ToggleMinimapWindow);
            }
        }
        None
    }
}

impl Drop for KeybindingSettings {
    fn drop(&mut self) {
        self.save();
    }
}

/// Available key options for the keybinding settings drop-downs.
#[derive(RustState, StateElement)]
pub struct KeybindingCapabilities {
    available_keys: Vec<BoundKey>,
}

impl Default for KeybindingCapabilities {
    fn default() -> Self {
        Self {
            available_keys: vec![
                BoundKey::F1,
                BoundKey::F2,
                BoundKey::F3,
                BoundKey::F4,
                BoundKey::F5,
                BoundKey::F6,
                BoundKey::F7,
                BoundKey::F8,
                BoundKey::F9,
                BoundKey::F10,
                BoundKey::F11,
                BoundKey::F12,
                BoundKey::A,
                BoundKey::B,
                BoundKey::C,
                BoundKey::D,
                BoundKey::E,
                BoundKey::F,
                BoundKey::G,
                BoundKey::H,
                BoundKey::I,
                BoundKey::J,
                BoundKey::K,
                BoundKey::L,
                BoundKey::M,
                BoundKey::N,
                BoundKey::O,
                BoundKey::P,
                BoundKey::Q,
                BoundKey::R,
                BoundKey::S,
                BoundKey::T,
                BoundKey::U,
                BoundKey::V,
                BoundKey::W,
                BoundKey::X,
                BoundKey::Y,
                BoundKey::Z,
                BoundKey::Num0,
                BoundKey::Num1,
                BoundKey::Num2,
                BoundKey::Num3,
                BoundKey::Num4,
                BoundKey::Num5,
                BoundKey::Num6,
                BoundKey::Num7,
                BoundKey::Num8,
                BoundKey::Num9,
            ],
        }
    }
}
