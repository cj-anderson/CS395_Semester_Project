use crate::{player::Player, upgrade_error::UpgradeError};

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Armor {
    pub name: String,
    pub level: u32,
    pub defense: u32,
}

impl Armor {
    pub fn new(name: &str, lvl: u32, defense: u32) -> Self {
        Armor {
            name: name.to_string(),
            level: lvl,
            defense,
        }
    }

    // Upgrade logic for Armor
    pub fn upgrade(a: Armor, p: &mut Player, upgrade_cost: u32) -> Result<Armor, UpgradeError> {
        if a.level >= 5 {
            return Err(UpgradeError::MaxLevelReached(a.name.clone()));
        }

        if p.gold < upgrade_cost {
            return Err(UpgradeError::NotEnoughGold(a.name.clone()));
        }

        p.gold -= upgrade_cost;

        // Update the armor's name based on its level
        let name_parts: Vec<&str> = a.name.splitn(2, ' ').collect();
        let binding = a.name.as_str();
        let base_name = name_parts.last().unwrap_or(&binding);
        let new_prefix = match a.level + 1 {
            2 => "Reinforced",
            3 => "Hardened",
            4 => "Impervious",
            5 => "Legendary",
            _ => "",
        };

        // Create upgraded armor
        Ok(Armor {
            name: format!("{} {}", new_prefix, base_name),
            level: a.level + 1,
            defense: a.defense + 5,
        })
    }
}
