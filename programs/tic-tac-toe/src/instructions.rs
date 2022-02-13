use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SetupGame<'info> {
  #[account(init, payer = player_one)]
  pub game: Account<'info, Game>,
  #[account(mut)]
  pub player_one: Signer<'info>,
  pub system_program: Program<'info, System>,
}