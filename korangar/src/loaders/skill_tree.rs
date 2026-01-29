use std::collections::HashMap;
use std::sync::Arc;


use serde::Deserialize;

use crate::loaders::GameFileLoader;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct SkillRequirement {
    pub skill_id: u16,
    pub min_level: u16,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct LocalSkillEntry {
    pub skill_id: u16,
    pub name: String,
    pub max_level: u16,
    pub requirements: Vec<SkillRequirement>,
    // Helper visual position if we want to code it, or we rely on auto-layout
    pub row: Option<usize>,
    pub col: Option<usize>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct LocalSkillTree {
    pub job_id: u32, // Using u32 to map to Job
    pub skills: Vec<LocalSkillEntry>,
}

pub struct SkillTreeLoader {
    // Map JobId -> Skill Tree
    cache: HashMap<u32, LocalSkillTree>,
}

impl SkillTreeLoader {
    pub fn new() -> Self {
        Self { cache: HashMap::new() }
    }

    pub fn load(&mut self, _game_file_loader: &Arc<GameFileLoader>) {
        // TODO: Load from actual file (data/skill_tree.ron)
        // For now, we populate with some defaults or load a dummy

        // Example: hardcoded common First Class tree (Swordman) just for testing
        // You should implement the full reading from a RON/JSON file here.
        let swordman_tree = LocalSkillTree {
            job_id: 1, // Swordman
            skills: vec![
                LocalSkillEntry {
                    skill_id: 1, // Basic Skill
                    name: "NV_BASIC".to_string(),
                    max_level: 9,
                    requirements: vec![],
                    row: Some(0),
                    col: Some(0),
                },
                LocalSkillEntry {
                    skill_id: 2, // Sword Mastery
                    name: "SM_SWORD".to_string(),
                    max_level: 10,
                    requirements: vec![],
                    row: Some(1),
                    col: Some(1),
                },
                LocalSkillEntry {
                    skill_id: 3, // Two Hand Mastery
                    name: "SM_TWOHAND".to_string(),
                    max_level: 10,
                    requirements: vec![SkillRequirement { skill_id: 2, min_level: 1 }],
                    row: Some(1),
                    col: Some(2),
                },
            ],
        };

        self.cache.insert(1, swordman_tree);
    }

    pub fn get_tree(&self, job_id: u32) -> Option<&LocalSkillTree> {
        self.cache.get(&job_id)
    }
}
