use approx::assert_relative_eq;
use cs395_project::enemy::Enemy;
use rstest::rstest;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write; // Import for file operations

    // Helper function to create a temporary file with enemy data
    fn create_test_enemy_file(filename: &str) {
        let enemies_data = r#"
        [
            {
                "name": "Goblin",
                "max_hp": 30,
                "hp": 30,
                "atk": 5,
                "def": 2,
                "exp_given": 10,
                "cr": 0.0
            },
            {
                "name": "Orc",
                "max_hp": 50,
                "hp": 50, 
                "atk": 10,
                "def": 5,
                "exp_given": 20,
                "cr": 0.0
            }
        ]
        "#;

        let mut file = fs::File::create(filename).expect("Unable to create file");
        file.write_all(enemies_data.as_bytes())
            .expect("Unable to write data to file");
    }

    // Test that checks if we can load and validate enemy data
    #[rstest]
    fn test_load_enemies_from_file() {
        // File name for test
        let filename = "test_enemies.json";

        // Create test file
        create_test_enemy_file(filename);

        // Load the enemies from the file
        let mut enemies =
            Enemy::load_enemies_from_file(filename).expect("Failed to load enemies from file");

        for e in &mut enemies {
            e.calculate_cr(); // Now you can mutate `e`
        }

        // Check the length of the enemies vector
        assert_eq!(enemies.len(), 2, "Expected 2 enemies");

        // Check the data for each enemy
        let goblin = &enemies[0];
        assert_eq!(goblin.name, "Goblin");
        assert_eq!(goblin.max_hp, 30);
        assert_eq!(goblin.atk, 5);
        assert_eq!(goblin.def, 2);
        assert_eq!(goblin.exp_given, 10);

        // Check calculated CR (should be 1.2 based on formula)
        assert_relative_eq!(goblin.cr, 1.2, epsilon = 0.01);

        let orc = &enemies[1];
        assert_eq!(orc.name, "Orc");
        assert_eq!(orc.max_hp, 50);
        assert_eq!(orc.atk, 10);
        assert_eq!(orc.def, 5);
        assert_eq!(orc.exp_given, 20);

        // Check calculated CR (should be 2.4 based on formula)
        assert_relative_eq!(orc.cr, 2.25, epsilon = 0.01);

        // Clean up the test file
        fs::remove_file(filename).expect("Failed to remove test file");
    }
}
