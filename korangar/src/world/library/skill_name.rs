use std::borrow::Cow;
use std::fmt::{Display, Formatter};

use hashbrown::HashMap;
use korangar_loaders::FileLoader;
use mlua::Lua;
use ragnarok_packets::SkillId;

use super::{Library, Table};
use crate::loaders::GameFileLoader;

/// Maps a skill ID to its constant name (e.g. SkillId(1) → "NV_BASIC").
/// The constant name is used as the sprite file name in the 아이템 directory.
/// Loaded dynamically from SkillID.lub.
pub struct SkillName(Cow<'static, str>);

impl Display for SkillName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Table for SkillName {
    type Key<'a> = SkillId;
    type Storage = HashMap<SkillId, SkillName>;

    fn load(game_file_loader: &GameFileLoader) -> mlua::Result<Self::Storage> {
        let state = Lua::new();

        // Try multiple paths - different GRFs use different casing
        let paths = [
            "data\\luafiles514\\lua files\\skillinfoz\\skillid.lub",
            "data\\luafiles514\\lua files\\skillInfoz\\SkillID.lub",
        ];

        let mut loaded = false;
        for path in &paths {
            if let Ok(data) = game_file_loader.get(path) {
                state.load(&data).exec()?;
                loaded = true;
                break;
            }
        }

        if !loaded {
            eprintln!("[SkillName] WARNING: Could not load SkillID.lub from any path");
            return Ok(HashMap::new());
        }

        let globals = state.globals();
        let mut result = HashMap::new();

        // SkillID.lub defines: SKID = { NV_BASIC = 1, SM_SWORD = 2, ... }
        // We need to invert it: 1 → "NV_BASIC", 2 → "SM_SWORD", etc.
        if let Ok(skid_table) = globals.get::<mlua::Table>("SKID") {
            for pair in skid_table.pairs::<String, u32>().flatten() {
                let (name, id) = pair;
                result.insert(SkillId(id as u16), SkillName(Cow::Owned(name)));
            }
        }

        eprintln!("[SkillName] Loaded {} skill name mappings from SkillID.lub", result.len());

        Ok(HashMap::from_iter(result))
    }

    fn try_get<'a, 'b>(library: &'a Library, key: Self::Key<'b>) -> Option<&'a Self> {
        library.skill_name_table.get(&key)
    }

    fn get<'a, 'b>(library: &'a Library, key: Self::Key<'b>) -> &'a Self {
        static DEFAULT: SkillName = SkillName(Cow::Borrowed("UNKNOWN_SKILL"));
        Self::try_get(library, key).unwrap_or(&DEFAULT)
    }
}
