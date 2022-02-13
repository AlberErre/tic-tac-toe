use anchor_lang::prelude::*;
use num_derive::*;

#[account]
#[derive(Default)]
pub struct Game {
  players: [Pubkey; 2],          // 64
  turn: u8,                      // 1
  board: [[Option<Sign>; 3]; 3], // 9 * (1 + 1) = 18
  state: GameState,              // 32 + 1
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
