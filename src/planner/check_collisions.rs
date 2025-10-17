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

#[derive(Clone, Copy, Default, PartialEq, Eq)]
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
        &self.data[y as usize * self.width + x as usize]
    }
}

impl<T> std::ops::IndexMut<Cell> for CellGrid<T> {
    fn index_mut(&mut self, Cell(x, y): Cell) -> &mut Self::Output {
        &mut self.data[y as usize * self.width + x as usize]
    }
}
