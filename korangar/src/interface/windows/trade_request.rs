use korangar_interface::window::{CustomWindow, Window};
use ragnarok_packets::AccountId;

use crate::input::InputEvent;
use crate::interface::windows::WindowClass;
use crate::state::ClientState;
use crate::state::theme::InterfaceThemeType;

pub struct TradeRequestWindow {
    requester_name: String,
    #[allow(dead_code)]
    account_id: AccountId,
    base_level: u16,
}

impl TradeRequestWindow {
    pub fn new(requester_name: String, account_id: AccountId, base_level: u16) -> Self {
        Self { requester_name, account_id, base_level }
    }
}

impl CustomWindow<ClientState> for TradeRequestWindow {
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::TradeRequest)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        window! {
            title: "Trade Request",
            class: Some(WindowClass::TradeRequest),
            theme: InterfaceThemeType::InGame,
            closable: true,
            elements: (
                text! {
                    text: format!(
                        "{} (Lv.{}) wants to trade with you.",
                        self.requester_name, self.base_level
                    ),
                },
                split! {
                    gaps: theme().window().gaps(),
                    children: (
                        button! {
                            text: "Reject",
                            event: InputEvent::RespondToTrade { accept: false },
                        },
                        button! {
                            text: "Accept",
                            event: InputEvent::RespondToTrade { accept: true },
                        },
                    ),
                },
            ),
        }
    }
}
