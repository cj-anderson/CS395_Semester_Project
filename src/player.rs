use crate::{armor::Armor, shield::Shield, upgrade_error::UpgradeError, weapon::Weapon};

use serde::{Serialize, Deserialize};
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub level: u32,
    pub exp: u32,
    pub exp_to_lv_up: u32,
    pub max_hp: u32,
    pub hp: u32,
    pub atk: u32,
    pub def: u32,
    pub gold: u32,

    pub weapon: Weapon,
    pub shield: Shield,
    pub armor: Armor,
}

impl Player {
    pub fn new(nme: &str) -> Self {
        Player {
            name: String::from(nme),
            level: 1,
            exp: 0,
            exp_to_lv_up: 10,
            max_hp: 20,
            hp: 20,
            atk: 5,
            def: 5,
            gold: 100, // Starting gold for upgrades
            weapon: Weapon::new("Iron Sword", 10, 1),
            shield: Shield::new("Wooden Shield", 3),
            armor: Armor::new("Cloth Armor", 5),
        }
    }

    pub fn level_up(&mut self) {
        if self.exp >= self.exp_to_lv_up {
            self.exp -= self.exp_to_lv_up; // Carry over excess XP
            self.level += 1;

            // Increase stats on level up
            self.max_hp += 5;
            self.hp = self.max_hp;
            self.atk += 2;
            self.def += 2;

            // Increase XP required for next level (rounded down)
            self.exp_to_lv_up = (self.exp_to_lv_up as f32 * 1.5).floor() as u32;

            println!(
                "Congratulations! You reached Level {}. Next level at {} XP.",
                self.level, self.exp_to_lv_up
            );
        }
    }

    pub fn upgrade_weapon(&mut self, upgrade_cost: u32) -> Result<(), UpgradeError> {
        match Weapon::upgrade(self.weapon.clone(), self, upgrade_cost) {
            Ok(new_weapon) => {
                self.weapon = new_weapon; // Store the upgraded weapon
                Ok(())
            }
            Err(e) => Err(e), // Pass the error back
        }
    }

    pub fn upgrade_armor(&mut self, upgrade_cost: u32) -> Result<(), UpgradeError> {
        match Armor::upgrade(self.armor.clone(), self, upgrade_cost) {
            Ok(new_armor) => {
                self.armor = new_armor; // Store the upgraded armor
                Ok(())
            }
            Err(e) => Err(e), // Pass the error back
        }
    }

    pub fn upgrade_shield(&mut self, upgrade_cost: u32) -> Result<(), UpgradeError> {
        match Shield::upgrade(self.shield.clone(), self, upgrade_cost) {
            Ok(new_shield) => {
                self.shield = new_shield; // Store the upgraded shield
                Ok(())
            }
            Err(e) => Err(e), // Pass the error back
        }
    }

     /// Save the player data to a JSON file
     pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self).expect("Failed to serialize");
        let mut file = fs::File::create(filename)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Load player data from a JSON file
    pub fn load_from_file(filename: &str) -> io::Result<Self> {
        let json = fs::read_to_string(filename)?;
        let player: Player = serde_json::from_str(&json).expect("Failed to deserialize");
        Ok(player)
    }
}
