use std::cmp::Ordering;

use korangar_interface::element::store::{ElementStore, ElementStoreMut};
use korangar_interface::element::{Element, ElementBox};
use korangar_interface::layout::{Resolver, WindowLayout};
use korangar_interface::window::{CustomWindow, Window};
use rust_state::{Context, ManuallyAssertExt, Path, Selector, VecIndexExt};

use crate::interface::windows::WindowClass;
use crate::state::theme::InterfaceThemeType;
use crate::state::{ClientState, QuestEntry, QuestEntryPathExt};

struct StatusSelector<A> {
    active_path: A,
    text: std::cell::UnsafeCell<String>,
}

impl<A> StatusSelector<A> {
    fn new(active_path: A) -> Self {
        Self {
            active_path,
            text: std::cell::UnsafeCell::default(),
        }
    }
}

impl<A> Selector<ClientState, String> for StatusSelector<A>
where
    A: Path<ClientState, bool>,
{
    fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a String> {
        let active = self.active_path.follow(state).unwrap();
        unsafe {
            *self.text.get() = if *active { "Active".to_string() } else { "Inactive".to_string() };
            Some(self.text.as_ref_unchecked())
        }
    }
}

struct QuestList<A> {
    quests_path: A,
    elements: Vec<ElementBox<ClientState>>,
}

impl<A> QuestList<A> {
    fn new(quests_path: A) -> Self {
        Self {
            quests_path,
            elements: Vec::new(),
        }
    }
}

impl<A> Element<ClientState> for QuestList<A>
where
    A: Path<ClientState, Vec<QuestEntry>>,
{
    type LayoutInfo = ();

    fn create_layout_info(
        &mut self,
        state: &Context<ClientState>,
        mut store: ElementStoreMut<'_>,
        resolver: &mut Resolver<'_, ClientState>,
    ) -> Self::LayoutInfo {
        use korangar_interface::prelude::*;

        let quests = state.get(&self.quests_path);

        match quests.len().cmp(&self.elements.len()) {
            Ordering::Less => {
                self.elements.truncate(quests.len());
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                for index in self.elements.len()..quests.len() {
                    let quest_path = self.quests_path.index(index).manually_asserted();
                    let name_path = quest_path.name();
                    let status_selector = StatusSelector::new(quest_path.active());

                    self.elements.push(ErasedElement::new(collapsable! {
                        text: name_path,
                        children: (
                            text! { text: status_selector },
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

pub struct QuestWindow<P> {
    quests_path: P,
}

impl<P> QuestWindow<P> {
    pub fn new(quests_path: P) -> Self {
        Self { quests_path }
    }
}

impl<P> CustomWindow<ClientState> for QuestWindow<P>
where
    P: Path<ClientState, Vec<QuestEntry>>,
{
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::Quest)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        window! {
            title: "Quest Log",
            class: Self::window_class(),
            theme: InterfaceThemeType::InGame,
            closable: true,
            minimum_width: 250.0,
            elements: [
                scroll_view! {
                    children: (
                        QuestList::new(self.quests_path),
                    ),
                },
            ],
        }
    }
}
