use crate::errors::*;
use crate::state::*;
use anchor_lang::prelude::*;
use num_traits::*;

impl Game {
  pub fn is_active(&self) -> bool {
    self.state == GameState::Active
  }

  fn current_player_index(&self) -> usize {
    ((self.turn - 1) % 2) as usize
  }

  pub fn current_player(&self) -> Pubkey {
    self.players[self.current_player_index()]
  }

  pub fn play(&mut self, tile: &Tile) -> ProgramResult {
    if !self.is_active() {
      return Err(TicTacToeError::GameAlreadyOver.into());
    }
    match tile {
      tile @ Tile {
        row: 0..=2,
        column: 0..=2,
      } => match self.board[tile.row as usize][tile.column as usize] {
        Some(_) => return Err(TicTacToeError::TileAlreadySet.into()),
        None => {
          self.board[tile.row as usize][tile.column as usize] =
            Some(Sign::from_usize(self.current_player_index()).unwrap());
        }
      },
      _ => return Err(TicTacToeError::TileOutOfBounds.into()),
    }

    self.update_state();

    if let GameState::Active = self.state {
      self.turn += 1;
    }

    Ok(())
  }

  fn is_winning_trio(&self, trio: [(usize, usize); 3]) -> bool {
    let [first, second, third] = trio;
    self.board[first.0][first.1].is_some()
      && self.board[first.0][first.1] == self.board[second.0][second.1]
      && self.board[first.0][first.1] == self.board[third.0][third.1]
  }

  fn update_state(&mut self) {
    for i in 0..=2 {
      // three of the same in one row
      if self.is_winning_trio([(i, 0), (i, 1), (i, 2)]) {
        self.state = GameState::Won {
          winner: self.current_player(),
        };
        return;
      }
      // three of the same in one column
      if self.is_winning_trio([(0, i), (1, i), (2, i)]) {
        self.state = GameState::Won {
          winner: self.current_player(),
        };
        return;
      }
    }

    // three of the same in one diagonal
    if self.is_winning_trio([(0, 0), (1, 1), (2, 2)])
      || self.is_winning_trio([(0, 2), (1, 1), (2, 0)])
    {
      self.state = GameState::Won {
        winner: self.current_player(),
      };
      return;
    }

    // reaching this code means the game has not been won,
    // so if there are unfilled tiles left, it's still active
    for row in 0..=2 {
      for column in 0..=2 {
        if self.board[row][column].is_none() {
          return;
        }
      }
    }

    // game has not been won
    // game has no more free tiles
    // -> game ends in a tie
    self.state = GameState::Tie;
  }
}
