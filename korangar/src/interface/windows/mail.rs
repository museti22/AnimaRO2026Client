use std::cmp::Ordering;

use korangar_interface::element::store::{ElementStore, ElementStoreMut};
use korangar_interface::element::{Element, ElementBox};
use korangar_interface::layout::{Resolver, WindowLayout};
use korangar_interface::window::{CustomWindow, Window};
use rust_state::{Context, ManuallyAssertExt, Path, Selector, VecIndexExt};

use crate::interface::windows::WindowClass;
use crate::state::theme::InterfaceThemeType;
use crate::state::{ClientState, MailEntry};

/// Selector for mail status text.
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

/// Selector that formats a mail entry's header line (sender + title + read status).
struct MailHeaderSelector<M> {
    mail_path: M,
    text: std::cell::UnsafeCell<String>,
}

impl<M> MailHeaderSelector<M> {
    fn new(mail_path: M) -> Self {
        Self {
            mail_path,
            text: std::cell::UnsafeCell::default(),
        }
    }
}

impl<M> Selector<ClientState, String> for MailHeaderSelector<M>
where
    M: Path<ClientState, MailEntry>,
{
    fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a String> {
        let mail = self.mail_path.follow(state)?;
        unsafe {
            let read_marker = if mail.read { "" } else { "[NEW] " };
            *self.text.get() = format!("{}From: {} - {}", read_marker, mail.sender_name, mail.title);
            Some(self.text.as_ref_unchecked())
        }
    }
}

/// Selector that formats a mail entry's body details.
struct MailBodySelector<M> {
    mail_path: M,
    text: std::cell::UnsafeCell<String>,
}

impl<M> MailBodySelector<M> {
    fn new(mail_path: M) -> Self {
        Self {
            mail_path,
            text: std::cell::UnsafeCell::default(),
        }
    }
}

impl<M> Selector<ClientState, String> for MailBodySelector<M>
where
    M: Path<ClientState, MailEntry>,
{
    fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a String> {
        let mail = self.mail_path.follow(state)?;
        unsafe {
            let mut detail = mail.body.clone();
            if mail.zeny > 0 {
                detail.push_str(&format!("\nZeny: {}", mail.zeny));
            }
            if mail.has_item {
                detail.push_str("\n[Has attached item]");
            }
            *self.text.get() = detail;
            Some(self.text.as_ref_unchecked())
        }
    }
}

/// Element that dynamically renders the mail list.
struct MailList<B> {
    mail_entries_path: B,
    elements: Vec<ElementBox<ClientState>>,
}

impl<B> MailList<B> {
    fn new(mail_entries_path: B) -> Self {
        Self {
            mail_entries_path,
            elements: Vec::new(),
        }
    }
}

impl<B> Element<ClientState> for MailList<B>
where
    B: Path<ClientState, Vec<MailEntry>>,
{
    type LayoutInfo = ();

    fn create_layout_info(
        &mut self,
        state: &Context<ClientState>,
        mut store: ElementStoreMut<'_>,
        resolver: &mut Resolver<'_, ClientState>,
    ) -> Self::LayoutInfo {
        use korangar_interface::prelude::*;

        let mails = state.get(&self.mail_entries_path);

        match mails.len().cmp(&self.elements.len()) {
            Ordering::Less => {
                self.elements.truncate(mails.len());
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                for index in self.elements.len()..mails.len() {
                    let mail_path = self.mail_entries_path.index(index).manually_asserted();
                    let header_selector = MailHeaderSelector::new(mail_path);
                    let body_selector = MailBodySelector::new(mail_path);

                    self.elements.push(ErasedElement::new(collapsable! {
                        text: header_selector,
                        children: (
                            text! { text: body_selector },
                        ),
                    }));
                }
            }
        }

        self.elements.iter_mut().enumerate().for_each(|(index, element)| {
            element.create_layout_info(state, store.child_store(index as u64), resolver);
        });
    }

    fn lay_out<'a>(
        &'a self,
        state: &'a Context<ClientState>,
        store: ElementStore<'a>,
        _: &'a Self::LayoutInfo,
        layout: &mut WindowLayout<'a, ClientState>,
    ) {
        self.elements.iter().enumerate().for_each(|(index, element)| {
            element.lay_out(state, store.child_store(index as u64), &(), layout);
        });
    }
}

/// Window displayed for the mail / RODEX inbox.
pub struct MailWindow<A, B> {
    has_new_mail_path: A,
    mail_entries_path: B,
}

impl<A, B> MailWindow<A, B> {
    pub fn new(has_new_mail_path: A, mail_entries_path: B) -> Self {
        Self {
            has_new_mail_path,
            mail_entries_path,
        }
    }
}

impl<A, B> CustomWindow<ClientState> for MailWindow<A, B>
where
    A: Path<ClientState, bool>,
    B: Path<ClientState, Vec<MailEntry>>,
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
            minimum_width: 350.0,
            elements: (
                text! {
                    text: status_selector,
                },
                scroll_view! {
                    children: (
                        MailList::new(self.mail_entries_path),
                    ),
                },
            ),
        }
    }
}
