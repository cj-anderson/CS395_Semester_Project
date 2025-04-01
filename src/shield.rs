use crate::{player::Player, upgrade_error::UpgradeError};

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shield {
    pub name: String,
    pub level: u32,
    pub defense: u32,
}

impl Shield {
    pub fn new(name: &str, lvl: u32, defense: u32) -> Self {
        Shield {
            name: name.to_string(),
            level: lvl,
            defense,
        }
    }

    // Upgrade logic for Shield
    pub fn upgrade(s: Shield, p: &mut Player, upgrade_cost: u32) -> Result<Shield, UpgradeError> {
        if s.level >= 5 {
            return Err(UpgradeError::MaxLevelReached(s.name.clone()));
        }

        if p.gold < upgrade_cost {
            return Err(UpgradeError::NotEnoughGold(s.name.clone()));
        }

        p.gold -= upgrade_cost;

        // Update the shield's name based on its level
        let name_parts: Vec<&str> = s.name.splitn(2, ' ').collect();
        let binding = s.name.as_str();
        let base_name = name_parts.last().unwrap_or(&binding);
        let new_prefix = match s.level + 1 {
            2 => "Reinforced",
            3 => "Fortified",
            4 => "Impenetrable",
            5 => "Legendary",
            _ => "",
        };

        // Create upgraded shield
        Ok(Shield {
            name: format!("{} {}", new_prefix, base_name),
            level: s.level + 1,
            defense: s.defense + 5,
        })
    }
}
