use crate::{
    api_types,
    error::{Error, Result},
};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell(pub i8, pub i8);

impl From<&api_types::Coordinates> for Cell {
    fn from(value: &api_types::Coordinates) -> Self {
        Self(value.x as i8, value.y as i8)
    }
}

#[derive(Debug)]
pub struct Battlesnake {
    cells: Vec<Cell>,
}

impl From<&api_types::Battlesnake> for Battlesnake {
    fn from(value: &api_types::Battlesnake) -> Self {
        let mut cells = Vec::with_capacity(value.length as usize);
        cells.push((&value.head).into());
        cells.extend(
            value
                .body
                .iter()
                .map(<&api_types::Coordinates as Into<Cell>>::into),
        );
        Self { cells }
    }
}

impl Battlesnake {
    pub fn new_dead() -> Self {
        Battlesnake { cells: vec![] }
    }

    pub fn update(&self, snake_move: Move, food: &[Cell]) -> Battlesnake {
        let mut cells = Vec::with_capacity(self.cells.len());
        if let Some(&head) = self.cells.first() {
            let new_head = head + snake_move;
            cells.push(new_head);
            cells.extend_from_slice(
                &self.cells[0..self.cells.len() - if food.contains(&new_head) { 0 } else { 1 }],
            );
        }
        Self { cells }
    }

    pub fn head(&self) -> Option<Cell> {
        self.cells.first().copied()
    }

    pub fn body(&self) -> &[Cell] {
        if self.cells.is_empty() {
            &self.cells
        } else {
            &self.cells[1..]
        }
    }

    pub fn is_alive(&self) -> bool {
        !self.cells.is_empty()
    }

    pub fn has_gone_oob(&self, board_width: i32, board_height: i32) -> bool {
        if let Some(head) = self.head() {
            let x = head.0 as i32;
            let y = head.1 as i32;
            x >= board_width || x < 0 || y >= board_height || y < 0
        } else {
            false
        }
    }
}

#[cfg(test)]
impl Battlesnake {
    pub fn new(cells: Vec<(usize, usize)>) -> Self {
        let cells = cells
            .into_iter()
            .map(|(x, y)| Cell(x as i8, y as i8))
            .collect();
        Self { cells }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    pub fn enumerate() -> MovesEnumerator {
        MovesEnumerator {
            next: Some(Move::Up),
        }
    }
}

pub struct MovesEnumerator {
    next: Option<Move>,
}

impl Iterator for MovesEnumerator {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.next;
        self.next = match self.next {
            None => None,
            Some(Move::Up) => Some(Move::Down),
            Some(Move::Down) => Some(Move::Left),
            Some(Move::Left) => Some(Move::Right),
            Some(Move::Right) => None,
        };
        result
    }
}

impl std::ops::Add<Move> for Cell {
    type Output = Cell;

    fn add(self, rhs: Move) -> Self::Output {
        match rhs {
            Move::Up => Cell(self.0, self.1 + 1),
            Move::Down => Cell(self.0, self.1 - 1),
            Move::Left => Cell(self.0 - 1, self.1),
            Move::Right => Cell(self.0 + 1, self.1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_move_to_cell() {
        assert_eq!(Cell(0, 0) + Move::Up, Cell(0, 1));
        assert_eq!(Cell(0, 0) + Move::Down, Cell(0, -1));
        assert_eq!(Cell(0, 0) + Move::Left, Cell(-1, 0));
        assert_eq!(Cell(0, 0) + Move::Right, Cell(1, 0));
        assert_eq!(Cell(3, 7) + Move::Up, Cell(3, 8));
        assert_eq!(Cell(3, 7) + Move::Down, Cell(3, 6));
        assert_eq!(Cell(3, 7) + Move::Left, Cell(2, 7));
        assert_eq!(Cell(3, 7) + Move::Right, Cell(4, 7));
    }

    #[test]
    fn moves_enumerator() {
        let result: Vec<_> = Move::enumerate().collect();
        assert_eq!(result.len(), 4);
        assert!(result.contains(&Move::Up));
        assert!(result.contains(&Move::Down));
        assert!(result.contains(&Move::Left));
        assert!(result.contains(&Move::Right));
    }

    #[test]
    fn update_battlesnake() {
        let target = Battlesnake {
            cells: vec![Cell(2, 3), Cell(2, 4), Cell(2, 5), Cell(1, 5), Cell(0, 5)],
        };
        assert_eq!(
            target.update(Move::Up, &vec![]).cells,
            vec![Cell(2, 4), Cell(2, 3), Cell(2, 4), Cell(2, 5), Cell(1, 5)]
        );
        assert_eq!(
            target.update(Move::Down, &vec![]).cells,
            vec![Cell(2, 2), Cell(2, 3), Cell(2, 4), Cell(2, 5), Cell(1, 5)]
        );
        assert_eq!(
            target.update(Move::Left, &vec![]).cells,
            vec![Cell(1, 3), Cell(2, 3), Cell(2, 4), Cell(2, 5), Cell(1, 5)]
        );
        assert_eq!(
            target.update(Move::Right, &vec![]).cells,
            vec![Cell(3, 3), Cell(2, 3), Cell(2, 4), Cell(2, 5), Cell(1, 5)]
        );

        let target = Battlesnake {
            cells: vec![Cell(5, 5), Cell(4, 5)],
        };
        assert_eq!(
            target.update(Move::Up, &vec![]).cells,
            vec![Cell(5, 6), Cell(5, 5)]
        );
        assert_eq!(
            target.update(Move::Left, &vec![]).cells,
            vec![Cell(4, 5), Cell(5, 5)]
        );

        let target = Battlesnake {
            cells: vec![Cell(7, 0)],
        };
        assert_eq!(target.update(Move::Up, &vec![]).cells, vec![Cell(7, 1)]);
        assert_eq!(target.update(Move::Left, &vec![]).cells, vec![Cell(6, 0)]);

        let target = Battlesnake { cells: vec![] };
        assert_eq!(target.update(Move::Up, &vec![]).cells, vec![]);
        assert_eq!(target.update(Move::Left, &vec![]).cells, vec![]);
    }

    #[test]
    fn battlesnake_has_gone_oob() {
        let target = Battlesnake {
            cells: vec![Cell(7, 3), Cell(6, 3), Cell(6, 2)],
        };
        assert!(!target.has_gone_oob(10, 10));
        assert!(!target.has_gone_oob(8, 4));
        assert!(target.has_gone_oob(7, 4));
        assert!(target.has_gone_oob(8, 3));
        assert!(target.has_gone_oob(8, 1));
        assert!(target.has_gone_oob(5, 1));
    }
}
