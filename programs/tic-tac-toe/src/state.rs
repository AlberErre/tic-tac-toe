use anchor_lang::prelude::*;
use num_derive::*;

#[account]
#[derive(Default)]
pub struct Game {
  pub players: [Pubkey; 2],          // 64
  pub turn: u8,                      // 1
  pub board: [[Option<Sign>; 3]; 3], // 9 * (1 + 1) = 18
  pub state: GameState,              // 32 + 1
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
  Active,
  Tie,
  Won { winner: Pubkey },
}

impl Default for GameState {
  fn default() -> Self {
    Self::Active
  }
}

#[derive(
  AnchorSerialize, AnchorDeserialize, FromPrimitive, ToPrimitive, Copy, Clone, PartialEq, Eq,
)]
pub enum Sign {
  X,
  O,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Tile {
  pub row: u8,
  pub column: u8,
}
