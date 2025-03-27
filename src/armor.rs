use crate::{prelude::Equipment, upgrade_error::{self, UpgradeError}};

#[derive(Debug, Clone)]
pub struct Armor {
    pub name: String,
    pub level: u32,
    pub defense: u32,
}

impl Armor {
    pub fn new(name: &str, defense: u32) -> Self {
        Armor {
            name: name.to_string(),
            level: 1,
            defense,
        }
    }

    pub fn upgrade(&mut self, player_gold: &mut u32, upgrade_cost: u32) -> Result<(), upgrade_error::UpgradeError> {
        if self.level >= 5 {
            return Err(UpgradeError::MaxLevelReached(self.name.clone()));
        }

        if *player_gold < upgrade_cost {
            return Err(UpgradeError::NotEnoughGold(self.name.clone()));
        }

        // Deduct gold from player
        *player_gold -= upgrade_cost;

        // Split the name at the first space to get base name and prefix
        let name_parts: Vec<&str> = self.name.splitn(2, ' ').collect();
        let binding = self.name.as_str();
        let base_name = name_parts.last().unwrap_or(&binding);

        // Generate new prefix based on level
        let new_prefix = match self.level + 1 {
            2 => "Reinforced",
            3 => "Enhanced",
            4 => "Gilded",
            5 => "Enchanted",
            _ => "",
        };

        // Create a new Armor with upgraded stats
        let upgraded_armor = Armor {
            name: format!("{} {}", new_prefix, base_name),
            level: self.level + 1,
            defense: self.defense + 3, // Increase defense per upgrade
        };

        // Return the upgraded armor wrapped in a Box<dyn Equipment>
        Ok(())
    }
}