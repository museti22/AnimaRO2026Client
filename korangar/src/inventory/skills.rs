use std::sync::Arc;

use korangar_interface::element::StateElement;
use ragnarok_packets::{ClientTick, SkillId, SkillInformation, SkillLevel, SkillType};
use rust_state::RustState;

use crate::loaders::{ActionLoader, SkillTreeLoader, Sprite, SpriteLoader};
use crate::world::{Actions, SpriteAnimationState};

#[derive(Clone, Debug, RustState, StateElement)]
pub struct Skill {
    pub skill_id: SkillId,
    pub skill_level: SkillLevel,
    pub max_level: SkillLevel,
    pub skill_type: SkillType,
    pub skill_name: String,
    // TODO: Unhide this
    #[hidden_element]
    pub sprite: Arc<Sprite>,
    // TODO: Unhide this
    #[hidden_element]
    pub actions: Arc<Actions>,
    pub animation_state: SpriteAnimationState,
}

#[derive(Default, RustState, StateElement)]
pub struct SkillTree {
    skills: Vec<Skill>,
}

impl SkillTree {
    pub fn fill(
        &mut self,
        sprite_loader: &SpriteLoader,
        action_loader: &ActionLoader,
        skill_tree_loader: &SkillTreeLoader,
        learned_skills: Vec<SkillInformation>,
        job_id: u32,
        client_tick: ClientTick,
    ) {
        let dummy_sprite = sprite_loader
            .get_or_load("아이템\\Basic_Skill.spr")
            .unwrap_or_else(|_| sprite_loader.get_or_load("아이템\\Basic_Skill.spr").unwrap()); // Simplify this unwrapping later
        let dummy_actions = action_loader
            .get_or_load("아이템\\Basic_Skill.act")
            .unwrap_or_else(|_| action_loader.get_or_load("아이템\\Basic_Skill.act").unwrap());

        // Initialize 80 slots with dummy skills
        let mut skills: Vec<Skill> = (0..80)
            .map(|_| Skill {
                skill_id: SkillId(0),
                skill_level: SkillLevel(0),
                max_level: SkillLevel(0),
                skill_type: SkillType::Passive,
                skill_name: String::new(),
                sprite: dummy_sprite.clone(),
                actions: dummy_actions.clone(),
                animation_state: SpriteAnimationState::new(client_tick),
            })
            .collect();

        // 1. Get the static tree for this job
        let empty_tree = crate::loaders::skill_tree::LocalSkillTree { job_id, skills: vec![] };
        let local_tree = skill_tree_loader.get_tree(job_id).unwrap_or(&empty_tree);

        // 2. Place skills into the grid
        for entry in &local_tree.skills {
            let row = entry.row.unwrap_or(0);
            let col = entry.col.unwrap_or(0);
            let index = row * 10 + col;

            if index >= skills.len() {
                continue;
            }

            // Find if we have learned this skill
            let learned = learned_skills.iter().find(|s| s.skill_id.0 == entry.skill_id);

            let (skill_level, skill_type) = if let Some(learned) = learned {
                (learned.skill_level, learned.skill_type)
            } else {
                (SkillLevel(0), SkillType::Passive) // Default to 0/Passive if not known
            };

            let skill_name = learned.map(|s| s.skill_name.clone()).unwrap_or_else(|| entry.name.clone());

            let file_path = format!("아이템\\{}", skill_name);
            let sprite = sprite_loader
                .get_or_load(&format!("{file_path}.spr"))
                .unwrap_or(dummy_sprite.clone());
            let actions = action_loader
                .get_or_load(&format!("{file_path}.act"))
                .unwrap_or(dummy_actions.clone());

            skills[index] = Skill {
                skill_id: SkillId(entry.skill_id),
                skill_level,
                max_level: SkillLevel(entry.max_level),
                skill_type,
                skill_name,
                sprite,
                actions,
                animation_state: SpriteAnimationState::new(client_tick),
            };
        }

        self.skills = skills;
    }

    pub fn find_skill(&self, skill_id: SkillId) -> Option<Skill> {
        self.skills
            .iter()
            .find(|skill| skill.skill_id == skill_id && skill.skill_id.0 != 0)
            .cloned()
    }
}
