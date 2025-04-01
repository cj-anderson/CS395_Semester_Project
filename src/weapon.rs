use crate::{player::Player, upgrade_error::UpgradeError};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weapon {
    pub name: String,
    pub level: u32,
    pub damage: u32,
    pub hit_mod: u32,
}

impl Weapon {
    pub fn new(name: &str, _lvl: u32, damage: u32, hit_mod: u32) -> Self {
        Weapon {
            name: name.to_string(),
            level: 1,
            damage,
            hit_mod,
        }
    }

    // Upgrade logic for Weapon
    pub fn upgrade(w: Weapon, p: &mut Player, upgrade_cost: u32) -> Result<Weapon, UpgradeError> {
        if w.level >= 5 {
            return Err(UpgradeError::MaxLevelReached(w.name.clone()));
        }

        if p.gold < upgrade_cost {
            return Err(UpgradeError::NotEnoughGold(p.name.clone()));
        }

        p.gold -= upgrade_cost;

        // Extract base name safely
        let (prefix, base_name) = w.name.split_once(' ').unwrap_or(("", &w.name));

        let new_prefix = match w.level + 1 {
            2 => "Strengthened",
            3 => "Tempered",
            4 => "Enchanted",
            5 => "Legendary",
            _ => prefix, // Keep existing prefix if beyond level 5
        };

        // Create upgraded weapon
        Ok(Weapon {
            name: format!("{} {}", new_prefix, base_name),
            level: w.level + 1,
            damage: w.damage + 5,
            hit_mod: w.hit_mod + 1,
        })
    }
}
