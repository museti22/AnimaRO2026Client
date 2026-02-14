use std::cmp::Ordering;

use korangar_interface::element::store::{ElementStore, ElementStoreMut};
use korangar_interface::element::{Element, ElementBox};
use korangar_interface::layout::{Resolver, WindowLayout};
use korangar_interface::window::{CustomWindow, Window};
use rust_state::{Context, ManuallyAssertExt, Path, Selector, VecIndexExt};

use crate::interface::windows::WindowClass;
use crate::state::theme::InterfaceThemeType;
use crate::state::{ClientState, GuildInfo, GuildMember, GuildMemberPathExt};

/// Selector that formats a guild member's level.
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
    L: Path<ClientState, u16>,
{
    fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a String> {
        let level = self.level_path.follow(state).unwrap();
        unsafe {
            *self.text.get() = format!("Level: {}", level);
            Some(self.text.as_ref_unchecked())
        }
    }
}

/// Selector that formats a guild member's job class.
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

/// Selector that formats a guild member's online status.
struct OnlineSelector<O> {
    online_path: O,
    text: std::cell::UnsafeCell<String>,
}

impl<O> OnlineSelector<O> {
    fn new(online_path: O) -> Self {
        Self {
            online_path,
            text: std::cell::UnsafeCell::default(),
        }
    }
}

impl<O> Selector<ClientState, String> for OnlineSelector<O>
where
    O: Path<ClientState, bool>,
{
    fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a String> {
        let online = self.online_path.follow(state).unwrap();
        unsafe {
            *self.text.get() = if *online {
                "Status: Online".to_string()
            } else {
                "Status: Offline".to_string()
            };
            Some(self.text.as_ref_unchecked())
        }
    }
}

/// Selector for the guild header info text.
struct GuildHeaderSelector<I> {
    guild_info_path: I,
    text: std::cell::UnsafeCell<String>,
}

impl<I> GuildHeaderSelector<I> {
    fn new(guild_info_path: I) -> Self {
        Self {
            guild_info_path,
            text: std::cell::UnsafeCell::default(),
        }
    }
}

impl<I> Selector<ClientState, String> for GuildHeaderSelector<I>
where
    I: Path<ClientState, GuildInfo>,
{
    fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a String> {
        let info = self.guild_info_path.follow(state).unwrap();
        unsafe {
            if info.guild_name.is_empty() {
                *self.text.get() = "Not in a guild".to_string();
            } else {
                *self.text.get() = format!(
                    "{} (Lv.{}) - Master: {} - Members: {}/{}",
                    info.guild_name, info.guild_level, info.master_name,
                    info.member_count, info.max_member_count,
                );
            }
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
    A: Path<ClientState, Vec<GuildMember>>,
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
                    let online_selector = OnlineSelector::new(member_path.online());

                    self.elements.push(ErasedElement::new(collapsable! {
                        text: name_path,
                        children: (
                            text! { text: member_path.position() },
                            text! { text: level_selector },
                            text! { text: job_selector },
                            text! { text: online_selector },
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

pub struct GuildWindow<P, I> {
    members_path: P,
    guild_info_path: I,
}

impl<P, I> GuildWindow<P, I> {
    pub fn new(members_path: P, guild_info_path: I) -> Self {
        Self {
            members_path,
            guild_info_path,
        }
    }
}

impl<P, I> CustomWindow<ClientState> for GuildWindow<P, I>
where
    P: Path<ClientState, Vec<GuildMember>>,
    I: Path<ClientState, GuildInfo>,
{
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::Guild)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        let header_selector = GuildHeaderSelector::new(self.guild_info_path);

        window! {
            title: "Guild",
            class: Self::window_class(),
            theme: InterfaceThemeType::InGame,
            closable: true,
            minimum_width: 300.0,
            elements: (
                text! { text: header_selector },
                scroll_view! {
                    children: (
                        MemberList::new(self.members_path),
                    ),
                },
            ),
        }
    }
}
