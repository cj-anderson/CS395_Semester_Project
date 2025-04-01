use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::entity::Entity;

#[derive(Debug, Serialize, Deserialize, Clone)]

// Define a Enemy.
pub struct Enemy {
    pub name: String,
    pub max_hp: u32,
    pub hp: u32,
    pub atk: u32,
    pub def: u32,
    pub exp_given: u32,
    pub cr: f32,
}

impl Enemy {
    // Create a default enemy.
    pub fn new() -> Self {
        Enemy {
            name: String::from("emacs"),
            max_hp: 10u32,
            hp: 10u32,
            atk: 5u32,
            def: 5u32,
            exp_given: 10u32,
            cr: 0.5,
        }
    }

    pub fn with_name(n: &str) -> Self {
        Enemy {
            name: n.to_string(),
            max_hp: 10u32,
            hp: 10u32,
            atk: 5u32,
            def: 5u32,
            exp_given: 10u32,
            cr: 0.5,
        }
    }

    pub fn load_enemies_from_file(filename: &str) -> Result<Vec<Enemy>, serde_json::Error> {
        // Read the content of the file

        // Replace with your actual filename
        let possible_paths = [format!("./{}", filename), format!("../{}", filename)];

        for path in &possible_paths {
            if Path::new(path).exists() {
                let file_content = fs::read_to_string(path).expect("Failed to read file");

                match serde_json::from_str::<Vec<Enemy>>(&file_content) {
                    Ok(enemies) => return Ok(enemies),
                    Err(err) => panic!("Failed to deserialize JSON: {:?}", err),
                }
            }
        }
        let file_content = fs::read_to_string(filename).expect("Failed to read file");

        // Deserialize the file content into a Vec<Enemy> (a list of enemies)
        let mut enemies: Vec<Enemy> = serde_json::from_str(&file_content)?;

        // Calculate CR for each enemy in the vector
        for enemy in &mut enemies {
            enemy.calculate_cr();
        }

        Ok(enemies)
    }

    pub fn calculate_cr(&mut self) {
        // CR calculation based on stats
        let base_cr =
            (self.max_hp as f32 * 0.02) + (self.atk as f32 * 0.1) + (self.def as f32 * 0.05);
        self.cr = base_cr;
    }
}

impl Entity for Enemy {
    fn name(&self) -> &String {
        &self.name
    }

    fn max_hp(&self) -> u32 {
        self.max_hp
    }

    fn hp(&self) -> u32 {
        self.hp
    }

    fn atk(&self) -> u32 {
        self.atk
    }

    fn def(&self) -> u32 {
        self.def
    }
}
