use thiserror::Error;

use crate::*;

#[derive(Debug, Error, PartialEq)]
pub enum UpgradeError<'a> {
    #[error("{equip_name} cannot be upgraded further!")]
    UpgradeMax { equip_name: &'a str,},
}   

#[derive(Debug, Error, PartialEq)]
pub struct ErrorWithValue<E: std::error::Error, V> {
    #[source]
    pub the_error: E,
    pub the_value: V,
}