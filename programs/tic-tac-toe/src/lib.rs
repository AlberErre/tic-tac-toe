mod errors;
mod game;
mod instructions;
mod state;

use crate::errors::*;
use crate::instructions::*;
use crate::state::*;
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod tic_tac_toe {
  use super::*;
  pub fn setup_game(ctx: Context<SetupGame>, player_two: Pubkey) -> ProgramResult {
    let game = &mut ctx.accounts.game;
    game.players = [ctx.accounts.player_one.key(), player_two];
    game.turn = 1;
    Ok(())
  }

  pub fn play(ctx: Context<Play>, tile: Tile) -> ProgramResult {
    let game = &mut ctx.accounts.game;

    require!(
      game.current_player() == ctx.accounts.player.key(),
      TicTacToeError::NotPlayersTurn
    );

    game.play(&tile)
  }
}
