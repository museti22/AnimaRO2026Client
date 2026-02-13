use korangar_components::item_box;
use korangar_interface::window::{CustomWindow, Window};
use korangar_networking::InventoryItem;
use rust_state::{Path, Selector, VecIndexExt};

use crate::ItemSource;
use crate::interface::windows::WindowClass;
use crate::loaders::OverflowBehavior;
use crate::state::localization::LocalizationPathExt;
use crate::state::theme::InterfaceThemeType;
use crate::state::{ClientState, ClientStatePathExt, client_state};
use crate::world::{Player, PlayerPathExt, ResourceMetadata};

struct WeightSelector<W, M> {
    weight_path: W,
    max_weight_path: M,
    text: std::cell::UnsafeCell<String>,
}

impl<W, M> WeightSelector<W, M> {
    fn new(weight_path: W, max_weight_path: M) -> Self {
        Self {
            weight_path,
            max_weight_path,
            text: std::cell::UnsafeCell::default(),
        }
    }
}

impl<W, M> Selector<ClientState, String> for WeightSelector<W, M>
where
    W: Path<ClientState, u32>,
    M: Path<ClientState, u32>,
{
    fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a String> {
        let weight = self.weight_path.follow(state).unwrap();
        let max_weight = self.max_weight_path.follow(state).unwrap();
        unsafe {
            *self.text.get() = format!("Weight: {weight} / {max_weight}");
            Some(self.text.as_ref_unchecked())
        }
    }
}

struct ZenySelector<Z> {
    zeny_path: Z,
    text: std::cell::UnsafeCell<String>,
}

impl<Z> ZenySelector<Z> {
    fn new(zeny_path: Z) -> Self {
        Self {
            zeny_path,
            text: std::cell::UnsafeCell::default(),
        }
    }
}

impl<Z> Selector<ClientState, String> for ZenySelector<Z>
where
    Z: Path<ClientState, u32>,
{
    fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a String> {
        let zeny = self.zeny_path.follow(state).unwrap();
        unsafe {
            *self.text.get() = format!("Zeny: {zeny}");
            Some(self.text.as_ref_unchecked())
        }
    }
}

pub struct InventoryWindow<P, A> {
    items_path: P,
    player_path: A,
}

impl<P, A> InventoryWindow<P, A> {
    pub fn new(items_path: P, player_path: A) -> Self {
        Self { items_path, player_path }
    }
}

impl<P, A> CustomWindow<ClientState> for InventoryWindow<P, A>
where
    P: Path<ClientState, Vec<InventoryItem<ResourceMetadata>>>,
    A: Path<ClientState, Player>,
{
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::Inventory)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        // TODO: Probably this should be more dynamic
        const INVENTORY_ROWS: usize = 4;
        const INVENTORY_COLUMNS: usize = 10;

        window! {
            title: client_state().localization().inventory_window_title(),
            class: Self::window_class(),
            theme: InterfaceThemeType::InGame,
            closable: true,
            elements: (
                scroll_view! {
                    children: std::array::from_fn::<_, INVENTORY_ROWS, _>(|row| {
                        split! {
                            gaps: theme().window().gaps(),
                            children: std::array::from_fn::<_, INVENTORY_COLUMNS, _>(|column| {
                                let path = self.items_path.index(row * INVENTORY_COLUMNS + column);

                                item_box! {
                                    item_path: path,
                                    source: ItemSource::Inventory,
                                }
                            }),
                        }
                    }),
                },
                split! {
                    children: (
                        text! {
                            text: WeightSelector::new(self.player_path.weight(), self.player_path.maximum_weight()),
                            overflow_behavior: OverflowBehavior::Shrink,
                        },
                        text! {
                            text: ZenySelector::new(self.player_path.zeny()),
                            horizontal_alignment: HorizontalAlignment::Right { offset: 5.0, border: 5.0 },
                            overflow_behavior: OverflowBehavior::Shrink,
                        },
                    ),
                },
            ),
        }
    }
}
