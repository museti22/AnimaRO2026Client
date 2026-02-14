use std::cmp::Ordering;

use korangar_interface::element::store::{ElementStore, ElementStoreMut};
use korangar_interface::element::{Element, ElementBox};
use korangar_interface::layout::{Resolver, WindowLayout};
use korangar_interface::window::{CustomWindow, Window};
use rust_state::{Context, ManuallyAssertExt, Path, Selector, VecIndexExt};

use crate::graphics::Color;
use crate::interface::windows::WindowClass;
use crate::loaders::OverflowBehavior;
use crate::state::theme::InterfaceThemeType;
use crate::state::{ClientState, PartyMember, PartyMemberPathExt};

struct LevelSelector<L> {
    level_path: L,
    text: std::cell::UnsafeCell<String>,
}

impl<L> LevelSelector<L> {
    fn new(level_path: L) -> Self {
        Self {
            level_path,
            text: std::cell::UnsafeCell::default(),
        }
    }
}

impl<L> Selector<ClientState, String> for LevelSelector<L>
where
    L: Path<ClientState, i16>,
{
    fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a String> {
        let level = self.level_path.follow(state).unwrap();
        unsafe {
            *self.text.get() = format!("Level: {}", level);
            Some(self.text.as_ref_unchecked())
        }
    }
}

struct JobSelector<J> {
    job_path: J,
    text: std::cell::UnsafeCell<String>,
}

impl<J> JobSelector<J> {
    fn new(job_path: J) -> Self {
        Self {
            job_path,
            text: std::cell::UnsafeCell::default(),
        }
    }
}

impl<J> Selector<ClientState, String> for JobSelector<J>
where
    J: Path<ClientState, i16>,
{
    fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a String> {
        let job = self.job_path.follow(state).unwrap();
        unsafe {
            *self.text.get() = format!("Job: {}", job);
            Some(self.text.as_ref_unchecked())
        }
    }
}

struct HpBarSelector<H, M> {
    hp_path: H,
    max_hp_path: M,
    text: std::cell::UnsafeCell<String>,
}

impl<H, M> HpBarSelector<H, M> {
    fn new(hp_path: H, max_hp_path: M) -> Self {
        Self {
            hp_path,
            max_hp_path,
            text: std::cell::UnsafeCell::default(),
        }
    }
}

impl<H, M> Selector<ClientState, String> for HpBarSelector<H, M>
where
    H: Path<ClientState, i32>,
    M: Path<ClientState, i32>,
{
    fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a String> {
        let hp = *self.hp_path.follow(state).unwrap();
        let max_hp = *self.max_hp_path.follow(state).unwrap();
        let percent = if max_hp > 0 {
            (hp as f64 / max_hp as f64 * 100.0).min(100.0)
        } else {
            0.0
        };
        unsafe {
            *self.text.get() = format!("HP: {} / {} ({:.0}%)", hp, max_hp, percent);
            Some(self.text.as_ref_unchecked())
        }
    }
}

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
    A: Path<ClientState, Vec<PartyMember>>,
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
                    let level_selector = LevelSelector::new(member_path.level());
                    let job_selector = JobSelector::new(member_path.job());
                    let hp_selector = HpBarSelector::new(member_path.health_points(), member_path.maximum_health_points());

                    self.elements.push(ErasedElement::new(collapsable! {
                        text: name_path,
                        children: (
                            text! { text: level_selector, overflow_behavior: OverflowBehavior::Shrink },
                            text! { text: job_selector, overflow_behavior: OverflowBehavior::Shrink },
                            text! {
                                text: hp_selector,
                                color: Color::rgb_u8(67, 163, 83),
                                overflow_behavior: OverflowBehavior::Shrink,
                            },
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

pub struct PartyWindow<P> {
    members_path: P,
}

impl<P> PartyWindow<P> {
    pub fn new(members_path: P) -> Self {
        Self { members_path }
    }
}

impl<P> CustomWindow<ClientState> for PartyWindow<P>
where
    P: Path<ClientState, Vec<PartyMember>>,
{
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::Party)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        window! {
            title: "Party",
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
