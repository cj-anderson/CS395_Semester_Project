# CS395 Semester Project

# Requirements:

* Cargo (Rust Compiler)

# Base Dependencies:

* thiserror (v2.0.12)
* serde (v1.0, includes "derive" feature)
* serde_json (v1.0)
* rand (v0.9 or latest)

# Dev Dependencies:

* hamcrest2 (v0.3 or current) 
* stringreader (v0.1.1)
* thiserror = (v2.0.12)
* eyre (v0.6.12)
* rstest (v0.25.0)
* serde (v1.0, includes "derive" feature)
* serde_json (v1.0)
* approx (v0.5)

# Objectives:

The aim of this project is to:
* Utilise Rust (and potentially FFI with Java - optional),
* Create a text-based roguelike that runs completely through the terminal, that includes:
    * Saving and loading the game, via a .json file 
    * Saving and loading from a backup save, also stored in a .json file
    * Have two (or more - optional) player "classes" for different playstyles 
        * (Current Version has one, may scrap second class idea if implementation becomes difficult)
    * Implement basic equipment, such as:
        * Weapons (Each has a name, level, hit modifier, and damage value.)
        * Armor (Each has a name, level, and defense value.)
        * Shield (Each has a name, level, and defense value.)
    * Upgrading player equipment, up to a maximum of 4 times (Max Level: 5)
    * Implement a basic consumable (Healing Potion)
        * (Potential Upgrade Idea: Add Shop Functionality to increase max carrying capacity)

# Overview

This program is a text-based rogue-like that runs completely in the terminal. Currently, there is one playable class, 4 levels of upgrade for each piece of equipment, and a carrying capacity of 3 healing potions.
