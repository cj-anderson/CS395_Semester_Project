# Overview
The following is a breakdown of what each of the classes in this project do, and how they work together.

# armor.rs
This file defines the "Armor" struct, providing a framework by which objects of type Armor can be created (data members include a name, the Armor's level, and its defense modifier.) This file also includes the upgrade logic for objects of type Armor, including checking the amount of money the player has and the level of the equipment, increasing the stats of the Armor, and renaming it to show that it has improved. This upgrade function either provides an error (if the Armor has already reached max level, for instance, or if the player doesn't have enough money), or returns the upgraded Armor.

# enemy.rs
This file defines the "Enemy" struct, and provides a framework for the various enemies of the game to be created, interacted with, and stored. The class contains the default values for an Enemy object (default enemy is named "Emacs"), as well as a way to define Enemies with a name or pull data from a .json file. Another provided function is the "calculate_cr" system, which takes the stats of the Enemy and calculates a Challenge Rating for them, making sure the player doesn't face Enemies that are beyond their capability to handle. This class implements the methods from the Entity trait, which are potentially functionally unused in the newer versions of this project.

# entity.rs
This trait provides a base set of "getters" for Entities (Originally planned to include Players and Enemies, later phased out.) Plans to include turn action logic in this file were later scrapped.

# error.rs
This file provides a basic framework for Errors in this project, and was taken from the example code provided for this course by Professor Kennedy.

# game.rs
This file includes the main structs used by the program. Code in this file includes logic to: 
- Load from a player's savefile, named "player_data.json",
- Load from a backup save, named "save_file.json",
- Start from scratch, overwriting the data from the previous player's save.
- Initialize the game loop,
- Handle player choices within the game loop and move forward accordingly,
  - Allow the player to visit a shop to upgrade equipment,
     - Call methods from the Player struct to handle upgrades and prevent ownership issues,
  - Allow the player to rest and regain HP and uses of a consumable "healing potion",
  - Allow the player to continue further into the game,
     - Create random encounters based on the player's level,
     - Populate the encounter with enemies the player should theoretically be able to handle,
     - Manage the combat loop and have random choices for each individual enemy,
     - Manage experience gain, leveling up, and gold gain from encounters,
  - Allow the player to view their stats and those of their equipment, and
  - Allow the player to save their data and exit the program.

# lib.rs
This file includes a list of all classes, structs, and traits used in this program.

# main.rs
This class initiates the game by calling functions from game.rs.

# player.rs
This file contains logic to make a new Player object, as well as logic to handle updates, leveling up, upgrades for equipment, displaying stats for the Player, healing the Player, and recharging the Player's uses of the healing potion. This file also contains logic to save and load data to and from a .json file, and store/create a Player object as a result.

# shield.rs
This file defines the "Shield" struct, providing a framework by which objects of type Shield can be created (data members include a name, the Shield's level, and its defense modifier.) This file also includes the upgrade logic for objects of type Shield, including checking the amount of money the player has and the level of the equipment, increasing the stats of the Shield, and renaming it to show that it has improved. This upgrade function either provides an error (if the Shield has already reached max level, for instance, or if the player doesn't have enough money), or returns the upgraded Shield.

# shop.rs
This file contains logic to calculate the cost of upgrades, as well as logic to pass calls from the game driver to the Player to upgrade their equipment.

# upgrade_error.rs
This file contains logic to provide specific details on errors related to upgrade functions within the various equipment structs.

# weapon.rs
This file defines the "Weapon" struct, providing a framework by which objects of type Weapon can be created (data members include a name, the Weapon's level, and its attack and hit rate modifiers.) This file also includes the upgrade logic for objects of type Weapon, including checking the amount of money the player has and the level of the equipment, increasing the stats of the Weapon, and renaming it to show that it has improved. This upgrade function either provides an error (if the Weapon has already reached max level, for instance, or if the player doesn't have enough money), or returns the upgraded Weapon.
