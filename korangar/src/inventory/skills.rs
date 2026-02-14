use std::sync::Arc;

use korangar_interface::element::StateElement;
use ragnarok_packets::{ClientTick, SkillId, SkillInf, SkillInformation, SkillLevel};
use rust_state::RustState;

use crate::loaders::{ActionLoader, Sprite, SpriteLoader};
use crate::world::{Actions, SpriteAnimationState};

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

/// Map a skill ID to the sprite/icon name used in the 아이템 directory.
/// In the pre-renewal 15-byte skill info format, the server does NOT send
/// skill names. The client must derive them from the skill ID.
fn skill_icon_name(skill_id: SkillId) -> &'static str {
    match skill_id.0 {
        // -- Basic / Novice --
        1 => "NV_BASIC",
        // -- Swordsman --
        2 => "SM_SWORD", 3 => "SM_TWOHAND", 4 => "SM_RECOVERY", 5 => "SM_BASH",
        6 => "SM_PROVOKE", 7 => "SM_MAGNUM", 8 => "SM_ENDURE",
        // -- Mage --
        9 => "MG_SRECOVERY", 10 => "MG_SIGHT", 11 => "MG_NAPALMBEAT",
        12 => "MG_SAFETYWALL", 13 => "MG_SOULSTRIKE", 14 => "MG_COLDBOLT",
        15 => "MG_FROSTDIVER", 16 => "MG_STONECURSE", 17 => "MG_FIREBALL",
        18 => "MG_FIREWALL", 19 => "MG_FIREBOLT", 20 => "MG_LIGHTNINGBOLT",
        21 => "MG_THUNDERSTORM",
        // -- Archer --
        22 => "AC_OWL", 23 => "AC_VULTURE", 24 => "AC_CONCENTRATION",
        25 => "AC_DOUBLE", 26 => "AC_SHOWER",
        // -- Acolyte --
        27 => "AL_DP", 28 => "AL_HEAL", 29 => "AL_INCAGI", 30 => "AL_DECAGI",
        31 => "AL_HOLYWATER", 32 => "AL_RUWACH", 33 => "AL_WARP",
        34 => "AL_PNEUMA", 35 => "AL_TELEPORT", 36 => "AL_CURE",
        37 => "AL_BLESSING",
        // -- Merchant --
        38 => "MC_INCCARRY", 39 => "MC_DISCOUNT", 40 => "MC_OVERCHARGE",
        41 => "MC_PUSHCART", 42 => "MC_IDENTIFY", 43 => "MC_VENDING",
        44 => "MC_MAMMONITE",
        // -- Thief --
        45 => "TF_DOUBLE", 46 => "TF_MISS", 47 => "TF_STEAL",
        48 => "TF_HIDING", 49 => "TF_POISON", 50 => "TF_DETOXIFY",
        51 => "TF_SPRINKLESAND", 52 => "TF_BACKSLIDING", 53 => "TF_PICKSTONE",
        54 => "TF_THROWSTONE",
        // -- Knight --
        55 => "KN_SPEARMASTERY", 56 => "KN_PIERCE", 57 => "KN_BRANDISHSPEAR",
        58 => "KN_SPEARSTAB", 59 => "KN_SPEARBOOMERANG", 60 => "KN_TWOHANDQUICKEN",
        61 => "KN_AUTOCOUNTER", 62 => "KN_BOWLINGBASH", 63 => "KN_RIDING",
        64 => "KN_CAVALIERMASTERY",
        // -- Priest --
        66 => "PR_MACEMASTERY", 67 => "PR_IMPOSITIO", 68 => "PR_SUFFRAGIUM",
        69 => "PR_ASPERSIO", 70 => "PR_BENEDICTIO", 71 => "PR_SANCTUARY",
        72 => "PR_SLOWPOISON", 73 => "PR_STRECOVERY", 74 => "PR_KYRIE",
        75 => "PR_MAGNIFICAT", 76 => "PR_GLORIA", 77 => "PR_LEXDIVINA",
        78 => "PR_TURNUNDEAD", 79 => "PR_LEXAETERNA", 80 => "PR_MAGNUS",
        // -- Wizard --
        81 => "WZ_FIREPILLAR", 82 => "WZ_SIGHTRASHER", 83 => "WZ_FIREIVY",
        84 => "WZ_METEOR", 85 => "WZ_JUPITEL", 86 => "WZ_VERMILION",
        87 => "WZ_WATERBALL", 88 => "WZ_ICEWALL", 89 => "WZ_FROSTNOVA",
        90 => "WZ_STORMGUST", 91 => "WZ_EARTHSPIKE", 92 => "WZ_HEAVENDRIVE",
        93 => "WZ_QUAGMIRE", 94 => "WZ_ESTIMATION",
        // -- Blacksmith --
        95 => "BS_IRON", 96 => "BS_STEEL", 97 => "BS_ENCHANTEDSTONE",
        98 => "BS_ORIDEOCON", 99 => "BS_DAGGER", 100 => "BS_SWORD",
        101 => "BS_TWOHANDSWORD", 102 => "BS_AXE", 103 => "BS_MACE",
        104 => "BS_KNUCKLE", 105 => "BS_HAMMERFALL", 106 => "BS_ADRENALINE",
        107 => "BS_WEAPONPERFECT", 108 => "BS_OVERTHRUST", 109 => "BS_MAXIMIZE",
        // -- Hunter --
        110 => "HT_SKIDTRAP", 111 => "HT_LANDMINE", 112 => "HT_ANKLESNARE",
        113 => "HT_SHOCKWAVE", 114 => "HT_SANDMAN", 115 => "HT_FLASHER",
        116 => "HT_FREEZINGTRAP", 117 => "HT_BLASTMINE", 118 => "HT_CLAYMORETRAP",
        119 => "HT_REMOVETRAP", 120 => "HT_TALKIEBOX", 121 => "HT_BEASTBANE",
        122 => "HT_FALCON", 123 => "HT_STEELCROW", 124 => "HT_BLITZBEAT",
        125 => "HT_DETECTING", 126 => "HT_SPRINGTRAP",
        // -- Assassin --
        127 => "AS_RIGHT", 128 => "AS_LEFT", 129 => "AS_KATAR",
        130 => "AS_CLOAKING", 131 => "AS_SONICBLOW", 132 => "AS_GRIMTOOTH",
        133 => "AS_ENCHANTPOISON", 134 => "AS_POISONREACT", 135 => "AS_VENOMDUST",
        136 => "AS_SPLASHER",
        // -- Crusader --
        248 => "CR_TRUST", 249 => "CR_AUTOGUARD", 250 => "CR_SHIELDCHARGE",
        251 => "CR_SHIELDBOOMERANG", 252 => "CR_REFLECTSHIELD", 253 => "CR_HOLYCROSS",
        254 => "CR_GRANDCROSS", 255 => "CR_DEVOTION", 256 => "CR_PROVIDENCE",
        257 => "CR_DEFENDER", 258 => "CR_SPEARQUICKEN",
        // -- Monk --
        259 => "MO_IRONHAND", 260 => "MO_SPIRITSRECOVERY", 261 => "MO_CALLSPIRITS",
        262 => "MO_ABSORBSPIRITS", 263 => "MO_TRIPLEATTACK", 264 => "MO_BODYRELOCATION",
        265 => "MO_DODGE", 266 => "MO_INVESTIGATE", 267 => "MO_FINGEROFFENSIVE",
        268 => "MO_STEELBODY", 269 => "MO_BLADESTOP", 270 => "MO_EXPLOSIONSPIRITS",
        271 => "MO_EXTREMITYFIST", 272 => "MO_CHAINCOMBO", 273 => "MO_COMBOFINISH",
        // -- Sage --
        274 => "SA_ADVANCEDBOOK", 275 => "SA_CASTCANCEL", 276 => "SA_MAGICROD",
        277 => "SA_SPELLBREAKER", 278 => "SA_FREECAST", 279 => "SA_AUTOSPELL",
        280 => "SA_FLAMELAUNCHER", 281 => "SA_FROSTWEAPON", 282 => "SA_LIGHTNINGLOADER",
        283 => "SA_SEISMICWEAPON", 284 => "SA_DRAGONOLOGY", 285 => "SA_VOLCANO",
        286 => "SA_DELUGE", 287 => "SA_VIOLENTGALE", 288 => "SA_LANDPROTECTOR",
        289 => "SA_DISPELL", 290 => "SA_ABRACADABRA",
        // -- Rogue --
        210 => "RG_SNATCHER", 211 => "RG_STEALCOIN", 212 => "RG_BACKSTAP",
        213 => "RG_TUNNELDRIVE", 214 => "RG_RAID", 215 => "RG_STRIPWEAPON",
        216 => "RG_STRIPSHIELD", 217 => "RG_STRIPARMOR", 218 => "RG_STRIPHELM",
        219 => "RG_INTIMIDATE", 220 => "RG_GRAFFITI", 221 => "RG_FLAGGRAFFITI",
        222 => "RG_CLEANER", 223 => "RG_GANGSTER", 224 => "RG_COMPULSION",
        225 => "RG_PLAGIARISM",
        // -- Alchemist --
        226 => "AM_AXEMASTERY", 227 => "AM_LEARNINGPOTION", 228 => "AM_PHARMACY",
        229 => "AM_DEMONSTRATION", 230 => "AM_ACIDTERROR", 231 => "AM_POTIONPITCHER",
        232 => "AM_CANNIBALIZE", 233 => "AM_SPHEREMINE", 234 => "AM_CP_WEAPON",
        235 => "AM_CP_SHIELD", 236 => "AM_CP_ARMOR", 237 => "AM_CP_HELM",
        // -- Bard --
        304 => "BA_MUSICALLESSON", 305 => "BA_MUSICALSTRIKE", 306 => "BA_DISSONANCE",
        307 => "BA_FROSTJOKER", 308 => "BA_WHISTLE", 309 => "BA_ASSASSINCROSS",
        310 => "BA_POEMBRAGI", 311 => "BA_APPLEIDUN",
        // -- Dancer --
        312 => "DC_DANCINGLESSON", 313 => "DC_THROWARROW", 314 => "DC_UGLYDANCE",
        315 => "DC_SCREAM", 316 => "DC_HUMMING", 317 => "DC_DONTFORGETME",
        318 => "DC_FORTUNEKISS", 319 => "DC_SERVICEFORYOU",
        // -- Lord Knight --
        355 => "LK_AURABLADE", 356 => "LK_PARRYING", 357 => "LK_CONCENTRATION",
        358 => "LK_TENSIONRELAX", 359 => "LK_BERSERK",
        360 => "LK_SPIRALPIERCE", 361 => "LK_HEADCRUSH", 362 => "LK_JOINTBEAT",
        // -- High Priest --
        363 => "HP_ASSUMPTIO", 364 => "HP_BASILICA", 365 => "HP_MEDITATIO",
        366 => "HP_MANARECHARGE",
        // -- High Wizard --
        367 => "HW_SOULDRAIN", 368 => "HW_MAGICCRASHER", 369 => "HW_MAGICPOWER",
        370 => "HW_NAPALMVULCAN",
        // -- Paladin --
        371 => "PA_PRESSURE", 372 => "PA_SACRIFICE", 373 => "PA_GOSPEL",
        // -- Champion --
        374 => "CH_PALMSTRIKE", 375 => "CH_TIGERFIST", 376 => "CH_CHAINCRUSH",
        // -- Professor --
        377 => "PF_HPCONVERSION", 378 => "PF_SOULCHANGE", 379 => "PF_SOULBURN",
        // -- Stalker --
        380 => "SL_KAIZEL", 381 => "SL_KAAHI", 382 => "SL_KAUPE",
        383 => "SL_KAITE",
        // -- Assassin Cross --
        386 => "ASC_KATAR", 387 => "ASC_EDP",
        388 => "ASC_BREAKER", 389 => "ASC_METEORASSAULT",
        // -- Whitesmith --
        390 => "WS_MELTDOWN", 391 => "WS_CARTBOOST",
        392 => "WS_SYSTEMCREATE", 393 => "WS_OVERTHRUSTMAX",
        // -- Sniper --
        394 => "SN_SIGHT", 395 => "SN_FALCONASSAULT",
        396 => "SN_SHARPSHOOTING", 397 => "SN_WINDWALK",
        // -- Creator --
        398 => "CR_SLIMPITCHER",
        // -- Additional classes: add as needed from skillinfolist --
        // -- Common fallback --
        _ => "UNKNOWN_SKILL",
    }
}

impl SkillTree {
    pub fn fill(
        &mut self,
        sprite_loader: &SpriteLoader,
        action_loader: &ActionLoader,
        skill_information: Vec<SkillInformation>,
        client_tick: ClientTick,
    ) {
        self.skills = skill_information
            .into_iter()
            .filter_map(|skill_information| {
                let name = skill_icon_name(skill_information.skill_id);
                let file_path = format!("아이템\\{name}");
                let sprite = sprite_loader.get_or_load(&format!("{file_path}.spr")).ok()?;
                let actions = action_loader.get_or_load(&format!("{file_path}.act")).ok()?;

                Some(Skill {
                    skill_id: skill_information.skill_id,
                    skill_level: skill_information.skill_level,
                    skill_inf: skill_information.skill_inf,
                    skill_name: name.to_string(),
                    sprite,
                    actions,
                    animation_state: SpriteAnimationState::new(client_tick),
                    cooldown_until: ClientTick(0),
                })
            })
            .collect();
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
