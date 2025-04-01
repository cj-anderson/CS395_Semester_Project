use cs395_project::{player::Player, upgrade_error::UpgradeError};

use std::fs;

#[cfg(test)]
use rstest::*;

#[fixture]
fn a_player() -> Player {
    Player::new("Tom")
}

#[rstest]
fn test_player_creation(a_player: Player) {
    assert_eq!(a_player.name, "Tom");
    assert_eq!(a_player.level, 1);
    assert_eq!(a_player.exp, 0);
    assert_eq!(a_player.max_hp, 20);
    assert_eq!(a_player.hp, 20);
    assert_eq!(a_player.atk, 5);
    assert_eq!(a_player.def, 5);
    assert_eq!(a_player.gold, 100);
    assert_eq!(a_player.weapon.name, "Iron Sword");
    assert_eq!(a_player.shield.name, "Wooden Shield");
    assert_eq!(a_player.armor.name, "Cloth Armor");
}

#[rstest]
fn test_level_up() {
    let mut player = Player::new("TestHero");

    // Simulate gaining enough XP for two levels
    player.exp = 10; // Level up once
    player.level_up();
    assert_eq!(player.level, 2);
    assert_eq!(player.exp, 0);
    assert_eq!(player.exp_to_lv_up, 15); // 10 * 1.5 = 15

    player.exp = 15; // Level up again
    player.level_up();
    assert_eq!(player.level, 3);
    assert_eq!(player.exp, 0);
    assert_eq!(player.exp_to_lv_up, 22); // 15 * 1.5 = 22 (rounded down)

    // Verify stat increases
    assert_eq!(player.max_hp, 30); // +5 per level-up
    assert_eq!(player.atk, 7); // +2 per level-up
    assert_eq!(player.def, 7); // +2 per level-up
}

#[rstest]
fn test_weapon_upgrade_success() {
    let mut player = Player::new("Hero");
    let initial_atk = player.weapon.damage;

    assert!(player.upgrade_weapon(50).is_ok()); // Assuming upgrade succeeds
    assert!(player.weapon.damage > initial_atk); // Weapon should be stronger
    assert!(player.gold < 100); // Gold should be deducted
}

#[rstest]
fn test_weapon_upgrade_fail_insufficient_gold() {
    let mut player = Player::new("Hero");
    let upgrade_result = player.upgrade_weapon(200); // More than starting gold

    assert!(matches!(
        upgrade_result,
        Err(UpgradeError::NotEnoughGold(_))
    ));
    assert_eq!(player.weapon.name, "Iron Sword"); // No upgrade should occur
    assert_eq!(player.gold, 100); // Gold should not be deducted
}

#[rstest]
fn test_armor_upgrade_success() {
    let mut player = Player::new("Hero");
    let initial_def = player.armor.defense;

    assert!(player.upgrade_armor(40).is_ok()); // Assuming upgrade succeeds
    assert!(player.armor.defense > initial_def); // Defense should increase
    assert!(player.gold < 100); // Gold should be deducted
}

#[rstest]
fn test_armor_upgrade_fail_insufficient_gold() {
    let mut player = Player::new("Hero");
    let upgrade_result = player.upgrade_armor(200); // More than starting gold

    assert!(matches!(
        upgrade_result,
        Err(UpgradeError::NotEnoughGold(_))
    ));
    assert_eq!(player.armor.name, "Cloth Armor"); // No upgrade should occur
    assert_eq!(player.gold, 100); // Gold should not be deducted
}

#[rstest]
fn test_shield_upgrade_success() {
    let mut player = Player::new("Hero");
    let initial_shield_def = player.shield.defense;

    assert!(player.upgrade_shield(30).is_ok()); // Assuming upgrade succeeds
    assert!(player.shield.defense > initial_shield_def); // Shield should improve
    assert!(player.gold < 100); // Gold should be deducted
}

#[rstest]
fn test_shield_upgrade_fail_insufficient_gold() {
    let mut player = Player::new("Hero");
    let upgrade_result = player.upgrade_shield(200); // More than starting gold

    assert!(matches!(
        upgrade_result,
        Err(UpgradeError::NotEnoughGold(_))
    ));
    assert_eq!(player.shield.name, "Wooden Shield"); // No upgrade should occur
    assert_eq!(player.gold, 100); // Gold should not be deducted
}

#[rstest]
fn test_save_and_load_player() {
    let player = Player::new("Tom");

    let test_filename = "test_player.json";

    assert!(player.save_to_file(test_filename).is_ok());

    let loaded_player = Player::load_from_file(test_filename).expect("Failed to load player");

    assert_eq!(player.name, loaded_player.name);
    assert_eq!(player.level, loaded_player.level);

    fs::remove_file(test_filename).expect("Failed to delete test file");
}
#[rstest]
fn test_heal() {
    // Create a player with initial stats
    let mut player = Player::new("Hero");

    // Save initial HP
    let initial_hp = player.hp;

    // Set a fixed level for testing
    player.level = 10;

    // Use the healing potion
    player.heal();

    // Calculate the expected heal amount
    let heal_percentage = 30.0 + (0.01 * player.level as f32);
    let expected_heal_amount = (player.max_hp as f32 * heal_percentage / 100.0).floor() as u32;
    let expected_hp = (initial_hp + expected_heal_amount).min(player.max_hp);

    // Check if the player's HP was correctly healed
    assert_eq!(
        player.hp, expected_hp,
        "Player's HP should be healed by the correct amount"
    );

    // Check if the potion use is reduced
    assert_eq!(
        player.potion_uses, 2,
        "Potion uses should decrease by 1 after using the potion"
    );
}

#[rstest]
fn test_heal_no_potion_left() {
    // Create a player with no potions left
    let mut player = Player::new("Hero");
    player.potion_uses = 0;

    // Try to heal
    player.heal();

    // HP should remain the same since there are no potions
    assert_eq!(
        player.hp, player.max_hp,
        "Player's HP should remain the same when there are no potions left"
    );

    // Ensure potion uses are still 0
    assert_eq!(
        player.potion_uses, 0,
        "Potion uses should remain 0 if there are no potions left"
    );
}

#[rstest]
fn test_recharge_potion() {
    // Create a player with some potions used
    let mut player = Player::new("Hero");
    player.potion_uses = 1; // Assume 1 potion use left

    // Recharge the potion
    player.recharge_potion();

    // Ensure the potion uses are reset to 3
    assert_eq!(
        player.potion_uses, 3,
        "Potion uses should be reset to 3 after recharge"
    );
}
