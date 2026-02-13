use korangar_interface::MouseMode;
use korangar_interface::application::Size;
use korangar_interface::element::store::{ElementStore, ElementStoreMut};
use korangar_interface::element::Element;
use korangar_interface::event::{DropHandler, EventQueue};
use korangar_interface::layout::area::Area;
use korangar_interface::layout::{Resolver, WindowLayout};
use korangar_interface::prelude::{HorizontalAlignment, VerticalAlignment};
use korangar_interface::window::{CustomWindow, Window};
use korangar_networking::InventoryItemDetails;
use rust_state::Context;

use crate::graphics::Color;
use crate::input::{InputEvent, MouseInputMode};
use crate::interface::resource::ItemSource;
use crate::interface::windows::WindowClass;
use crate::loaders::{FontSize, OverflowBehavior};
use crate::state::theme::InterfaceThemeType;
use crate::state::{ClientState, ClientStatePathExt, TradeState, client_state};

/// Drop handler that adds an inventory item to the trade when dropped on the
/// trade window.
struct TradeDropHandler;

impl DropHandler<ClientState> for TradeDropHandler {
    fn handle_drop(&self, _: &Context<ClientState>, queue: &mut EventQueue<ClientState>, mouse_mode: &MouseMode<ClientState>) {
        if let MouseMode::Custom {
            mode: MouseInputMode::MoveItem { source: ItemSource::Inventory, item },
        } = mouse_mode
        {
            let amount = match &item.details {
                InventoryItemDetails::Regular { amount, .. } => *amount as u32,
                _ => 1,
            };

            queue.queue(InputEvent::TradeAddItem {
                inventory_index: item.index,
                amount,
            });
        }
    }
}

struct TradeLayoutInfo {
    area: Area,
    lines: Vec<String>,
    line_heights: Vec<f32>,
}

struct TradeItemsElement {
    drop_handler: TradeDropHandler,
}

impl TradeItemsElement {
    fn build_lines(trade_state: &TradeState) -> Vec<String> {
        let mut lines = Vec::new();

        let lock_str = if trade_state.player_locked { " [LOCKED]" } else { "" };
        lines.push(format!("--- Your Items{} ---", lock_str));

        if trade_state.player_items.is_empty() && trade_state.player_zeny == 0 {
            lines.push("  (none)".to_string());
        } else {
            for item in &trade_state.player_items {
                let refine = if item.refinement_level > 0 {
                    format!("+{} ", item.refinement_level)
                } else {
                    String::new()
                };
                let ident = if !item.identified { " (unid)" } else { "" };
                lines.push(format!("  {}{} x{}{}", refine, item.name, item.amount, ident));
            }
            if trade_state.player_zeny > 0 {
                lines.push(format!("  Zeny: {}", trade_state.player_zeny));
            }
        }

        let partner_lock_str = if trade_state.partner_locked { " [LOCKED]" } else { "" };
        lines.push(format!("--- Partner Items{} ---", partner_lock_str));

        if trade_state.partner_items.is_empty() && trade_state.partner_zeny == 0 {
            lines.push("  (none)".to_string());
        } else {
            for item in &trade_state.partner_items {
                let refine = if item.refinement_level > 0 {
                    format!("+{} ", item.refinement_level)
                } else {
                    String::new()
                };
                let ident = if !item.identified { " (unid)" } else { "" };
                lines.push(format!("  {}{} x{}{}", refine, item.name, item.amount, ident));
            }
            if trade_state.partner_zeny > 0 {
                lines.push(format!("  Zeny: {}", trade_state.partner_zeny));
            }
        }

        lines
    }
}

impl Element<ClientState> for TradeItemsElement {
    type LayoutInfo = TradeLayoutInfo;

    fn create_layout_info(
        &mut self,
        state: &rust_state::Context<ClientState>,
        _: ElementStoreMut<'_>,
        resolver: &mut Resolver<'_, ClientState>,
    ) -> Self::LayoutInfo {
        let trade_state_path = client_state().trade_state();
        let trade_state = state.get(&trade_state_path);
        let lines = Self::build_lines(trade_state);
        let line_spacing = 4.0;
        let mut total_height = 0.0;
        let mut line_heights = Vec::new();

        for line in &lines {
            let color = if line.starts_with("--- ") {
                Color::rgb_u8(255, 200, 100)
            } else {
                Color::monochrome_u8(220)
            };

            let (size, _) = resolver.get_text_dimensions(
                line,
                color,
                Color::rgb_u8(255, 160, 60),
                FontSize(14.0),
                HorizontalAlignment::Left { offset: 5.0, border: 3.0 },
                OverflowBehavior::LineBreak,
            );

            if total_height != 0.0 {
                total_height += line_spacing;
            }
            total_height += size.height();
            line_heights.push(size.height());
        }

        let area = resolver.with_height(total_height);
        TradeLayoutInfo { area, lines, line_heights }
    }

    fn lay_out<'a>(
        &'a self,
        _state: &'a rust_state::Context<ClientState>,
        _: ElementStore<'a>,
        layout_info: &'a Self::LayoutInfo,
        layout: &mut WindowLayout<'a, ClientState>,
    ) {
        // Register drop handler so items can be dragged from inventory onto the
        // trade window.
        if let MouseMode::Custom {
            mode: MouseInputMode::MoveItem { .. },
        } = layout.get_mouse_mode()
        {
            if layout_info.area.check().any_mouse_mode().run(layout) {
                layout.set_hovered();
                layout.register_drop_handler(&self.drop_handler);
            }
        }

        let line_spacing = 4.0;
        let mut offset = 0.0;

        for (line, height) in layout_info.lines.iter().zip(layout_info.line_heights.iter()) {
            let color = if line.starts_with("--- ") {
                Color::rgb_u8(255, 200, 100)
            } else if line.starts_with("  +") || line.contains(" x") {
                Color::monochrome_u8(240)
            } else {
                Color::monochrome_u8(200)
            };

            if offset != 0.0 {
                offset += line_spacing;
            }

            let text_area = Area {
                left: layout_info.area.left,
                top: layout_info.area.top + offset,
                width: layout_info.area.width,
                height: *height,
            };

            layout.add_text(
                text_area,
                line,
                FontSize(14.0),
                color,
                Color::rgb_u8(255, 160, 60),
                HorizontalAlignment::Left { offset: 5.0, border: 3.0 },
                VerticalAlignment::Center { offset: 0.0 },
                OverflowBehavior::LineBreak,
            );

            offset += height;
        }
    }
}

/// Window for player-to-player trading.
pub struct TradeWindow;

impl CustomWindow<ClientState> for TradeWindow {
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::Trade)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        window! {
            title: "Trade",
            class: Some(WindowClass::Trade),
            theme: InterfaceThemeType::InGame,
            closable: true,
            minimum_width: 300.0,
            elements: (
                scroll_view! {
                    children: (
                        TradeItemsElement { drop_handler: TradeDropHandler },
                    ),
                },
                split! {
                    gaps: theme().window().gaps(),
                    children: (
                        button! {
                            text: "Cancel",
                            event: InputEvent::TradeCancel,
                        },
                        button! {
                            text: "Lock",
                            event: InputEvent::TradeLock,
                        },
                        button! {
                            text: "Complete",
                            event: InputEvent::TradeComplete,
                        },
                    ),
                },
            ),
        }
    }
}
