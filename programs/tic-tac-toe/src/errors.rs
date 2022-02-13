use anchor_lang::prelude::*;

#[error]
pub enum TicTacToeError {
  TileOutOfBounds,
  TileAlreadySet,
  GameAlreadyOver,
  NotPlayersTurn,
}
