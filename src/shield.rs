use crate::{prelude::Equipment, upgrade_error::{self, UpgradeError}};

#[derive(Debug, Clone)]
pub struct Shield {
    pub name: String,
    pub level: u32,
    pub defense: u32,
}

impl Shield {
    pub fn new(name: &str, defense: u32) -> Self {
        Shield {
            name: name.to_string(),
            level: 1,
            defense,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
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

        // Bind the result of self.name.as_str() to a variable to prevent unused warning
        let base_name = self.name.splitn(2, ' ').last().unwrap_or(self.name.as_str()); // Get base name after split

        // Generate new prefix based on level
        let new_prefix = match self.level + 1 {
            2 => "Restored",
            3 => "Tempered",
            4 => "Reinforced",
            5 => "Hero's",
            _ => "",
        };

        // Create a new Shield with upgraded stats
        let upgraded_shield = Shield {
            name: format!("{} {}", new_prefix, base_name), // Combine new prefix with base name
            level: self.level + 1,
            defense: self.defense + 2, // Increase defense per upgrade
        };

        // Return the upgraded shield wrapped in a Box<dyn Equipment>
        Ok(())
    }
}