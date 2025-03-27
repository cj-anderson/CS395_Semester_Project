use thiserror::Error;

#[derive(Debug, Error)]
pub enum UpgradeError {
    #[error("Not enough gold to upgrade {0}!")]
    NotEnoughGold(String),

    #[error("{0} has already reached max level!")]
    MaxLevelReached(String),
}