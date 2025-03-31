use cs395_project::{player::Player, shop::Shop};

fn main() {
    // Create a new player with a custom name
    let mut player = Player::new("Arthur");

    // Create a shop instance
    let shop = Shop;

    player.gold= 999999;

    // Display initial player and weapon stats
    println!("Player: {}", player.name);
    println!("Gold: {}", player.gold);
    println!("Before upgrade: {:?}", player.weapon);

    // Get the upgrade cost
    let cost = Shop::get_wpn_upgrade_cost(&player.weapon);
    println!("Upgrade cost: {}", cost);

    // Attempt to upgrade the weapon
    match shop.upgrade_player_weapon(&mut player) {
        Ok(_) => {
            println!("Upgrade successful!");
            println!("After first upgrade: {:?}", player.weapon);
            println!("Gold remaining: {}", player.gold);
        }
        Err(e) => println!("Upgrade failed: {:?}", e),
    }

    // Test another upgrade to see if the scaling works
    let cost = Shop::get_wpn_upgrade_cost(&player.weapon);
    println!("\nNext upgrade cost: {}", cost);

    match shop.upgrade_player_weapon(&mut player) {
        Ok(_) => {
            println!("Upgrade successful again!");
            println!("After second upgrade: {:?}", player.weapon);
            println!("Gold remaining: {}", player.gold);
        }
        Err(e) => println!("Upgrade failed: {:?}", e),
    }

    // Test another upgrade to see if the scaling works
    let cost = Shop::get_wpn_upgrade_cost(&player.weapon);
    println!("\nNext upgrade cost: {}", cost);

    match shop.upgrade_player_weapon(&mut player) {
        Ok(_) => {
            println!("Upgrade successful again!");
            println!("After third upgrade: {:?}", player.weapon);
            println!("Gold remaining: {}", player.gold);
        }
        Err(e) => println!("Upgrade failed: {:?}", e),
    }

    // Test another upgrade to see if the scaling works
    let cost = Shop::get_wpn_upgrade_cost(&player.weapon);
    println!("\nNext upgrade cost: {}", cost);

    match shop.upgrade_player_weapon(&mut player) {
        Ok(_) => {
            println!("Upgrade successful again!");
            println!("After fourth upgrade: {:?}", player.weapon);
            println!("Gold remaining: {}", player.gold);
        }
        Err(e) => println!("Upgrade failed: {:?}", e),
    }

    // Test another upgrade to see if the scaling works
    let cost = Shop::get_shld_upgrade_cost(&player.shield);
    println!("\nNext upgrade cost: {}", cost);

    match shop.upgrade_player_shield(&mut player) {
        Ok(_) => {
            println!("Upgrade successful!");
            println!("After first upgrade: {:?}", player.shield);
            println!("Gold remaining: {}", player.gold);
        }
        Err(e) => println!("Upgrade failed: {:?}", e),
    }

    // Test another upgrade to see if the scaling works
    let cost = Shop::get_shld_upgrade_cost(&player.shield);
    println!("\nNext upgrade cost: {}", cost);

    match shop.upgrade_player_shield(&mut player) {
        Ok(_) => {
            println!("Upgrade successful again!");
            println!("After second upgrade: {:?}", player.shield);
            println!("Gold remaining: {}", player.gold);
        }
        Err(e) => println!("Upgrade failed: {:?}", e),
    }

    // Test another upgrade to see if the scaling works
    let cost = Shop::get_shld_upgrade_cost(&player.shield);
    println!("\nNext upgrade cost: {}", cost);

    match shop.upgrade_player_shield(&mut player) {
        Ok(_) => {
            println!("Upgrade successful again!");
            println!("After third upgrade: {:?}", player.shield);
            println!("Gold remaining: {}", player.gold);
        }
        Err(e) => println!("Upgrade failed: {:?}", e),
    }

    // Test another upgrade to see if the scaling works
    let cost = Shop::get_shld_upgrade_cost(&player.shield);
    println!("\nNext upgrade cost: {}", cost);

    match shop.upgrade_player_shield(&mut player) {
        Ok(_) => {
            println!("Upgrade successful again!");
            println!("After fourth upgrade: {:?}", player.shield);
            println!("Gold remaining: {}", player.gold);
        }
        Err(e) => println!("Upgrade failed: {:?}", e),
    }

    // Test another upgrade to see if the scaling works
    let cost = Shop::get_amr_upgrade_cost(&player.armor);
    println!("\nNext Armor upgrade cost: {}", cost);

    match shop.upgrade_player_armor(&mut player) {
        Ok(_) => {
            println!("Upgrade successful!");
            println!("After first upgrade: {:?}", player.armor);
            println!("Gold remaining: {}", player.gold);
        }
        Err(e) => println!("Upgrade failed: {:?}", e),
    }

    // Test another upgrade to see if the scaling works
    let cost = Shop::get_amr_upgrade_cost(&player.armor);
    println!("\nNext Armor upgrade cost: {}", cost);

    match shop.upgrade_player_armor(&mut player) {
        Ok(_) => {
            println!("Upgrade successful again!");
            println!("After second upgrade: {:?}", player.weapon);
            println!("Gold remaining: {}", player.gold);
        }
        Err(e) => println!("Upgrade failed: {:?}", e),
    }

    // Test another upgrade to see if the scaling works
    let cost = Shop::get_amr_upgrade_cost(&player.armor);
    println!("\nNext Armor upgrade cost: {}", cost);

    match shop.upgrade_player_armor(&mut player) {
        Ok(_) => {
            println!("Upgrade successful again!");
            println!("After third upgrade: {:?}", player.armor);
            println!("Gold remaining: {}", player.gold);
        }
        Err(e) => println!("Upgrade failed: {:?}", e),
    }

    // Test another upgrade to see if the scaling works
    let cost = Shop::get_amr_upgrade_cost(&player.armor);
    println!("\nNext Armor upgrade cost: {}", cost);

    match shop.upgrade_player_armor(&mut player) {
        Ok(_) => {
            println!("Upgrade successful again!");
            println!("After fourth upgrade: {:?}", player.armor);
            println!("Gold remaining: {}", player.gold);
        }
        Err(e) => println!("Upgrade failed: {:?}", e),
    }
}
