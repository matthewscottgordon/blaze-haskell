use crate::{
    api_types,
    game_state::{Cell, Move},
};

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
    pub fn new(cells: Vec<(usize, usize)>) -> Self {
        let cells = cells
            .into_iter()
            .map(|(x, y)| Cell(x as i8, y as i8))
            .collect();
        Self { cells }
    }
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

    pub fn length(&self) -> usize {
        self.cells.len()
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
mod tests {
    use super::*;

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
