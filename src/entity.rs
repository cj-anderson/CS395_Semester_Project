//define the basic traits of all entities
pub trait Entity {
    //return the name of the entity as a fixed string
    fn name(&self) -> &String;

    //return stats of the entity
    fn max_hp(&self) -> u32;
    fn hp(&self) -> u32;
    fn atk(&self) -> u32;
    fn def(&self) -> u32;

    //take actions on turn
    //to be added
}
