use thiserror::Error;
#[derive(Debug, Error)]
#[derive(PartialEq)]
pub enum GameErr {
    #[error("Path is blocked.")]
    PathIsBlocked,
    #[error("Position is occupied.")]
    PositionOccupied,
    #[error("No piece at position.")]
    NoPieceAtPosition,
    #[error("No move registeret")]
    NoMoveRegistered,
    #[error("You can only play your own pieces.")]
    IllegalMoveOnOtherPlayer,
    #[error("Illegal pawn move.")]
    IllegalPawnMove,
    #[error("Illegal king move.")]
    IllegalKingMove,
    #[error("Illegal queen move.")]
    IllegalQueenMove,
    #[error("Illegal knight move.")]
    IllegalKnightMove,
    #[error("Illegal bishop move.")]
    IllegalBishopMove,
    #[error("Illegal rook move.")]
    IllegalRookMove,

}

pub type GameResult<T> = std::result::Result<T, GameErr>;