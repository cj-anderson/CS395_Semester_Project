use crate::{armor::Armor, shield::Shield, upgrade_error::UpgradeError, weapon::Weapon};

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub level: u32,
    pub exp: u32,
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
}
