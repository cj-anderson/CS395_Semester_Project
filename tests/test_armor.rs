use cs395_project::{armor::Armor, player::Player, upgrade_error::UpgradeError};
use rstest::*;

#[fixture]
fn player() -> Player {
    let mut player = Player::new("test");
    player.gold = 100;
    player
}

#[fixture]
fn armor() -> Armor {
    Armor::new("Leather Armor", 1, 5)
}

#[rstest]
fn test_armor_initialization(armor: Armor) {
    assert_eq!(armor.name, "Leather Armor");
    assert_eq!(armor.level, 1);
    assert_eq!(armor.defense, 5);
}

#[rstest]
fn test_armor_upgrade_success(mut player: Player, armor: Armor) {
    let result = Armor::upgrade(armor, &mut player, 20);
    assert!(result.is_ok());
    let upgraded_armor = result.unwrap();
    assert_eq!(upgraded_armor.level, 2);
    assert_eq!(upgraded_armor.defense, 10);
    assert_eq!(upgraded_armor.name, "Reinforced Armor");
    assert_eq!(player.gold, 80); // Ensure gold is deducted
}

#[rstest]
fn test_armor_upgrade_max_level(mut player: Player) {
    let armor = Armor {
        name: "Impervious Armor".to_string(),
        level: 5,
        defense: 30,
    };
    let result = Armor::upgrade(armor.clone(), &mut player, 20);
    assert!(matches!(result, Err(UpgradeError::MaxLevelReached(_))));
}

#[rstest]
fn test_armor_upgrade_insufficient_gold(mut player: Player, armor: Armor) {
    player.gold = 10; // Not enough gold for upgrade
    let result = Armor::upgrade(armor, &mut player, 20);
    assert!(matches!(result, Err(UpgradeError::NotEnoughGold(_))));
}
