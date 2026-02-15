use std::sync::Arc;

use korangar_interface::element::StateElement;
use ragnarok_packets::{ClientTick, SkillId, SkillInf, SkillInformation, SkillLevel};
use rust_state::RustState;

use crate::loaders::{ActionLoader, Sprite, SpriteLoader};
use crate::world::{Actions, Library, SkillName, SpriteAnimationState};

#[derive(Clone, Debug, RustState, StateElement)]
pub struct Skill {
    pub skill_id: SkillId,
    pub skill_level: SkillLevel,
    pub skill_inf: SkillInf,
    pub skill_name: String,
    // TODO: Unhide this
    #[hidden_element]
    pub sprite: Arc<Sprite>,
    // TODO: Unhide this
    #[hidden_element]
    pub actions: Arc<Actions>,
    pub animation_state: SpriteAnimationState,
    /// Tick at which the cooldown expires. 0 means no cooldown.
    pub cooldown_until: ClientTick,
}

#[derive(Default, RustState, StateElement)]
pub struct SkillTree {
    skills: Vec<Skill>,
}

impl SkillTree {
    /// Fill the skill tree from server data. Skill icon names are loaded
    /// dynamically from SkillID.lub via the Library, not hardcoded.
    pub fn fill(
        &mut self,
        sprite_loader: &SpriteLoader,
        action_loader: &ActionLoader,
        skill_information: Vec<SkillInformation>,
        client_tick: ClientTick,
        library: &Library,
    ) {
        self.skills = skill_information
            .into_iter()
            .filter_map(|skill_information| {
                let name = library.get::<SkillName>(skill_information.skill_id).to_string();

                if name == "UNKNOWN_SKILL" {
                    eprintln!(
                        "[SkillTree] WARNING: No icon mapping for skill_id={}, skipping",
                        skill_information.skill_id.0
                    );
                    return None;
                }

                let file_path = format!("아이템\\{name}");
                let sprite = sprite_loader.get_or_load(&format!("{file_path}.spr")).ok()?;
                let actions = action_loader.get_or_load(&format!("{file_path}.act")).ok()?;

                Some(Skill {
                    skill_id: skill_information.skill_id,
                    skill_level: skill_information.skill_level,
                    skill_inf: skill_information.skill_inf,
                    skill_name: name,
                    sprite,
                    actions,
                    animation_state: SpriteAnimationState::new(client_tick),
                    cooldown_until: ClientTick(0),
                })
            })
            .collect();
    }

    pub fn skill_count(&self) -> usize {
        self.skills.len()
    }

    pub fn find_skill(&self, skill_id: SkillId) -> Option<Skill> {
        self.skills.iter().find(|skill| skill.skill_id == skill_id).cloned()
    }

    pub fn set_cooldown(&mut self, skill_id: SkillId, until: ClientTick) {
        for skill in &mut self.skills {
            if skill.skill_id == skill_id {
                skill.cooldown_until = until;
            }
        }
    }
}
