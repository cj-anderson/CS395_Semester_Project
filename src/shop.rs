use crate::{
    armor::Armor, player::Player, shield::Shield, upgrade_error::UpgradeError, weapon::Weapon,
};

pub struct Shop;

impl Shop {
    pub fn get_wpn_upgrade_cost(weapon: &Weapon) -> u32 {
        // Example scaling cost formula: base cost * level multiplier
        20 + (weapon.level * 10)
    }

    pub fn get_amr_upgrade_cost(armor: &Armor) -> u32 {
        20 + (armor.level * 10)
    }

    pub fn get_shld_upgrade_cost(shield: &Shield) -> u32 {
        20 + (shield.level * 10)
    }

    pub fn upgrade_player_weapon(&self, player: &mut Player) -> Result<(), UpgradeError> {
        let cost = Self::get_wpn_upgrade_cost(&player.weapon); // Calculate upgrade cost
        player.upgrade_weapon(cost) // Pass the dynamic cost
    }

    pub fn upgrade_player_armor(&self, player: &mut Player) -> Result<(), UpgradeError> {
        let cost = Self::get_amr_upgrade_cost(&player.armor); // Calculate upgrade cost
        player.upgrade_armor(cost) // Pass the dynamic cost
    }

    pub fn upgrade_player_shield(&self, player: &mut Player) -> Result<(), UpgradeError> {
        let cost = Self::get_shld_upgrade_cost(&player.shield); // Calculate upgrade cost
        player.upgrade_shield(cost) // Pass the dynamic cost
    }
}
