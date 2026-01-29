use korangar_components::item_box;
use korangar_interface::window::{CustomWindow, Window};
use korangar_networking::InventoryItem;
use rust_state::{Path, VecIndexExt};

use crate::ItemSource;
use crate::interface::windows::WindowClass;
use crate::state::theme::InterfaceThemeType;
use crate::state::ClientState;
use crate::world::ResourceMetadata;

pub struct StorageWindow<P> {
    items_path: P,
}

impl<P> StorageWindow<P> {
    pub fn new(items_path: P) -> Self {
        Self { items_path }
    }
}

impl<P> CustomWindow<ClientState> for StorageWindow<P>
where
    P: Path<ClientState, Vec<InventoryItem<ResourceMetadata>>>,
{
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::Storage)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        const STORAGE_ROWS: usize = 6;
        const STORAGE_COLUMNS: usize = 10;

        window! {
            title: "Storage",
            class: Self::window_class(),
            theme: InterfaceThemeType::InGame,
            closable: true,
            elements: std::array::from_fn::<_, STORAGE_ROWS, _>(|row| {
                split! {
                    gaps: theme().window().gaps(),
                    children: std::array::from_fn::<_, STORAGE_COLUMNS, _>(|column| {
                        let path = self.items_path.index(row * STORAGE_COLUMNS + column);

                        item_box! {
                            item_path: path,
                            source: ItemSource::Storage,
                        }
                    }),
                }
            }),
        }
    }
}
