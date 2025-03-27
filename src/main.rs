use cs395_project::{player::Player, shop::Shop};

fn main() {
    // Create a new player
    let mut player = Player::new();
    let shop = Shop;

    println!("Player's starting gold: {}", player.gold);

    // Attempt to upgrade the weapon
    let upgrade_cost = 20;  // Define a cost for upgrading the weapon
    if let Err(e) = player.upgrade_weapon(&shop, upgrade_cost) {
        println!("Upgrade failed: {:?}", e);
    } else {
        println!("Weapon upgraded! Remaining gold: {}", player.gold);
    }

    // Attempt to upgrade the armor
    let upgrade_cost = 15;  // Define a cost for upgrading the armor
    if let Err(e) = player.upgrade_armor(&shop, upgrade_cost) {
        println!("Upgrade failed: {:?}", e);
    } else {
        println!("Armor upgraded! Remaining gold: {}", player.gold);
    }

    // Attempt to upgrade the shield
    let upgrade_cost = 10;  // Define a cost for upgrading the shield
    if let Err(e) = player.upgrade_shield(&shop, upgrade_cost) {
        println!("Upgrade failed: {:?}", e);
    } else {
        println!("Shield upgraded! Remaining gold: {}", player.gold);
    }

    // Print updated player status
    println!("Player's final gold: {}", player.gold);
    // Print the updated equipment names and stats
    println!("Weapon: {}", player.weapon.name());
    println!("Armor: {}", player.armor.name());
    println!("Shield: {}", player.shield.name());
}
