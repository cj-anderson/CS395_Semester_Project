use crate::entity::Entity;

#[derive(Debug)]

/// Define a PLayer.
pub struct Player {
    pub name: String,
    pub level: u32,
    pub exp: u32,
    pub max_hp: u32,
    pub hp: u32,
    pub atk: u32,
    pub def: u32,
    pub mag_atk: u32,
    pub mag_def: u32,

    //Equipment for a Player. (to be implemented.)
    // pub weapon: Weapon;
    // pub shield: Shield;
    // pub armor: Armor;
    // pub items: Inventory;
    // pub money: u32;
}

impl Player {
    ///Create a default player.
    pub fn new() -> Self {
        Player {
            name: String::from("John Doe"),
            level: 1u32,
            exp: 0u32,
            max_hp: 20u32,
            hp: 20u32,
            atk: 5u32,
            def: 5u32,
            mag_atk: 5u32,
            mag_def: 5u32,
        }
    }

    pub fn with_name(n: &str) -> Self {
        Player {
            name: n.to_string(),
            level: 1u32,
            exp: 0u32,
            max_hp: 20u32,
            hp: 20u32,
            atk: 5u32,
            def: 5u32,
            mag_atk: 5u32,
            mag_def: 5u32,
        }
    }
}

impl Entity for Player { 
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

    fn atk_mag(&self) -> u32 {
        self.mag_atk
    }

    fn def_mag(&self) -> u32 {
        self.mag_def
    }
}