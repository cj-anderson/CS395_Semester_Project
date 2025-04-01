use crate::{armor::Armor, shield::Shield, upgrade_error::UpgradeError, weapon::Weapon};

use serde::{Serialize, Deserialize};
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub name: String,
    pub level: u32,
    pub exp: u32,
    pub exp_to_lv_up: u32,
    pub max_hp: u32,
    pub hp: u32,
    pub atk: u32,
    pub def: u32,
    pub gold: u32,

    pub weapon: Weapon,
    pub shield: Shield,
    pub armor: Armor,

    pub potion_uses: u32, // Number of uses left for the healing potion
}

impl Player {
    pub fn new(nme: &str) -> Self {
        Player {
            name: String::from(nme),
            level: 1,
            exp: 0,
            exp_to_lv_up: 10,
            max_hp: 20,
            hp: 20,
            atk: 5,
            def: 5,
            gold: 100, // Starting gold for upgrades
            weapon: Weapon::new("Iron Sword", 1, 10, 1),
            shield: Shield::new("Wooden Shield", 1, 3),
            armor: Armor::new("Cloth Armor",1,  5),
            potion_uses: 3,
        }
    }

    pub fn from_file(filename: &str) -> Self {
        if !std::path::Path::new(filename).exists() {
            eprintln!("Save file not found. Creating a new save file...");
            print!("Enter your character's name: ");
            io::stdout().flush().expect("Failed to flush stdout");
            
            let mut name = String::new();
            io::stdin().read_line(&mut name).expect("Failed to read input");
            let name = name.trim().to_string();
            
            let default_player = Player::new(&name);
            default_player.save_to_file(filename).expect("Failed to create default save file.");
            return default_player;
        }

        match fs::read_to_string(filename) {
            Ok(content) => match serde_json::from_str::<Player>(&content) {
                Ok(player) => player,
                Err(_) => Player::handle_corrupt_save(filename),
            },
            Err(_) => Player::handle_corrupt_save(filename),
        }
    }

    fn handle_corrupt_save(filename: &str) -> Self {
        eprintln!("Corrupt save file detected. Creating a new one...");
        print!("Enter your character's name: ");
        io::stdout().flush().expect("Failed to flush stdout");
        
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input");
        let name = name.trim().to_string();
        
        let default_player = Player::new(&name);
        default_player.save_to_file(filename).expect("Failed to create default save file.");
        default_player
    }

pub fn level_up(&mut self) {
    while self.exp >= self.exp_to_lv_up {
        self.exp -= self.exp_to_lv_up;
        self.level += 1;
        self.max_hp += 5;
        self.hp = self.max_hp;
        self.atk += 1;
        self.def += 1;
        self.exp_to_lv_up = (self.exp_to_lv_up as f32 * 1.5).floor() as u32;
        println!("Congratulations! You reached Level {}. Next level at {} XP.", self.level, self.exp_to_lv_up);
    }

    println!("\nPress Enter to return to the menu...");

    // Wait for the player to press Enter
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    // Clear the terminal screen after the player presses Enter
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().expect("Failed to flush stdout");
}

    pub fn upgrade_weapon(&mut self, upgrade_cost: u32) -> Result<(), UpgradeError> {
        match Weapon::upgrade(self.weapon.clone(), self, upgrade_cost) {
            Ok(new_weapon) => {
                self.weapon = new_weapon; // Store the upgraded weapon
                Ok(())
            }
            Err(e) => Err(e), // Pass the error back
        }
    }

    pub fn display_stats(&self) {
        Self::clear_screen();
        println!("\n--- Player Stats ---");
        println!("Name: {}", self.name);
        println!("Level: {}", self.level);
        println!("XP: {}/{}", self.exp, self.exp_to_lv_up);
        println!("HP: {}/{}", self.hp, self.max_hp);
        println!("ATK: {} | DEF: {}", self.atk, self.def);
        println!("Gold: {}", self.gold);
        println!("Weapon: {} (Level {}, Hit Modifier: {}, Damage: {})", self.weapon.name, self.weapon.level, self.weapon.hit_mod, self.weapon.damage);
        println!("Armor: {} (Level {}, Defense: {})", self.armor.name, self.armor.level, self.armor.defense);
        println!("Shield: {} (Level {}, Defense: {})", self.shield.name, self.shield.level, self.shield.defense);
        println!("Potion Uses: {}\n", self.potion_uses);

        // Prompt for user input to continue
        println!("\nPress Enter to return to the menu...");

        // Wait for the player to press Enter
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        // Clear the terminal screen after the player presses Enter
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        io::stdout().flush().expect("Failed to flush stdout");
    }


    pub fn clear_screen() {
        print!("{}[2J{}[1;1H", 27 as char, 27 as char);
        io::stdout().flush().unwrap();
    }

    pub fn upgrade_armor(&mut self, upgrade_cost: u32) -> Result<(), UpgradeError> {
        match Armor::upgrade(self.armor.clone(), self, upgrade_cost) {
            Ok(new_armor) => {
                self.armor = new_armor; // Store the upgraded armor
                Ok(())
            }
            Err(e) => Err(e), // Pass the error back
        }
    }

    pub fn upgrade_shield(&mut self, upgrade_cost: u32) -> Result<(), UpgradeError> {
        match Shield::upgrade(self.shield.clone(), self, upgrade_cost) {
            Ok(new_shield) => {
                self.shield = new_shield; // Store the upgraded shield
                Ok(())
            }
            Err(e) => Err(e), // Pass the error back
        }
    }

     /// Save the player data to a JSON file
     pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self).expect("Failed to serialize");
        let mut file = fs::File::create(filename)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Load player data from a JSON file
    pub fn load_from_file(filename: &str) -> io::Result<Self> {
        let json = fs::read_to_string(filename)?;
        let player: Player = serde_json::from_str(&json).expect("Failed to deserialize");
        Ok(player)
    }

    // Heal function
    pub fn heal(&mut self) {
        if self.potion_uses > 0 {
            // Calculate the heal amount: 30 + (0.01 * level)% of max HP
            let heal_percentage = 30.0 + (0.01 * self.level as f32);
            let heal_amount = (self.max_hp as f32 * heal_percentage / 100.0).floor() as u32;

            // Heal the player, ensuring they don't exceed max HP
            self.hp = (self.hp + heal_amount).min(self.max_hp);

            // Decrease the potion uses
            self.potion_uses -= 1;
            println!("You healed for {} HP using a potion!", heal_amount);
            println!("Potion uses remaining: {}", self.potion_uses);
        } else {
            println!("You have no healing potions left!");
        }
    }

    // Recharge potion (you can call this function after certain events like resting)
    pub fn recharge_potion(&mut self) {
        self.potion_uses = 3; // Reset potion uses to 3
        println!("Your healing potions have been recharged!");
    }
}
