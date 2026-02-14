use korangar_interface::MouseMode;
use korangar_interface::element::store::{ElementStore, ElementStoreMut};
use korangar_interface::element::{BaseLayoutInfo, Element};
use korangar_interface::event::{ClickHandler, DropHandler, Event, EventQueue};
use korangar_interface::layout::area::Area;
use korangar_interface::layout::tooltip::TooltipExt;
use korangar_interface::layout::{MouseButton, Resolver, WindowLayout};
use korangar_interface::prelude::{HorizontalAlignment, VerticalAlignment};
use korangar_networking::{InventoryItem, InventoryItemDetails};
use ragnarok_packets::EquipPosition;
use rust_state::{Context, Path};

use crate::graphics::{Color, CornerDiameter, ShadowPadding};
use crate::input::{InputEvent, MouseInputMode};
use crate::interface::resource::ItemSource;
use crate::loaders::{FontSize, OverflowBehavior};
use crate::renderer::LayoutExt;
use crate::state::ClientState;
use crate::world::ResourceMetadata;

#[derive(Default)]
struct AmountDisplay {
    amount: u16,
    string: Option<String>,
}

impl AmountDisplay {
    fn update(&mut self, new_amount: u16) {
        if self.string.is_none() || self.amount != new_amount {
            self.string = Some(new_amount.to_string());
            self.amount = new_amount;
        }
    }
}

/// Caches a detailed tooltip string for an inventory item.
#[derive(Default)]
struct DetailedTooltip {
    item_id: u32,
    text: String,
}

impl DetailedTooltip {
    fn update(&mut self, item: &InventoryItem<ResourceMetadata>) {
        let new_id = item.item_id.0;
        if self.item_id == new_id && !self.text.is_empty() {
            return;
        }
        self.item_id = new_id;

        let mut lines = vec![item.metadata.name.clone()];

        let type_name = match item.item_type {
            0 => "Healing",
            2 => "Usable",
            3 => "Etc",
            4 => "Weapon",
            5 => "Armor",
            6 => "Card",
            7 => "Pet Egg",
            8 => "Pet Armor",
            10 => "Ammo",
            11 => "Delay Usable",
            12 => "Shadow Equip",
            18 => "Cash Usable",
            _ => "Item",
        };
        lines.push(format!("Type: {type_name}"));

        match &item.details {
            InventoryItemDetails::Regular { amount, .. } => {
                if *amount > 1 {
                    lines.push(format!("Amount: {amount}"));
                }
            }
            InventoryItemDetails::Equippable {
                refinement_level,
                enchantment_level,
                equip_position,
                ..
            } => {
                if *refinement_level > 0 {
                    lines.push(format!("Refine: +{refinement_level}"));
                }
                if *enchantment_level > 0 {
                    lines.push(format!("Enchant: +{enchantment_level}"));
                }
                let mut slots = Vec::new();
                if equip_position.contains(EquipPosition::RIGHT_HAND) { slots.push("Weapon"); }
                if equip_position.contains(EquipPosition::LEFT_HAND) { slots.push("Shield"); }
                if equip_position.contains(EquipPosition::ARMOR) { slots.push("Armor"); }
                if equip_position.contains(EquipPosition::GARMENT) { slots.push("Garment"); }
                if equip_position.contains(EquipPosition::SHOES) { slots.push("Shoes"); }
                if equip_position.contains(EquipPosition::HEAD_TOP) { slots.push("Head Top"); }
                if equip_position.contains(EquipPosition::HEAD_MIDDLE) { slots.push("Head Mid"); }
                if equip_position.contains(EquipPosition::HEAD_LOWER) { slots.push("Head Low"); }
                if equip_position.contains(EquipPosition::LEFT_ACCESSORY)
                    || equip_position.contains(EquipPosition::RIGTH_ACCESSORY) { slots.push("Accessory"); }
                if equip_position.contains(EquipPosition::AMMO) { slots.push("Ammo"); }
                if !slots.is_empty() {
                    lines.push(format!("Equip: {}", slots.join(", ")));
                }
            }
        }

        // Show card slots
        let has_cards = item.slot.iter().any(|&c| c != 0);
        if has_cards {
            let card_count = item.slot.iter().filter(|&&c| c != 0).count();
            lines.push(format!("Cards: {card_count}/4"));
        }

        lines.push(format!("ID: {}", item.item_id.0));

        self.text = lines.join("\n");
    }
}

struct ItemBoxHandler<P> {
    item_path: P,
    source: ItemSource,
}

impl<P> ItemBoxHandler<P> {
    fn new(item_path: P, source: ItemSource) -> Self {
        Self { item_path, source }
    }
}

impl<P> ClickHandler<ClientState> for ItemBoxHandler<P>
where
    P: Path<ClientState, InventoryItem<ResourceMetadata>, false>,
{
    fn handle_click(&self, state: &Context<ClientState>, queue: &mut EventQueue<ClientState>) {
        // SAFETY:
        //
        // Unwrapping here is fine since we only register the handler if the slot has a
        // item.
        let item = state.try_get(&self.item_path).unwrap().clone();

        queue.queue(Event::SetMouseMode {
            mouse_mode: MouseMode::Custom {
                mode: MouseInputMode::MoveItem { item, source: self.source },
            },
        });
    }
}

/// Handler for double-clicking an item.
/// - Inventory equippable items: equip them.
/// - Inventory consumable/regular items: use them.
/// - Equipment window items: unequip them.
struct ItemUseHandler<P> {
    item_path: P,
    source: ItemSource,
}

impl<P> ClickHandler<ClientState> for ItemUseHandler<P>
where
    P: Path<ClientState, InventoryItem<ResourceMetadata>, false>,
{
    fn handle_click(&self, state: &Context<ClientState>, queue: &mut EventQueue<ClientState>) {
        let Some(item) = state.try_get(&self.item_path) else {
            return;
        };

        match self.source {
            ItemSource::Inventory => match &item.details {
                InventoryItemDetails::Equippable { equip_position, .. } => {
                    queue.queue(InputEvent::MoveItem {
                        source: ItemSource::Inventory,
                        destination: ItemSource::Equipment { position: *equip_position },
                        item: item.clone(),
                    });
                }
                InventoryItemDetails::Regular { .. } => {
                    queue.queue(InputEvent::UseItem {
                        item_index: item.index,
                    });
                }
            },
            ItemSource::Equipment { position } => {
                queue.queue(InputEvent::MoveItem {
                    source: ItemSource::Equipment { position },
                    destination: ItemSource::Inventory,
                    item: item.clone(),
                });
            }
            ItemSource::Storage => {
                // Double-click on a storage item withdraws it to inventory.
                queue.queue(InputEvent::MoveItem {
                    source: ItemSource::Storage,
                    destination: ItemSource::Inventory,
                    item: item.clone(),
                });
            }
        }
    }
}

impl<P> DropHandler<ClientState> for ItemBoxHandler<P>
where
    P: Path<ClientState, InventoryItem<ResourceMetadata>, false>,
{
    fn handle_drop(&self, _: &Context<ClientState>, queue: &mut EventQueue<ClientState>, mouse_mode: &MouseMode<ClientState>) {
        if let MouseMode::Custom {
            mode: MouseInputMode::MoveItem { source, item },
        } = mouse_mode
        {
            queue.queue(InputEvent::MoveItem {
                source: *source,
                destination: self.source,
                item: item.clone(),
            });
        }
    }
}

pub struct ItemBox<A> {
    item_path: A,
    handler: ItemBoxHandler<A>,
    item_use_handler: ItemUseHandler<A>,
    amount_display: AmountDisplay,
    tooltip: DetailedTooltip,
}

impl<A> ItemBox<A>
where
    A: Copy,
{
    /// This function is supposed to be called from a component macro
    /// and not intended to be called manually.
    #[inline(always)]
    pub fn component_new(item_path: A, source: ItemSource) -> Self {
        Self {
            item_path,
            handler: ItemBoxHandler::new(item_path, source),
            item_use_handler: ItemUseHandler { item_path, source },
            amount_display: AmountDisplay::default(),
            tooltip: DetailedTooltip::default(),
        }
    }
}

impl<A> Element<ClientState> for ItemBox<A>
where
    A: Path<ClientState, InventoryItem<ResourceMetadata>, false>,
{
    type LayoutInfo = BaseLayoutInfo;

    fn create_layout_info(
        &mut self,
        state: &Context<ClientState>,
        _: ElementStoreMut<'_>,
        resolver: &mut Resolver<'_, ClientState>,
    ) -> Self::LayoutInfo {
        let area = resolver.with_height(40.0);

        if let Some(item) = state.try_get(&self.item_path)
            && item.metadata.texture.as_ref().is_some()
        {
            if let InventoryItemDetails::Regular { amount, .. } = &item.details {
                self.amount_display.update(*amount);
            }
            self.tooltip.update(item);
        }

        Self::LayoutInfo { area }
    }

    fn lay_out<'a>(
        &'a self,
        state: &'a Context<ClientState>,
        _: ElementStore<'a>,
        layout_info: &'a Self::LayoutInfo,
        layout: &mut WindowLayout<'a, ClientState>,
    ) {
        let (is_hovered, background_color) = match layout.get_mouse_mode() {
            MouseMode::Custom {
                mode: MouseInputMode::MoveItem { .. },
            } => match layout_info.area.check().any_mouse_mode().run(layout) {
                true => {
                    // Since we are not in default mouse mode we need to mark the window as
                    // hovered.
                    layout.set_hovered();

                    (true, Color::rgb_u8(80, 180, 180))
                }
                false => (false, Color::rgb_u8(180, 180, 80)),
            },
            _ => match layout_info.area.check().run(layout) {
                true => (true, Color::rgb_u8(60, 60, 60)),
                false => (false, Color::rgb_u8(40, 40, 40)),
            },
        };

        layout.add_rectangle(
            layout_info.area,
            CornerDiameter::uniform(20.0),
            background_color,
            Color::rgba_u8(0, 0, 0, 100),
            ShadowPadding::diagonal(2.0, 5.0),
        );

        if is_hovered {
            layout.register_drop_handler(&self.handler);
        }

        if let Some(item) = state.try_get(&self.item_path)
            && let Some(texture) = item.metadata.texture.as_ref()
        {
            let texture_size = layout_info.area.width.min(layout_info.area.height);
            let texture_area = Area {
                left: layout_info.area.left + (layout_info.area.width - texture_size) / 2.0,
                top: layout_info.area.top + (layout_info.area.height - texture_size) / 2.0,
                width: texture_size,
                height: texture_size,
            };

            layout.add_texture(texture_area, texture.clone(), Color::WHITE, false);

            if is_hovered {
                layout.register_click_handler(MouseButton::Left, &self.handler);
                layout.register_click_handler(MouseButton::DoubleLeft, &self.item_use_handler);

                struct ItemBoxTooltipId;
                layout.add_tooltip(&self.tooltip.text, ItemBoxTooltipId.tooltip_id());
            }

            if matches!(item.details, InventoryItemDetails::Regular { .. }) {
                layout.add_text(
                    layout_info.area,
                    self.amount_display.string.as_ref().unwrap(),
                    // TODO: Put this in the theme
                    FontSize(12.0),
                    // TODO: Put this in the theme
                    Color::rgb_u8(255, 200, 255),
                    // TODO: Put this in the theme
                    Color::rgb_u8(255, 160, 60),
                    // TODO: Put this in the theme
                    HorizontalAlignment::Right { offset: 3.0, border: 3.0 },
                    // TODO: Put this in the theme
                    VerticalAlignment::Bottom { offset: 3.0 },
                    OverflowBehavior::Shrink,
                );
            }
        }
    }
}
