use cs395_project::{player::Player, shield::Shield, upgrade_error::UpgradeError};
use rstest::*;

#[fixture]
fn player() -> Player {
    let mut player = Player::new("test");
    player.gold = 100;
    player
}

#[fixture]
fn shield() -> Shield {
    Shield::new("Wooden Shield", 1, 5)
}

#[rstest]
fn test_shield_initialization(shield: Shield) {
    assert_eq!(shield.name, "Wooden Shield");
    assert_eq!(shield.level, 1);
    assert_eq!(shield.defense, 5);
}

#[rstest]
fn test_shield_upgrade_success(mut player: Player, shield: Shield) {
    let result = Shield::upgrade(shield, &mut player, 20);
    assert!(result.is_ok());
    let upgraded_shield = result.unwrap();
    assert_eq!(upgraded_shield.level, 2);
    assert_eq!(upgraded_shield.defense, 10);
    assert_eq!(upgraded_shield.name, "Reinforced Shield");
    assert_eq!(player.gold, 80); // Ensure gold is deducted
}

#[rstest]
fn test_shield_upgrade_max_level(mut player: Player) {
    let shield = Shield {
        name: "Impenetrable Shield".to_string(),
        level: 5,
        defense: 30,
    };
    let result = Shield::upgrade(shield.clone(), &mut player, 20);
    assert!(matches!(result, Err(UpgradeError::MaxLevelReached(_))));
}

#[rstest]
fn test_shield_upgrade_insufficient_gold(mut player: Player, shield: Shield) {
    player.gold = 10; // Not enough gold for upgrade
    let result = Shield::upgrade(shield, &mut player, 20);
    assert!(matches!(result, Err(UpgradeError::NotEnoughGold(_))));
}
