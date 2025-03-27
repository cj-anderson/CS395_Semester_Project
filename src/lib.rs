#[cfg(test)]

extern crate hamcrest2;
extern crate thiserror;

pub mod entity;
pub mod player;
pub mod enemy;

pub mod equipment;
pub mod weapon;
pub mod shield;
pub mod armor;

pub mod error;

pub mod prelude{
    pub use crate::entity::Entity;
    pub use crate::equipment::Equipment;
}