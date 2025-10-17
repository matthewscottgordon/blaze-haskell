use crate::{
    api_types,
    error::{Error, Result},
};

mod battlesnake;
pub use battlesnake::Battlesnake;
mod movement;
pub use movement::Move;
mod cell;
pub use cell::Cell;

#[derive(Debug)]
pub struct GameState {
    pub height: u16,
    pub width: u16,
    pub player: Battlesnake,
    pub enemies: Vec<Battlesnake>,
    pub food: Vec<Cell>,
}

impl GameState {
    pub fn from_board(board: &api_types::Board, player_id: &str) -> Result<Self> {
        let height = board.height as u16;
        let width = board.width as u16;
        let player = board
            .snakes
            .iter()
            .find_map(|snake| {
                if snake.id == player_id {
                    Some(snake.into())
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                Error::new(format!(
                    "Could not find player snake \"{}\" on board.",
                    player_id
                ))
            })?;
        let enemies = board
            .snakes
            .iter()
            .filter_map(|snake| {
                if snake.id != player_id {
                    Some(snake.into())
                } else {
                    None
                }
            })
            .collect();
        let food = board.food.iter().map(|coord| coord.into()).collect();
        Ok(GameState {
            height,
            width,
            player,
            enemies,
            food,
        })
    }
}
