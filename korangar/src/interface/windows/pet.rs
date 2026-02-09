use korangar_interface::application::Size;
use korangar_interface::element::store::{ElementStore, ElementStoreMut};
use korangar_interface::element::Element;
use korangar_interface::layout::area::Area;
use korangar_interface::layout::{Resolver, WindowLayout};
use korangar_interface::prelude::{HorizontalAlignment, VerticalAlignment};
use korangar_interface::window::{CustomWindow, Window};

use crate::graphics::Color;
use crate::input::InputEvent;
use crate::interface::windows::WindowClass;
use crate::loaders::{FontSize, OverflowBehavior};
use crate::state::{ClientState, HomunculusInfo, PetInfo};
use crate::state::theme::InterfaceThemeType;

fn loyalty_bar(value: u16, max: u16) -> String {
    let ratio = if max > 0 { value as f32 / max as f32 } else { 0.0 };
    let filled = (ratio * 10.0) as usize;
    let empty = 10 - filled;
    let percent = (ratio * 100.0) as u16;
    format!(
        "{}{} {}%",
        "\u{2588}".repeat(filled),
        "\u{2591}".repeat(empty),
        percent
    )
}

struct PetInfoLayoutInfo {
    area: Area,
    lines: Vec<String>,
    line_heights: Vec<f32>,
}

struct PetInfoElement {
    pet_info: Option<PetInfo>,
    homunculus_info: Option<HomunculusInfo>,
}

impl PetInfoElement {
    fn new(pet_info: Option<PetInfo>, homunculus_info: Option<HomunculusInfo>) -> Self {
        Self { pet_info, homunculus_info }
    }

    fn build_lines(pet_info: &Option<PetInfo>, homunculus_info: &Option<HomunculusInfo>) -> Vec<String> {
        let mut lines = Vec::new();

        if pet_info.is_none() && homunculus_info.is_none() {
            lines.push("No active companion.".to_string());
            return lines;
        }

        if let Some(pet) = pet_info {
            lines.push(format!("^ff8800--- Pet: {} ---^000000", pet.name));
            lines.push(format!("Level: {}  |  Class: {}", pet.level, pet.class_id));
            lines.push(format!("Intimacy: {}", loyalty_bar(pet.intimacy, 1000)));
            lines.push(format!("Hunger: {}", loyalty_bar(pet.fullness, 100)));
        }

        if let Some(hom) = homunculus_info {
            lines.push(format!("^ff8800--- Homunculus: {} ---^000000", hom.name));
            lines.push(format!("Level: {}  |  {}", hom.level, if hom.alive { "Alive" } else { "Dead" }));
            lines.push(format!("HP: {}/{}  |  SP: {}/{}", hom.hp, hom.max_hp, hom.sp, hom.max_sp));
            lines.push(format!("Intimacy: {}", loyalty_bar(hom.intimacy, 1000)));
            lines.push(format!("Hunger: {}", loyalty_bar(hom.hunger, 100)));
        }

        lines
    }
}

impl Element<ClientState> for PetInfoElement {
    type LayoutInfo = PetInfoLayoutInfo;

    fn create_layout_info(
        &mut self,
        _state: &rust_state::Context<ClientState>,
        _: ElementStoreMut<'_>,
        resolver: &mut Resolver<'_, ClientState>,
    ) -> Self::LayoutInfo {
        let lines = Self::build_lines(&self.pet_info, &self.homunculus_info);
        let line_spacing = 4.0;
        let mut total_height = 0.0;
        let mut line_heights = Vec::new();

        for line in &lines {
            let (size, _) = resolver.get_text_dimensions(
                line,
                Color::monochrome_u8(220),
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
        PetInfoLayoutInfo { area, lines, line_heights }
    }

    fn lay_out<'a>(
        &'a self,
        _state: &'a rust_state::Context<ClientState>,
        _: ElementStore<'a>,
        layout_info: &'a Self::LayoutInfo,
        layout: &mut WindowLayout<'a, ClientState>,
    ) {
        let line_spacing = 4.0;
        let mut offset = 0.0;

        for (line, height) in layout_info.lines.iter().zip(layout_info.line_heights.iter()) {
            let color = if line.starts_with("^ff8800") {
                Color::rgb_u8(255, 136, 0)
            } else {
                Color::monochrome_u8(220)
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

/// Window displayed for pet and homunculus information.
pub struct PetInfoWindow {
    pet_info: Option<PetInfo>,
    homunculus_info: Option<HomunculusInfo>,
}

impl PetInfoWindow {
    pub fn new(pet_info: Option<PetInfo>, homunculus_info: Option<HomunculusInfo>) -> Self {
        Self { pet_info, homunculus_info }
    }
}

impl CustomWindow<ClientState> for PetInfoWindow {
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::PetInfo)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        let element = PetInfoElement::new(self.pet_info, self.homunculus_info);

        window! {
            title: "Companions",
            class: Some(WindowClass::PetInfo),
            theme: InterfaceThemeType::InGame,
            closable: true,
            minimum_width: 280.0,
            elements: (
                element,
                split! {
                    gaps: theme().window().gaps(),
                    children: (
                        button! { text: "Feed Pet", event: InputEvent::FeedPet },
                        button! { text: "Feed Hom.", event: InputEvent::FeedHomunculus },
                    ),
                },
            ),
        }
    }
}
