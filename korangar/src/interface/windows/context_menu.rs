use std::cmp::Ordering;

use korangar_interface::element::store::{ElementStore, ElementStoreMut};
use korangar_interface::element::{Element, ElementBox};
use korangar_interface::layout::{Resolver, WindowLayout};
use korangar_interface::window::{CustomWindow, Window};
use ragnarok_packets::EntityId;
use rust_state::Context;

use crate::input::InputEvent;
use crate::interface::windows::WindowClass;
use crate::state::ClientState;
use crate::state::theme::InterfaceThemeType;
use crate::world::EntityType;

/// A menu entry with label and event.
struct MenuEntry {
    label: String,
    event: InputEvent,
}

/// Dynamic element that renders a list of buttons from menu entries.
struct MenuEntryList {
    entries: Vec<MenuEntry>,
    elements: Vec<ElementBox<ClientState>>,
}

impl MenuEntryList {
    fn new(entries: Vec<MenuEntry>) -> Self {
        Self {
            entries,
            elements: Vec::new(),
        }
    }
}

impl Element<ClientState> for MenuEntryList {
    type LayoutInfo = ();

    fn create_layout_info(
        &mut self,
        state: &Context<ClientState>,
        mut store: ElementStoreMut<'_>,
        resolver: &mut Resolver<'_, ClientState>,
    ) -> Self::LayoutInfo {
        use korangar_interface::prelude::*;

        match self.entries.len().cmp(&self.elements.len()) {
            Ordering::Less => {
                self.elements.truncate(self.entries.len());
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                for entry in self.elements.len()..self.entries.len() {
                    let label = self.entries[entry].label.clone();
                    let event = self.entries[entry].event.clone();
                    self.elements.push(ErasedElement::new(button! {
                        text: label,
                        event: event,
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

/// Context menu shown on right-click on a world entity.
pub struct ContextMenuWindow {
    entries: Vec<MenuEntry>,
    title: String,
}

impl ContextMenuWindow {
    pub fn new(entity_name: String, entity_id: EntityId, entity_type: EntityType) -> Self {
        let mut entries = Vec::new();

        match entity_type {
            EntityType::Player => {
                entries.push(MenuEntry {
                    label: "Trade".to_string(),
                    event: InputEvent::RequestTrade { entity_id },
                });
                entries.push(MenuEntry {
                    label: "Party Invite".to_string(),
                    event: InputEvent::InviteToParty {
                        character_name: entity_name.clone(),
                    },
                });
                entries.push(MenuEntry {
                    label: "Attack".to_string(),
                    event: InputEvent::PlayerInteract { entity_id },
                });
            }
            EntityType::Monster => {
                entries.push(MenuEntry {
                    label: "Attack".to_string(),
                    event: InputEvent::PlayerInteract { entity_id },
                });
            }
            EntityType::Npc => {
                entries.push(MenuEntry {
                    label: "Talk".to_string(),
                    event: InputEvent::PlayerInteract { entity_id },
                });
            }
            _ => {
                entries.push(MenuEntry {
                    label: "Interact".to_string(),
                    event: InputEvent::PlayerInteract { entity_id },
                });
            }
        }

        Self {
            entries,
            title: entity_name,
        }
    }
}

impl CustomWindow<ClientState> for ContextMenuWindow {
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::ContextMenu)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        window! {
            title: self.title,
            class: Self::window_class(),
            theme: InterfaceThemeType::InGame,
            closable: true,
            minimum_width: 150.0,
            elements: (
                MenuEntryList::new(self.entries),
            ),
        }
    }
}
