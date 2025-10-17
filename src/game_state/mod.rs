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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_state_from_board() {
        let board_json = r##"{
  "height": 11,
  "width": 12,
  "food": [
    {"x": 5, "y": 5},
    {"x": 9, "y": 0},
    {"x": 2, "y": 6}
  ],
  "hazards": [
    {"x": 0, "y": 0},
    {"x": 0, "y": 1},
    {"x": 0, "y": 2}
  ],
  "snakes": [
    {
      "id": "snake-one",
      "name": "Snake One",
      "health": 54,
      "body": [
        {"x": 1, "y": 0},
        {"x": 1, "y": 0},
        {"x": 2, "y": 0}
      ],
      "latency": "123",
      "head": {"x": 1, "y": 1},
      "length": 3,
      "shout": "why are we shouting??",
      "squad": "1",
      "customizations":{
        "color":"#26CF04",
        "head":"smile",
        "tail":"bolt"
      }
    },
    {
      "id": "snake-two",
      "name": "Snake Two",
      "health": 54,
      "body": [
        {"x": 1, "y": 2},
        {"x": 1, "y": 1},
        {"x": 1, "y": 0}
      ],
      "latency": "123",
      "head": {"x": 2, "y": 2},
      "length": 3,
      "shout": "why are we shouting??",
      "squad": "1",
      "customizations":{
        "color":"#26CF04",
        "head":"smile",
        "tail":"bolt"
      }
    },
    {
      "id": "snake-one",
      "name": "Snake 3",
      "health": 54,
      "body": [
        {"x": 3, "y": 4},
        {"x": 3, "y": 4},
        {"x": 2, "y": 4}
      ],
      "latency": "123",
      "head": {"x": 3, "y": 3},
      "length": 3,
      "shout": "why are we shouting??",
      "squad": "1",
      "customizations":{
        "color":"#26CF04",
        "head":"smile",
        "tail":"bolt"
      }
    }
  ]
}"##;
        let board: api_types::Board = serde_json::from_str(board_json).unwrap();
        let target = GameState::from_board(&board, "snake-two").unwrap();
        assert_eq!(target.height, 11);
        assert_eq!(target.width, 12);
        assert_eq!(target.player.head(), Some(Cell(2, 2)));
        assert_eq!(target.enemies.len(), 2);
        assert!(target.enemies.iter().any(|e| e.head() == Some(Cell(1, 1))));
        assert_eq!(target.enemies.len(), 2);
        assert!(target.enemies.iter().any(|e| e.head() == Some(Cell(3, 3))));
        assert_eq!(target.food.len(), 3);
    }
}
