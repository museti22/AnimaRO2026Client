

use hashbrown::HashMap;
use korangar_loaders::FileLoader;

use crate::loaders::GameFileLoader;

#[derive(Default)]
pub struct MapNameTable {
    names: HashMap<String, String>,
}

#[cfg(feature = "debug")]
use korangar_debug::logging::{Colorize, print_debug};

impl MapNameTable {
    pub fn new(game_file_loader: &GameFileLoader) -> Self {
        let mut names = HashMap::new();

        #[cfg(feature = "debug")]
        print_debug!("Attempting to load mapnametable.txt");

        if let Ok(bytes) = game_file_loader.get("data\\mapnametable.txt") {
            let content = String::from_utf8_lossy(&bytes);
            #[cfg(feature = "debug")]
            print_debug!("Loaded mapnametable.txt, size: {}", content.len());

            for line in content.lines() {
                if line.starts_with("//") {
                    continue;
                }

                let parts: Vec<&str> = line.split('#').collect();
                if parts.len() >= 2 {
                    let actual_map = parts[0].trim();
                    let alias_map = parts[1].trim();

                    if !actual_map.is_empty() && !alias_map.is_empty() {
                        // Key = Map name requested by server (parts[0])
                        // Value = Actual file to load (parts[1])
                        names.insert(actual_map.to_lowercase(), alias_map.to_string());
                        #[cfg(feature = "debug")]
                        print_debug!("Added map alias: {} -> {}", actual_map, alias_map);
                    }
                }
            }
        } else {
            #[cfg(feature = "debug")]
            print_debug!("Failed to load data\\mapnametable.txt");
        }

        Self { names }
    }

    pub fn resolve(&self, map_name: &str) -> String {
        let key = if map_name.ends_with(".rsw") {
            map_name.to_lowercase()
        } else {
            format!("{}.rsw", map_name).to_lowercase()
        };

        match self.names.get(&key) {
            Some(resolved) => {
                let stripped = resolved.strip_suffix(".rsw").unwrap_or(resolved).to_string();
                #[cfg(feature = "debug")]
                print_debug!("Resolved map: {} -> {}", map_name, stripped);
                stripped
            }
            None => {
                #[cfg(feature = "debug")]
                print_debug!("Could not resolve map: {}", map_name);
                map_name.to_string()
            }
        }
    }
}
