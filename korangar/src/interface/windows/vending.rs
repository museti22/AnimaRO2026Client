use std::cmp::Ordering;
use std::fmt::Display;

use korangar_interface::element::store::{ElementStore, ElementStoreMut};
use korangar_interface::element::{Element, ElementBox, ElementSet};
use korangar_interface::event::ClickHandler;
use korangar_interface::layout::area::Area;
use korangar_interface::layout::{Resolver, WindowLayout};
use korangar_interface::prelude::{HorizontalAlignment, VerticalAlignment};
use korangar_interface::window::{CustomWindow, Window};
use korangar_networking::VendingItem;
use ragnarok_packets::{AccountId, VendingPurchaseItemInformation};
use rust_state::{Context, ManuallyAssertExt, Path, VecIndexExt};

use super::WindowClass;
use crate::InputEvent;
use crate::graphics::{Color, CornerDiameter, ShadowPadding};
use crate::loaders::{FontSize, OverflowBehavior};
use crate::renderer::LayoutExt;
use crate::state::ClientState;
use crate::state::theme::InterfaceThemeType;
use crate::world::ResourceMetadata;

struct PartialEqDisplayStr<T> {
    last_value: Option<T>,
    text: String,
}

impl<T> PartialEqDisplayStr<T> {
    pub fn new() -> Self {
        Self {
            last_value: None,
            text: String::new(),
        }
    }
}

impl<T> PartialEqDisplayStr<T>
where
    T: Clone + PartialEq + Display + 'static,
{
    fn update(&mut self, value: T) {
        if self.last_value.is_none() || self.last_value.as_ref().is_some_and(|last| *last != value) {
            self.text = value.to_string();
            self.last_value = Some(value.clone());
        }
    }

    fn get_str(&self) -> &str {
        &self.text
    }
}

struct ItemLayoutInfo<A> {
    area: Area,
    texture_area: Area,
    text_area: Area,
    children: A,
}

struct ItemElement<A, B> {
    item_path: A,
    children: B,
    amount_string: PartialEqDisplayStr<u16>,
    price_string: PartialEqDisplayStr<u32>,
}

impl<A, B> ItemElement<A, B> {
    fn new(item_path: A, children: B) -> Self {
        Self {
            item_path,
            children,
            amount_string: PartialEqDisplayStr::new(),
            price_string: PartialEqDisplayStr::new(),
        }
    }
}

impl<A, B> Element<ClientState> for ItemElement<A, B>
where
    A: Path<ClientState, VendingItem<ResourceMetadata>>,
    B: ElementSet<ClientState>,
{
    type LayoutInfo = ItemLayoutInfo<B::LayoutInfo>;

    fn create_layout_info(
        &mut self,
        state: &Context<ClientState>,
        store: ElementStoreMut<'_>,
        resolver: &mut Resolver<'_, ClientState>,
    ) -> Self::LayoutInfo {
        let (area, (texture_area, text_area, children)) = resolver.with_derived(3.0, 3.0, |resolver| {
            let area = resolver.with_height(34.0);

            let texture_area = Area {
                width: 34.0,
                height: 34.0,
                ..area
            };

            let text_area = Area {
                left: area.left + 43.0,
                width: area.width - 43.0,
                ..area
            };

            let children = self.children.create_layout_info(state, store, resolver);

            (texture_area, text_area, children)
        });

        let item = state.get(&self.item_path);

        self.amount_string.update(item.amount);
        self.price_string.update(item.price.0);

        Self::LayoutInfo {
            area,
            texture_area,
            text_area,
            children,
        }
    }

    fn lay_out<'a>(
        &'a self,
        state: &'a Context<ClientState>,
        store: ElementStore<'a>,
        layout_info: &'a Self::LayoutInfo,
        layout: &mut WindowLayout<'a, ClientState>,
    ) {
        let item = state.get(&self.item_path);

        layout.add_rectangle(
            layout_info.area,
            CornerDiameter::uniform(4.0),
            Color::rgb_u8(80, 80, 80),
            Color::rgba_u8(0, 0, 0, 100),
            ShadowPadding::diagonal(2.0, 5.0),
        );

        if let Some(texture) = &item.metadata.texture {
            layout.add_texture(layout_info.texture_area, texture.clone(), Color::WHITE, false);

            layout.add_text(
                layout_info.texture_area,
                self.amount_string.get_str(),
                FontSize(16.0),
                Color::monochrome_u8(220),
                Color::rgb_u8(255, 160, 60),
                HorizontalAlignment::Right { offset: 3.0, border: 3.0 },
                VerticalAlignment::Bottom { offset: 0.0 },
                OverflowBehavior::Shrink,
            );
        }

        layout.add_text(
            layout_info.text_area,
            &item.metadata.name,
            FontSize(16.0),
            Color::monochrome_u8(220),
            Color::rgb_u8(255, 160, 60),
            HorizontalAlignment::Left { offset: 3.0, border: 3.0 },
            VerticalAlignment::Center { offset: 0.0 },
            OverflowBehavior::Shrink,
        );

        layout.add_text(
            layout_info.text_area,
            self.price_string.get_str(),
            FontSize(16.0),
            Color::rgb_u8(250, 230, 130),
            Color::rgb_u8(255, 160, 60),
            HorizontalAlignment::Right { offset: 3.0, border: 3.0 },
            VerticalAlignment::Center { offset: 0.0 },
            OverflowBehavior::Shrink,
        );

        self.children.lay_out(state, store, &layout_info.children, layout);
    }
}

struct ItemList<A, B, C> {
    items_path: A,
    account_id_path: B,
    unique_id_path: C,
    elements: Vec<ElementBox<ClientState>>,
}

impl<A, B, C> ItemList<A, B, C> {
    fn new(items_path: A, account_id_path: B, unique_id_path: C) -> Self {
        Self {
            items_path,
            account_id_path,
            unique_id_path,
            elements: Vec::new(),
        }
    }
}

impl<A, B, C> Element<ClientState> for ItemList<A, B, C>
where
    A: Path<ClientState, Vec<VendingItem<ResourceMetadata>>>,
    B: Path<ClientState, AccountId>,
    C: Path<ClientState, u32>,
{
    type LayoutInfo = ();

    fn create_layout_info(
        &mut self,
        state: &Context<ClientState>,
        mut store: ElementStoreMut<'_>,
        resolver: &mut Resolver<'_, ClientState>,
    ) -> Self::LayoutInfo {
        use korangar_interface::prelude::*;

        let items = state.get(&self.items_path);

        match items.len().cmp(&self.elements.len()) {
            Ordering::Less => {
                self.elements.truncate(items.len());
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                for index in self.elements.len()..items.len() {
                    let item_path = self.items_path.index(index).manually_asserted();
                    let account_id_path = self.account_id_path;
                    let unique_id_path = self.unique_id_path;

                    struct BuyAction<A, B, C> {
                        item_path: A,
                        account_id_path: B,
                        unique_id_path: C,
                        amount: u16,
                    }

                    impl<A, B, C> BuyAction<A, B, C> {
                        fn new(item_path: A, account_id_path: B, unique_id_path: C, amount: u16) -> Self {
                            Self {
                                item_path,
                                account_id_path,
                                unique_id_path,
                                amount,
                            }
                        }
                    }

                    impl<A, B, C> ClickHandler<ClientState> for BuyAction<A, B, C>
                    where
                        A: Path<ClientState, VendingItem<ResourceMetadata>>,
                        B: Path<ClientState, AccountId>,
                        C: Path<ClientState, u32>,
                    {
                        fn handle_click(&self, state: &Context<ClientState>, queue: &mut EventQueue<ClientState>) {
                            let item = state.get(&self.item_path);
                            let account_id = *state.get(&self.account_id_path);
                            let unique_id = *state.get(&self.unique_id_path);
                            let buy_amount = self.amount.min(item.amount);

                            if buy_amount == 0 {
                                return;
                            }

                            queue.queue(InputEvent::PurchaseFromVending {
                                account_id,
                                unique_id,
                                items: vec![VendingPurchaseItemInformation {
                                    amount: buy_amount,
                                    index: item.index,
                                }],
                            });
                        }
                    }

                    let buttons = (split! {
                        gaps: theme().window().gaps(),
                        children: (
                            button! {
                                text: "Buy 1",
                                event: BuyAction::new(item_path, account_id_path, unique_id_path, 1),
                            },
                            button! {
                                text: "Buy 10",
                                event: BuyAction::new(item_path, account_id_path, unique_id_path, 10),
                            },
                        ),
                    },);

                    self.elements.push(ErasedElement::new(ItemElement::new(item_path, buttons)));
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

/// Window displayed when viewing a player vendor's shop.
pub struct VendingWindow<A, B, C> {
    items_path: A,
    account_id_path: B,
    unique_id_path: C,
}

impl<A, B, C> VendingWindow<A, B, C> {
    pub fn new(items_path: A, account_id_path: B, unique_id_path: C) -> Self {
        Self {
            items_path,
            account_id_path,
            unique_id_path,
        }
    }
}

impl<A, B, C> CustomWindow<ClientState> for VendingWindow<A, B, C>
where
    A: Path<ClientState, Vec<VendingItem<ResourceMetadata>>>,
    B: Path<ClientState, AccountId>,
    C: Path<ClientState, u32>,
{
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::Vending)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        window! {
            title: "Vending Shop",
            class: Self::window_class(),
            theme: InterfaceThemeType::InGame,
            closable: true,
            resizable: true,
            minimum_width: 300.0,
            elements: (
                button! {
                    text: "Close",
                    event: InputEvent::CloseVending,
                },
                scroll_view! {
                    children: (
                        ItemList::new(self.items_path, self.account_id_path, self.unique_id_path),
                    ),
                },
            ),
        }
    }
}
