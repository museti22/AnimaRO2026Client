use std::cmp::Ordering;
use std::cell::UnsafeCell;

use korangar_interface::element::store::{ElementStore, ElementStoreMut};
use korangar_interface::element::{Element, ElementBox};
use korangar_interface::layout::{Resolver, WindowLayout};
use korangar_interface::window::{CustomWindow, Window};
use rust_state::{Context, ManuallyAssertExt, Path, Selector, VecIndexExt};

use crate::interface::windows::WindowClass;
use crate::loaders::OverflowBehavior;
use crate::state::theme::InterfaceThemeType;
use crate::state::{ActiveStatusEffect, ClientState};

/// Selector that formats the remaining time for a status effect.
struct RemainingTimeSelector<A> {
    effect_path: A,
    text: UnsafeCell<String>,
}

impl<A> RemainingTimeSelector<A> {
    fn new(effect_path: A) -> Self {
        Self {
            effect_path,
            text: UnsafeCell::new(String::new()),
        }
    }
}

impl<A> Selector<ClientState, String> for RemainingTimeSelector<A>
where
    A: Path<ClientState, ActiveStatusEffect>,
{
    fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a String> {
        let effect = self.effect_path.follow(state)?;
        let remaining = effect.remaining_text();

        // SAFETY: Single-threaded UI.
        unsafe {
            *self.text.get() = if remaining.is_empty() {
                effect.name.clone()
            } else {
                format!("{} ({})", effect.name, remaining)
            };
            Some(self.text.as_ref_unchecked())
        }
    }
}

struct StatusEffectList<A> {
    effects_path: A,
    elements: Vec<ElementBox<ClientState>>,
}

impl<A> StatusEffectList<A> {
    fn new(effects_path: A) -> Self {
        Self {
            effects_path,
            elements: Vec::new(),
        }
    }
}

impl<A> Element<ClientState> for StatusEffectList<A>
where
    A: Path<ClientState, Vec<ActiveStatusEffect>>,
{
    type LayoutInfo = ();

    fn create_layout_info(
        &mut self,
        state: &Context<ClientState>,
        mut store: ElementStoreMut<'_>,
        resolver: &mut Resolver<'_, ClientState>,
    ) -> Self::LayoutInfo {
        use korangar_interface::prelude::*;

        let effects = state.get(&self.effects_path);

        match effects.len().cmp(&self.elements.len()) {
            Ordering::Less => {
                self.elements.truncate(effects.len());
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                for index in self.elements.len()..effects.len() {
                    let effect_path = self.effects_path.index(index).manually_asserted();

                    self.elements.push(ErasedElement::new(text! {
                        text: RemainingTimeSelector::new(effect_path),
                        overflow_behavior: OverflowBehavior::Shrink,
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

pub struct StatusEffectsWindow<P> {
    effects_path: P,
}

impl<P> StatusEffectsWindow<P> {
    pub fn new(effects_path: P) -> Self {
        Self { effects_path }
    }
}

impl<P> CustomWindow<ClientState> for StatusEffectsWindow<P>
where
    P: Path<ClientState, Vec<ActiveStatusEffect>>,
{
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::StatusEffects)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        window! {
            title: "Status Effects",
            class: Self::window_class(),
            theme: InterfaceThemeType::InGame,
            closable: true,
            elements: [
                scroll_view! {
                    children: (
                        StatusEffectList::new(self.effects_path),
                    ),
                },
            ],
        }
    }
}
