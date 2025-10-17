use crate::game_state::Cell;

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
}
