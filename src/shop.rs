use crate::{prelude::Equipment, upgrade_error};

pub struct Shop;

impl Shop {
    // Upgrade the equipment and return the upgraded item
    pub fn upgrade_item(
        &self,
        equipment: &mut Equipment,  // Equipment to upgrade (Weapon, Armor, Shield)
        player_gold: &mut u32,      // Player's current gold
        upgrade_cost: u32,          // The cost of upgrading
    ) -> Result<(), upgrade_error::UpgradeError> {
        equipment.upgrade(player_gold, upgrade_cost)?;
        Ok(())
    }
}