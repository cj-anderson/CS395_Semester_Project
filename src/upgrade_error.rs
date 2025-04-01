use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum UpgradeError {
    #[error("Not enough gold to upgrade {0}!")]
    NotEnoughGold(String),

    #[error("{0} has already reached max level!")]
    MaxLevelReached(String),

    #[error("Unknown Equipment Type!")]
    InvalidUpgrade,
}
