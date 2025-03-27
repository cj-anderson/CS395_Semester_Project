use crate::upgrade_error;

use crate::{weapon::Weapon, armor::Armor, shield::Shield};

#[derive(Debug, Clone)]
pub enum Equipment {
    Weapon(Weapon),
    Armor(Armor),
    Shield(Shield),
}

impl Equipment {
    pub fn name(&self) -> &String {
        match self {
            Equipment::Weapon(weapon) => &weapon.name,
            Equipment::Armor(armor) => &armor.name,
            Equipment::Shield(shield) => &shield.name,
        }
    }

    pub fn upgrade(&mut self, player_gold: &mut u32, upgrade_cost: u32) -> Result<(), upgrade_error::UpgradeError> {
        match self {
            Equipment::Weapon(weapon) => weapon.upgrade(player_gold, upgrade_cost),
            Equipment::Armor(armor) => armor.upgrade(player_gold, upgrade_cost),
            Equipment::Shield(shield) => shield.upgrade(player_gold, upgrade_cost),
        }
    }
}
