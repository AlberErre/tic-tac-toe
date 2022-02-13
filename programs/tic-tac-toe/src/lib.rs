mod instructions;
mod state;

use crate::instructions::*;
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod tic_tac_toe {
  use super::*;
  pub fn setup_game(ctx: Context<SetupGame>, player_two: Pubkey) -> ProgramResult {
    Ok(())
  }
}
