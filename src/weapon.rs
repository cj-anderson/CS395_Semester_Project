use crate::{equipment::Equipment, upgrade_error::{self, UpgradeError}};

#[derive(Debug, Clone)]

pub struct Weapon {
    pub name: String,
    pub level: u32,
    pub damage: u32,
    pub hit_mod: u32,
}

impl Weapon {
    pub fn new(name: &str, damage: u32, hit_mod: u32) -> Self{
        Weapon{
            name: name.to_string(),
            level: 1,
            damage,
            hit_mod,
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
        let base_name = name_parts.last().unwrap_or(&binding); // Get the base name after the split
    
        // Generate new prefix based on level
        let new_prefix = match self.level + 1 {
            2 => "Strengthened",
            3 => "Tempered",
            4 => "Enchanted",
            5 => "Legendary",
            _ => "", // This should never happen
        };
    
        // Create a new Weapon with the upgraded values
        let upgraded_weapon = Weapon {
            name: format!("{} {}", new_prefix, base_name), // Combine new prefix with the base name
            level: self.level + 1, // Increase the level
            damage: self.damage + 5, // Increase damage per upgrade
            hit_mod: self.hit_mod + 1, // Increase hit modifier per upgrade
        };
    
        // Return the upgraded weapon wrapped in a Box<dyn Equipment>
        Ok(())
    }
    


}

