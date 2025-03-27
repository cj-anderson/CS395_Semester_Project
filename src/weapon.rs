use crate::equipment::Equipment;

#[derive(Debug)]

pub struct Weapon {
    pub name: String,
    pub level: u32,
    pub damage: u32,
    pub hit_mod: u32,
}

impl Weapon {

}

impl Equipment for Weapon {
    fn name(&self) -> &String {
        &self.name
    }

    fn upgrade(&self) -> Result<Weapon, Error> {

    }
}