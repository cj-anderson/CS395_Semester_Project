use crate::{shop::Shop, upgrade_error, armor::Armor, equipment::Equipment, shield::Shield, weapon::Weapon};

pub struct Player {
    pub name: String,
    pub level: u32,
    pub exp: u32,
    pub max_hp: u32,
    pub hp: u32,
    pub atk: u32,
    pub def: u32,
    pub gold: u32,

    // Equipment for the player, as an enum to handle all equipment types
    pub weapon: Equipment,
    pub shield: Equipment,
    pub armor: Equipment,
}

impl Player {
    pub fn new() -> Self {
        Player {
            name: String::from("John Doe"),
            level: 1u32,
            exp: 0u32,
            max_hp: 20u32,
            hp: 20u32,
            atk: 5u32,
            def: 5u32,
            gold: 100u32, // Starting gold
            weapon: Equipment::Weapon(Weapon::new("Iron Sword", 10, 1)),
            shield: Equipment::Shield(Shield::new("Wooden Shield", 3)),
            armor: Equipment::Armor(Armor::new("Leather Armor", 5)),
        }
    }

    pub fn upgrade_weapon(&mut self, shop: &Shop, upgrade_cost: u32) -> Result<(), upgrade_error::UpgradeError> {
        let mut player_gold = self.gold;
        shop.upgrade_item(&mut self.weapon, &mut player_gold, upgrade_cost)?;

        // Update the player's gold after the upgrade
        self.gold = player_gold;

        Ok(())
    }

    pub fn upgrade_armor(&mut self, shop: &Shop, upgrade_cost: u32) -> Result<(), upgrade_error::UpgradeError> {
        let mut player_gold = self.gold;
        shop.upgrade_item(&mut self.armor, &mut player_gold, upgrade_cost)?;

        // Update the player's gold after the upgrade
        self.gold = player_gold;

        Ok(())
    }

    pub fn upgrade_shield(&mut self, shop: &Shop, upgrade_cost: u32) -> Result<(), upgrade_error::UpgradeError> {
        let mut player_gold = self.gold;
        shop.upgrade_item(&mut self.shield, &mut player_gold, upgrade_cost)?;

        // Update the player's gold after the upgrade
        self.gold = player_gold;

        Ok(())
    }
}
