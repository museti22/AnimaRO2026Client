#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "interface", derive(rust_state::RustState, korangar_interface::element::StateElement))]
pub enum MessageColor {
    Rgb { red: u8, green: u8, blue: u8 },
    Broadcast,
    Server,
    Error,
    Information,
    Whisper,
    Party,
    Guild,
    Battle,
}
