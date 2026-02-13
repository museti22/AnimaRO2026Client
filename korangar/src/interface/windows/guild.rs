use std::cmp::Ordering;

use korangar_interface::element::store::{ElementStore, ElementStoreMut};
use korangar_interface::element::{Element, ElementBox};
use korangar_interface::layout::{Resolver, WindowLayout};
use korangar_interface::window::{CustomWindow, Window};
use rust_state::{Context, ManuallyAssertExt, Path, VecIndexExt};

use crate::interface::windows::WindowClass;
use crate::state::theme::InterfaceThemeType;
use crate::state::{ClientState, GuildMember, GuildMemberPathExt};

struct MemberList<A> {
    members_path: A,
    elements: Vec<ElementBox<ClientState>>,
}

impl<A> MemberList<A> {
    fn new(members_path: A) -> Self {
        Self {
            members_path,
            elements: Vec::new(),
        }
    }
}

impl<A> Element<ClientState> for MemberList<A>
where
    A: Path<ClientState, Vec<GuildMember>>,
{
    type LayoutInfo = ();

    fn create_layout_info(
        &mut self,
        state: &Context<ClientState>,
        mut store: ElementStoreMut<'_>,
        resolver: &mut Resolver<'_, ClientState>,
    ) -> Self::LayoutInfo {
        use korangar_interface::prelude::*;

        let members = state.get(&self.members_path);

        match members.len().cmp(&self.elements.len()) {
            Ordering::Less => {
                self.elements.truncate(members.len());
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                for index in self.elements.len()..members.len() {
                    let member_path = self.members_path.index(index).manually_asserted();
                    let name_path = member_path.name();

                    self.elements.push(ErasedElement::new(collapsable! {
                        text: name_path,
                        children: (
                            text! { text: "Position: (not available)" },
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

pub struct GuildWindow<P> {
    members_path: P,
}

impl<P> GuildWindow<P> {
    pub fn new(members_path: P) -> Self {
        Self { members_path }
    }
}

impl<P> CustomWindow<ClientState> for GuildWindow<P>
where
    P: Path<ClientState, Vec<GuildMember>>,
{
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::Guild)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        window! {
            title: "Guild",
            class: Self::window_class(),
            theme: InterfaceThemeType::InGame,
            closable: true,
            elements: [
                scroll_view! {
                    children: (
                        MemberList::new(self.members_path),
                    ),
                },
            ],
        }
    }
}
