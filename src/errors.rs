use thiserror::Error;
#[derive(Debug, Error)]
pub enum GameErr {
    #[error("Illigal move.")]
    IllegalMove,
}

pub type GameResult<T> = std::result::Result<T, GameErr>;