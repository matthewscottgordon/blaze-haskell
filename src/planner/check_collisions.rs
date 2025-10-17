use crate::game_state::{Battlesnake, Cell, GameState};

pub fn check_collisions(game_state: GameState) -> GameState {
    let mut cells = CellGrid::new(game_state.width as usize, game_state.height as usize);

    for &cell in game_state.player.body() {
        cells[cell] = CellContents::PlayerBody;
    }
    for snake in &game_state.enemies {
        for &cell in snake.body() {
            cells[cell] = CellContents::EnemySnakeBody;
        }
    }

    let mut player = if let Some(cell) = game_state.player.head() {
        match cells[cell] {
            CellContents::Empty => game_state.player,
            CellContents::PlayerBody => Battlesnake::new_dead(),
            CellContents::EnemySnakeBody => Battlesnake::new_dead(),
        }
    } else {
        game_state.player
    };
    let mut enemies: Vec<Battlesnake> = game_state
        .enemies
        .into_iter()
        .filter_map(|snake| {
            if let Some(cell) = snake.head() {
                match cells[cell] {
                    CellContents::Empty => Some(snake),
                    CellContents::PlayerBody => None,
                    CellContents::EnemySnakeBody => None,
                }
            } else {
                Some(snake)
            }
        })
        .collect();

    let num_enemies = enemies.len();
    for i in 0..num_enemies {
        if enemies[i].is_alive() && enemies[i].head() == player.head() {
            if enemies[i].length() >= player.length() {
                player = Battlesnake::new_dead()
            }
            if enemies[i].length() <= player.length() {
                enemies[i] = Battlesnake::new_dead()
            }
        }
        for j in i + 1..num_enemies {
            if enemies[j].is_alive() && enemies[i].head() == enemies[j].head() {
                if enemies[i].length() >= enemies[j].length() {
                    enemies[j] = Battlesnake::new_dead()
                }
                if enemies[i].length() <= enemies[j].length() {
                    enemies[i] = Battlesnake::new_dead()
                }
            }
        }
    }

    GameState {
        player,
        enemies,
        ..game_state
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum CellContents {
    #[default]
    Empty,
    PlayerBody,
    EnemySnakeBody,
}

struct CellGrid<T> {
    width: usize,
    data: Vec<T>,
}

impl<T: Default + Clone> CellGrid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let data = vec![T::default(); width * height];
        Self { data, width }
    }
}

impl<T> std::ops::Index<Cell> for CellGrid<T> {
    type Output = T;

    fn index(&self, Cell(x, y): Cell) -> &Self::Output {
        assert!(x >= 0);
        assert!(y >= 0);
        assert!((x as usize) < self.width);
        &self.data[y as usize * self.width + x as usize]
    }
}

impl<T> std::ops::IndexMut<Cell> for CellGrid<T> {
    fn index_mut(&mut self, Cell(x, y): Cell) -> &mut Self::Output {
        assert!(x >= 0);
        assert!(y >= 0);
        assert!((x as usize) < self.width);
        &mut self.data[y as usize * self.width + x as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_grid_read_empty() {
        let target: CellGrid<CellContents> = CellGrid::new(13, 9);
        for x in 0..13 {
            for y in 0..9 {
                assert_eq!(target[Cell(x, y)], CellContents::Empty);
            }
        }
    }

    #[test]
    fn cell_grid_oob_panic() {
        let target: CellGrid<CellContents> = CellGrid::new(13, 9);
        let result = std::panic::catch_unwind(|| target[Cell(13, 2)]);
        assert!(result.is_err());
        let result = std::panic::catch_unwind(|| target[Cell(2, 9)]);
        assert!(result.is_err());
        let result = std::panic::catch_unwind(|| target[Cell(-1, 3)]);
        assert!(result.is_err());
        let result = std::panic::catch_unwind(|| target[Cell(2, -1)]);
        assert!(result.is_err());
    }

    #[test]
    fn cell_grid_write_read() {
        let mut target: CellGrid<(i8, i8)> = CellGrid::new(7, 9);
        for x in 0..7 {
            for y in 0..9 {
                target[Cell(x, y)] = (x, y);
            }
        }
        for x in 0..7 {
            for y in 0..9 {
                assert_eq!(target[Cell(x, y)], (x, y));
            }
        }
    }

    #[test]
    fn test_player_collision_detected() {
        let gamestate = GameState {
            height: 11,
            width: 11,
            player: Battlesnake::new(&[(3, 3), (3, 2), (3, 1), (3, 0)]),
            enemies: vec![
                Battlesnake::new(&[(7, 2), (7, 3), (8, 3)]),
                Battlesnake::new(&[(1, 3), (2, 3), (3, 3), (4, 3), (5, 3)]),
            ],
            food: vec![],
        };
        let new_gamestate = check_collisions(gamestate);
        assert!(!new_gamestate.player.is_alive());
        assert!(new_gamestate.enemies[0].is_alive());
        assert!(new_gamestate.enemies[1].is_alive());
        let gamestate = GameState {
            height: 11,
            width: 11,
            player: Battlesnake::new(&[(3, 3), (3, 2), (3, 1), (3, 0)]),
            enemies: vec![
                Battlesnake::new(&[(1, 3), (2, 3), (3, 3), (4, 3), (5, 3)]),
                Battlesnake::new(&[(7, 2), (7, 3), (8, 3)]),
            ],
            food: vec![],
        };
        let new_gamestate = check_collisions(gamestate);
        assert!(!new_gamestate.player.is_alive());
        assert!(new_gamestate.enemies[0].is_alive());
        assert!(new_gamestate.enemies[1].is_alive());
    }
}
