use korangar_interface::window::{CustomWindow, Window};
use rust_state::{Path, Selector};

use crate::interface::windows::WindowClass;
use crate::state::ClientState;
use crate::state::theme::InterfaceThemeType;

struct MailStatusSelector<A> {
    has_new_mail_path: A,
    text: std::cell::UnsafeCell<String>,
}

impl<A> MailStatusSelector<A> {
    fn new(has_new_mail_path: A) -> Self {
        Self {
            has_new_mail_path,
            text: std::cell::UnsafeCell::default(),
        }
    }
}

impl<A> Selector<ClientState, String> for MailStatusSelector<A>
where
    A: Path<ClientState, bool>,
{
    fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a String> {
        let has_new = self.has_new_mail_path.follow(state).unwrap();
        unsafe {
            *self.text.get() = if *has_new {
                "You have new mail!".to_string()
            } else {
                "No new mail.".to_string()
            };
            Some(self.text.as_ref_unchecked())
        }
    }
}

/// Window displayed for the mail / RODEX inbox.
pub struct MailWindow<A> {
    has_new_mail_path: A,
}

impl<A> MailWindow<A> {
    pub fn new(has_new_mail_path: A) -> Self {
        Self { has_new_mail_path }
    }
}

impl<A> CustomWindow<ClientState> for MailWindow<A>
where
    A: Path<ClientState, bool>,
{
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::Mail)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        let status_selector = MailStatusSelector::new(self.has_new_mail_path);

        window! {
            title: "Mailbox",
            class: Some(WindowClass::Mail),
            theme: InterfaceThemeType::InGame,
            closable: true,
            minimum_width: 300.0,
            elements: (
                text! {
                    text: status_selector,
                },
                text! {
                    text: "Mail content will be available in a future update.",
                },
            ),
        }
    }
}
