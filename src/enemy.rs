use crate::entity::Entity;

#[derive(Debug)]

/// Define a PLayer.
pub struct Enemy {
    pub name: String,
    pub max_hp: u32,
    pub hp: u32,
    pub atk: u32,
    pub def: u32,
    pub exp_given: u32,
}

impl Enemy {
    ///Create a default player.
    pub fn new() -> Self {
        Enemy {
            name: String::from("emacs"),
            max_hp: 10u32,
            hp: 10u32,
            atk: 5u32,
            def: 5u32,
            exp_given: 10u32,
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
        }
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
