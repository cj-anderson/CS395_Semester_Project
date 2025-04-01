use cs395_project::{
    armor::Armor, player::Player, shield::Shield, shop::Shop, upgrade_error::UpgradeError,
    weapon::Weapon,
};
use rstest::*;

#[fixture]
fn player() -> Player {
    let mut player = Player::new("test");

    player.weapon = Weapon {
        level: 1,
        hit_mod: 5,
        damage: 10,
        name: "Sword".to_string(),
    };
    player.armor = Armor {
        level: 1,
        name: "Leather Armor".to_string(),
        defense: 5,
    };
    player.shield = Shield {
        level: 1,
        name: "Wooden Shield".to_string(),
        defense: 3,
    };
    player.gold = 100;

    player
}

#[fixture]
fn shop() -> Shop {
    Shop
}

#[rstest]
fn test_weapon_upgrade_cost() {
    let weapon = Weapon {
        level: 3,
        hit_mod: 7,
        damage: 15,
        name: "Greatsword".to_string(),
    };
    assert_eq!(Shop::get_wpn_upgrade_cost(&weapon), 50);
}

#[rstest]
fn test_armor_upgrade_cost() {
    let armor = Armor {
        level: 2,
        name: "Chainmail".to_string(),
        defense: 8,
    };
    assert_eq!(Shop::get_amr_upgrade_cost(&armor), 40);
}

#[rstest]
fn test_shield_upgrade_cost() {
    let shield = Shield {
        level: 4,
        name: "Iron Shield".to_string(),
        defense: 10,
    };
    assert_eq!(Shop::get_shld_upgrade_cost(&shield), 60);
}

#[rstest]
fn test_upgrade_player_weapon(mut player: Player, shop: Shop) {
    player.gold = 50; // Set enough gold to upgrade
    let result = shop.upgrade_player_weapon(&mut player);
    assert!(result.is_ok());
}

#[rstest]
fn test_upgrade_player_armor(mut player: Player, shop: Shop) {
    player.gold = 50; // Set enough gold to upgrade
    let result = shop.upgrade_player_armor(&mut player);
    assert!(result.is_ok());
}

#[rstest]
fn test_upgrade_player_shield(mut player: Player, shop: Shop) {
    player.gold = 50; // Set enough gold to upgrade
    let result = shop.upgrade_player_shield(&mut player);
    assert!(result.is_ok());
}

#[rstest]
fn test_insufficient_gold_for_weapon_upgrade(mut player: Player, shop: Shop) {
    player.gold = 10; // Not enough gold to upgrade
    let result = shop.upgrade_player_weapon(&mut player);
    assert!(matches!(result, Err(UpgradeError::NotEnoughGold(_))));
}
