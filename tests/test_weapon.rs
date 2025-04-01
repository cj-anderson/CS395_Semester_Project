use cs395_project::{player::Player, upgrade_error::UpgradeError, weapon::Weapon};
use rstest::*;

#[fixture]
fn player() -> Player {
    let mut player = Player::new("test");
    player.gold = 100;
    player
}

#[fixture]
fn weapon() -> Weapon {
    Weapon::new("Iron Sword", 1, 10, 5)
}

#[rstest]
fn test_weapon_initialization(weapon: Weapon) {
    assert_eq!(weapon.name, "Iron Sword");
    assert_eq!(weapon.level, 1);
    assert_eq!(weapon.damage, 10);
    assert_eq!(weapon.hit_mod, 5);
}

#[rstest]
fn test_weapon_upgrade_success(mut player: Player, weapon: Weapon) {
    let result = Weapon::upgrade(weapon, &mut player, 20);
    assert!(result.is_ok());
    let upgraded_weapon = result.unwrap();
    assert_eq!(upgraded_weapon.level, 2);
    assert_eq!(upgraded_weapon.damage, 15);
    assert_eq!(upgraded_weapon.hit_mod, 6);
    assert_eq!(upgraded_weapon.name, "Strengthened Sword");
    assert_eq!(player.gold, 80); // Ensure gold is deducted
}

#[rstest]
fn test_weapon_upgrade_max_level(mut player: Player) {
    let weapon = Weapon {
        name: "Enchanted Sword".to_string(),
        level: 5,
        damage: 30,
        hit_mod: 10,
    };
    let result = Weapon::upgrade(weapon.clone(), &mut player, 20);
    assert!(matches!(result, Err(UpgradeError::MaxLevelReached(_))));
}

#[rstest]
fn test_weapon_upgrade_insufficient_gold(mut player: Player, weapon: Weapon) {
    player.gold = 10; // Not enough gold for upgrade
    let result = Weapon::upgrade(weapon, &mut player, 20);
    assert!(matches!(result, Err(UpgradeError::NotEnoughGold(_))));
}
