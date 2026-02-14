use std::cell::UnsafeCell;

use korangar_interface::window::{CustomWindow, Window};
use rust_state::{Path, Selector};

use crate::graphics::Color;
use crate::interface::windows::WindowClass;
use crate::loaders::OverflowBehavior;
use crate::state::theme::InterfaceThemeType;
use crate::state::ClientState;
use crate::world::{CommonPathExt, Player, PlayerPathExt};

struct BarTextSelector<C, M> {
    current_path: C,
    max_path: M,
    label: &'static str,
    text: UnsafeCell<String>,
}

impl<C, M> BarTextSelector<C, M> {
    fn new(current_path: C, max_path: M, label: &'static str) -> Self {
        Self {
            current_path,
            max_path,
            label,
            text: UnsafeCell::default(),
        }
    }
}

impl<C, M> Selector<ClientState, String> for BarTextSelector<C, M>
where
    C: Path<ClientState, usize>,
    M: Path<ClientState, usize>,
{
    fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a String> {
        let current = *self.current_path.follow(state)?;
        let max = *self.max_path.follow(state)?;
        let percent = if max > 0 {
            (current as f64 / max as f64 * 100.0).min(100.0)
        } else {
            0.0
        };
        unsafe {
            *self.text.get() = format!("{}: {} / {} ({:.0}%)", self.label, current, max, percent);
            Some(self.text.as_ref_unchecked())
        }
    }
}

struct ExpBarTextSelector<C, N> {
    current_path: C,
    next_path: N,
    label: &'static str,
    text: UnsafeCell<String>,
}

impl<C, N> ExpBarTextSelector<C, N> {
    fn new(current_path: C, next_path: N, label: &'static str) -> Self {
        Self {
            current_path,
            next_path,
            label,
            text: UnsafeCell::default(),
        }
    }
}

impl<C, N> Selector<ClientState, String> for ExpBarTextSelector<C, N>
where
    C: Path<ClientState, u64>,
    N: Path<ClientState, u64>,
{
    fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a String> {
        let current = *self.current_path.follow(state)?;
        let next = *self.next_path.follow(state)?;
        let percent = if next > 0 {
            (current as f64 / next as f64 * 100.0).min(100.0)
        } else {
            0.0
        };
        unsafe {
            *self.text.get() = format!("{}: {:.1}%", self.label, percent);
            Some(self.text.as_ref_unchecked())
        }
    }
}

struct WeightTextSelector<W, M> {
    weight_path: W,
    max_weight_path: M,
    text: UnsafeCell<String>,
}

impl<W, M> WeightTextSelector<W, M> {
    fn new(weight_path: W, max_weight_path: M) -> Self {
        Self {
            weight_path,
            max_weight_path,
            text: UnsafeCell::default(),
        }
    }
}

impl<W, M> Selector<ClientState, String> for WeightTextSelector<W, M>
where
    W: Path<ClientState, u32>,
    M: Path<ClientState, u32>,
{
    fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a String> {
        let weight = *self.weight_path.follow(state)?;
        let max_weight = *self.max_weight_path.follow(state)?;
        let percent = if max_weight > 0 {
            (weight as f64 / max_weight as f64 * 100.0).min(100.0)
        } else {
            0.0
        };
        unsafe {
            *self.text.get() = format!("Weight: {} / {} ({:.0}%)", weight, max_weight, percent);
            Some(self.text.as_ref_unchecked())
        }
    }
}

struct ZenyTextSelector<Z> {
    zeny_path: Z,
    text: UnsafeCell<String>,
}

impl<Z> ZenyTextSelector<Z> {
    fn new(zeny_path: Z) -> Self {
        Self {
            zeny_path,
            text: UnsafeCell::default(),
        }
    }
}

impl<Z> Selector<ClientState, String> for ZenyTextSelector<Z>
where
    Z: Path<ClientState, u32>,
{
    fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a String> {
        let zeny = *self.zeny_path.follow(state)?;
        unsafe {
            *self.text.get() = format!("Zeny: {}", zeny);
            Some(self.text.as_ref_unchecked())
        }
    }
}

struct LevelTextSelector<B, J> {
    base_path: B,
    job_path: J,
    text: UnsafeCell<String>,
}

impl<B, J> LevelTextSelector<B, J> {
    fn new(base_path: B, job_path: J) -> Self {
        Self {
            base_path,
            job_path,
            text: UnsafeCell::default(),
        }
    }
}

impl<B, J> Selector<ClientState, String> for LevelTextSelector<B, J>
where
    B: Path<ClientState, usize>,
    J: Path<ClientState, usize>,
{
    fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a String> {
        let base = *self.base_path.follow(state)?;
        let job = *self.job_path.follow(state)?;
        unsafe {
            *self.text.get() = format!("Lv {} / Job {}", base, job);
            Some(self.text.as_ref_unchecked())
        }
    }
}

#[derive(Default)]
pub struct HudWindow<A> {
    player_path: A,
}

impl<A> HudWindow<A> {
    pub fn new(player_path: A) -> Self {
        Self { player_path }
    }
}

impl<A> CustomWindow<ClientState> for HudWindow<A>
where
    A: Path<ClientState, Player>,
{
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::Hud)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        window! {
            title: "HUD",
            class: Self::window_class(),
            theme: InterfaceThemeType::InGame,
            closable: false,
            minimum_width: 220.0,
            maximum_width: 220.0,
            elements: (
                text! {
                    text: LevelTextSelector::new(self.player_path.base_level(), self.player_path.job_level()),
                    color: Color::rgb_u8(13, 231, 255),
                    overflow_behavior: OverflowBehavior::Shrink,
                },
                text! {
                    text: BarTextSelector::new(
                        self.player_path.common().health_points(),
                        self.player_path.common().maximum_health_points(),
                        "HP",
                    ),
                    color: Color::rgb_u8(67, 163, 83),
                    overflow_behavior: OverflowBehavior::Shrink,
                },
                text! {
                    text: BarTextSelector::new(
                        self.player_path.spell_points(),
                        self.player_path.maximum_spell_points(),
                        "SP",
                    ),
                    color: Color::rgb_u8(0, 129, 163),
                    overflow_behavior: OverflowBehavior::Shrink,
                },
                text! {
                    text: ExpBarTextSelector::new(
                        self.player_path.base_experience(),
                        self.player_path.next_base_experience(),
                        "Base EXP",
                    ),
                    color: Color::rgb_u8(255, 200, 50),
                    overflow_behavior: OverflowBehavior::Shrink,
                },
                text! {
                    text: ExpBarTextSelector::new(
                        self.player_path.job_experience(),
                        self.player_path.next_job_experience(),
                        "Job EXP",
                    ),
                    color: Color::rgb_u8(255, 200, 50),
                    overflow_behavior: OverflowBehavior::Shrink,
                },
                text! {
                    text: WeightTextSelector::new(
                        self.player_path.weight(),
                        self.player_path.maximum_weight(),
                    ),
                    color: Color::rgb_u8(218, 145, 81),
                    overflow_behavior: OverflowBehavior::Shrink,
                },
                text! {
                    text: ZenyTextSelector::new(self.player_path.zeny()),
                    color: Color::rgb_u8(255, 220, 100),
                    overflow_behavior: OverflowBehavior::Shrink,
                },
            ),
        }
    }
}
