use rand::seq::IndexedRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::io::{self, Write};
use std::process::exit;
use crate::player::Player;
use crate::enemy::Enemy;


#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub player: Player,
    pub enemies: Vec<Enemy>,
}

impl GameState {
    // Check if the player save exists
    pub fn player_save_exists(filename: &str) -> bool {
        fs::metadata(filename).is_ok() // Returns true if the file exists
    }

    pub fn create_new_player() -> Player {
        let mut player_name = String::new();
    
        print!("Enter your character's name: ");
        io::stdout().flush().unwrap(); // Flush to ensure prompt appears
    
        io::stdin().read_line(&mut player_name).unwrap();
        let player_name = player_name.trim().to_string(); // Remove trailing newline
    
        if player_name.is_empty() {
            println!("No name entered. Defaulting to 'Hero'.");
            Player::new("Hero")
        } else {
            Player::new(&player_name)
        }
    }

    // Load game state from the provided filename
    pub fn load(filename: &str) -> Option<Self> {
        match fs::read_to_string(filename) {
            Ok(file_content) => {
                match serde_json::from_str::<GameState>(&file_content) {
                    Ok(game_state) => Some(game_state),
                    Err(err) => {
                        eprintln!("Failed to deserialize game state: {}", err);
                        None
                    }
                }
            }
            Err(err) => {
                eprintln!("Failed to read file '{}': {}", filename, err);
                None
            }
        }
    }

    // Save game state to the specified filename
    pub fn save(&self) {
        let serialized = match serde_json::to_string(self) {
            Ok(serialized) => serialized,
            Err(err) => {
                eprintln!("Failed to serialize game state: {}", err);
                return;
            }
        };

        match fs::write("save_file.json", serialized) {
            Ok(_) => println!("Game state saved!"),
            Err(err) => eprintln!("Failed to save game state: {}", err),
        }
    }

    
    pub fn prompt_for_game_start() -> Option<Self> {
        println!("Welcome to the Game!");
        println!("Please choose an option:");
        println!("1. Load from save file (player_data.json)");
        println!("2. Load from backup (save_file.json)");
        println!("3. Start fresh");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                if let Ok(file_content) = fs::read_to_string("player_data.json") {
                    match serde_json::from_str::<Player>(&file_content) {
                        Ok(player) => {
                            let enemies = Enemy::load_enemies_from_file("enemies.json").unwrap_or_else(|_| {
                                println!("Failed to load enemies, creating new ones.");
                                Vec::new()
                            });
                            return Some(GameState { player, enemies });
                        }
                        Err(_) => println!("Error: Save file is corrupted. Starting a fresh game."),
                    }
                } else {
                    println!("Error: No save file found. Starting a fresh game.");
                }
            }
            "2" => {
                return Self::load("save_file.json");
            }
            "3" => {
                println!("Starting a new game.");
            }
            _ => {
                println!("Invalid choice. Starting a fresh game.");
            }
        }

        // If we reach here, start fresh
        let player = Self::create_new_player();
        let enemies = Enemy::load_enemies_from_file("enemies.json").unwrap_or_else(|_| {
            println!("Failed to load enemies, creating new ones.");
            Vec::new()
        });
        let game_state = GameState { player, enemies };
        game_state.save(); // Save the new game state
        Some(game_state)
    }

    // Function to check and create a default save file if not present or corrupt
    pub fn check_or_create_default_save(&self) {
        if let None = GameState::load("save_file.json") {
            println!("Creating a default save file...");
            let default_state = GameState {
                // Initialize with default values or prompt for user input
                player: Player::new("default hero"),
                enemies: self.enemies.clone(), // or fetch this from the user's input
            };
            default_state.save();
            println!("Default save file created successfully!");
        }
    }
}

pub fn game_driver() {
    Player::clear_screen();

    // Load or start a new game based on user input
    let game_state = GameState::prompt_for_game_start().unwrap_or_else(|| {
        // This will only be called if the user chooses to start fresh or if no valid state is loaded.
        println!("Something went wrong. Starting a fresh game...");
        let player = create_new_player();  // This creates a new player
        let enemies = Enemy::load_enemies_from_file("assets/enemies.json").unwrap_or_else(|_| {
            println!("Failed to load enemies, creating new ones.");
            Vec::new() // Default empty enemies if fail to load
        });
        let game_state = GameState { player, enemies };
        game_state.save();  // Save the new game state
        game_state
    });

    let mut player = game_state.player;
    let mut enemies = game_state.enemies;

    loop {
        Player::clear_screen();
        println!("\nWhat would you like to do?");
        println!("1. Visit Shop");
        println!("2. Rest");
        println!("3. Move to Next Room");
        println!("4. View Stats");
        println!("5. Save and Quit");

        let mut choice = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap();

        match choice {
            1 => visit_shop(&mut player),
            2 => rest(&mut player),
            3 => move_to_next_room(&mut player, &mut enemies),
            4 => player.display_stats(),
            5 => {
                let game_state = GameState { player: player.clone(), enemies };
                game_state.save(); // Save game state
                let _ = player.save_to_file("player_data.json"); // Save player data
                println!("Game saved. Quitting...");
                break;
            },
            _ => println!("Invalid choice, try again."),
        }
    }
}

fn create_new_player() -> Player {
    println!("Enter your player name:");
    let mut player_name = String::new();
    io::stdin().read_line(&mut player_name).unwrap();
    let player_name = player_name.trim().to_string();
    
    Player::new(&player_name)
}

fn visit_shop(player: &mut Player) {
    Player::clear_screen();
    println!("Welcome to the shop!");
    
    loop {
        // Show current equipment and its potential upgrades
        println!("\nCurrent Equipment:");
        println!("Weapon: {} (Damage: {}, Hit Modifier: {}, Level: {})", 
                 player.weapon.name, player.weapon.damage, player.weapon.hit_mod, player.weapon.level);
        println!("Shield: {} (Defense: {}, Level: {})", 
                 player.shield.name, player.shield.defense, player.shield.level);
        println!("Armor: {} (Defense: {}, Level: {})", 
                 player.armor.name, player.armor.defense, player.armor.level);

        // Display gold amounts
        println!("\nGold: {}", player.gold);
        // Display the available upgrades and their costs
        println!("\nAvailable Upgrades:");

        let weapon_upgrade_cost = 35 + 15 * player.weapon.level;
        let shield_upgrade_cost = 25 + 15 * player.shield.level;
        let armor_upgrade_cost = 30 + 15 * player.armor.level;

        if player.weapon.level < 5 {
            println!("1. Upgrade Weapon (Cost: {} gold)", weapon_upgrade_cost);
        } else {
            println!("1. Weapon is already at max level!");
        }

        if player.shield.level < 5 {
            println!("2. Upgrade Shield (Cost: {} gold)", shield_upgrade_cost);
        } else {
            println!("2. Shield is already at max level!");
        }

        if player.armor.level < 5 {
            println!("3. Upgrade Armor (Cost: {} gold)", armor_upgrade_cost);
        } else {
            println!("3. Armor is already at max level!");
        }

        println!("4. Leave the shop");

        // Get player choice
        let mut choice = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap();

        match choice {
            1 => {
                Player::clear_screen();
                if player.weapon.level < 5 {
                    if player.gold >= weapon_upgrade_cost{
                        let _ = player.upgrade_weapon(weapon_upgrade_cost);  // Call the upgrade method on the weapon
                        println!("Your weapon has been upgraded!");
                    } else {
                        println!("You don't have enough gold to upgrade the weapon.");
                    }
                } else {
                    println!("Your weapon is already at max level!");
                }
            }
            2 => {
                Player::clear_screen();
                if player.shield.level < 5 {
                    if player.gold >= shield_upgrade_cost {
                        let _ = player.upgrade_shield(shield_upgrade_cost);  // Call the upgrade method on the shield
                        println!("Your shield has been upgraded!");
                    } else {
                        println!("You don't have enough gold to upgrade the shield.");
                    }
                } else {
                    println!("Your shield is already at max level!");
                }
            }
            3 => {
                Player::clear_screen();
                if player.armor.level < 5 {
                    if player.gold >= armor_upgrade_cost {
                        let _ = player.upgrade_armor(armor_upgrade_cost);  // Call the upgrade method on the armor
                        println!("Your armor has been upgraded!");
                    } else {
                        println!("You don't have enough gold to upgrade the armor.");
                    }
                } else {
                    println!("Your armor is already at max level!");
                }
            }
            4 => break,
            _ => println!("Invalid choice, try again."),
        }
    }
}

fn rest(player: &mut Player) {
    // Let the player rest and recover some health
    Player::clear_screen();
    player.hp = player.max_hp;
    println!("You rest and regain HP.");
    player.recharge_potion();

     // Prompt for user input to continue
     println!("\nPress Enter to return to the menu...");

     // Wait for the player to press Enter
     let mut input = String::new();
     io::stdin().read_line(&mut input).expect("Failed to read line");
     // Clear the terminal screen after the player presses Enter
     print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
     io::stdout().flush().expect("Failed to flush stdout");
}

pub fn move_to_next_room(player: &mut Player, enemies: &mut Vec<Enemy>) {
    let cap_cr = 2.5 * player.level as f32;
    let filtered_enemies: Vec<&Enemy> = enemies.iter()
        .filter(|&enemy| enemy.cr <= cap_cr)
        .collect();

    if filtered_enemies.is_empty() {
        println!("No suitable enemies found.");
        return;
    }

    let mut rng = rand::rng();
    let num_enemies = rng.random_range(1..=3);

    let mut selected_enemies = Vec::new();
    for _ in 0..num_enemies {
        if let Some(enemy) = filtered_enemies.choose(&mut rng) {
            selected_enemies.push((*enemy).clone()); 
        }
    }

    combat(player, &mut selected_enemies);
}

pub fn combat(player: &mut Player, enemies: &mut Vec<Enemy>) {
    println!("You encountered:");
    for enemy in enemies.iter() {
        println!("{} (HP: {}/{})", enemy.name, enemy.hp, enemy.max_hp);
    }

    let mut enemy_defending: bool = false; 
    while player.hp > 0 && enemies.iter().any(|e| e.hp > 0) {
        println!("\nYour HP: {}/{}", player.hp, player.max_hp);
        println!("Potions left: {}", player.potion_uses);
        println!("Choose an action:");
        println!("1. Attack");
        println!("2. Defend");
        println!("3. Heal");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = match choice.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Try again.");
                continue;
            }
        };

        let mut player_defending = false;
        
        match choice {
            1 => {
                Player::clear_screen();
                println!("Select an enemy to attack:");
                for (i, enemy) in enemies.iter().enumerate() {
                    if enemy.hp > 0 {
                        println!("{}. {} (HP: {}/{})", i + 1, enemy.name, enemy.hp, enemy.max_hp);
                    }
                }

                let mut enemy_choice = String::new();
                io::stdin().read_line(&mut enemy_choice).unwrap();
                let enemy_index = match enemy_choice.trim().parse::<usize>() {
                    Ok(num) if num > 0 && num <= enemies.len() => num - 1,
                    _ => {
                        println!("Invalid choice.");
                        continue;
                    }
                };

                if let Some(enemy) = enemies.get_mut(enemy_index) {
                    if enemy.hp > 0 {
                        let damage = player.atk + player.weapon.damage - enemy.def;
                        let final_damage = if enemy_defending { damage / 2 } else { damage }.max(1);
                        enemy.hp = enemy.hp.saturating_sub(final_damage);
                        println!("You attacked {} for {} damage!", enemy.name, final_damage);
                        if enemy.hp == 0 {
                            println!("{} has been defeated!", enemy.name);
                            player.exp += enemy.exp_given;
                            player.gold += enemy.exp_given;
                        }
                    }
                }
            }
            2 => {
                player_defending = true;
                println!("You brace yourself for incoming attacks!");
            }
            3 => {
                if player.potion_uses > 0 {
                    let heal_amount = player.max_hp / 2;
                    player.hp = (player.hp + heal_amount).min(player.max_hp);
                    player.potion_uses -= 1;
                    println!("You drank a potion and restored {} HP!", heal_amount);
                } else {
                    println!("You're out of potions!");
                }
            }
            _ => println!("Invalid choice!"),
        }

        enemies.retain(|e| e.hp > 0);

        for enemy in enemies.iter_mut() {
            if enemy.hp > 0 {
                let mut rng = rand::rng();
                match rng.random_range(0..=1) {
                    0 => {
                        let raw_damage = enemy.atk.saturating_sub(if player_defending { player.def + player.shield.defense } else { player.def });
                        let final_damage = raw_damage.max(1);
                        player.hp = player.hp.saturating_sub(final_damage);
                        println!("{} attacked you for {} damage!", enemy.name, final_damage);
                    }
                    1 => {
                        enemy_defending = true;
                        println!("{} braces for defense!", enemy.name);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    if player.hp == 0 {
        println!("You have been defeated... Game Over.");
        exit(0);
    } else {
        println!("You defeated the enemies!");
        player.level_up();
    }
}