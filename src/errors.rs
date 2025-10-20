use thiserror::Error;
#[derive(Debug, Error)]
pub enum GameErr {
    #[error("Illegal move. {0}")]
    IllegalMove(String),
}

pub type GameResult<T> = std::result::Result<T, GameErr>;