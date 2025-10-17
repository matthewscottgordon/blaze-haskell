use crate::game_state::{Battlesnake, GameState, Move};

mod check_collisions;
use check_collisions::check_collisions;

static MAX_SEARCH_DEPTH: usize = 4;
static WIN_VALUE: f32 = 1.0;
static LOSE_VALUE: f32 = -10.0;

pub async fn devise_plan(game_state: GameState) -> Move {
    find_plan(&game_state, MAX_SEARCH_DEPTH).0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameStatus {
    Win,
    Lose,
    Continue,
}

pub fn find_plan(game_state: &GameState, search_depth: usize) -> (Move, f32) {
    if search_depth > 0 {
        Move::enumerate()
            .map(|player_move| {
                (
                    player_move,
                    combine_scores(
                        get_possible_next_states(game_state, player_move)
                            .map(check_out_of_bounds)
                            .map(check_collisions)
                            .map(|new_game_state| match check_win_lose(&new_game_state) {
                                GameStatus::Win => WIN_VALUE,
                                GameStatus::Lose => LOSE_VALUE,
                                GameStatus::Continue => {
                                    find_plan(&new_game_state, search_depth - 1).1
                                }
                            }),
                    ),
                )
            })
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap()
    } else {
        (Move::Up, heuristic_score(game_state))
    }
}

fn combine_scores(scores: impl ExactSizeIterator<Item = f32>) -> f32 {
    let count = scores.len() as f32;
    scores.sum::<f32>() / count
}

fn check_out_of_bounds(game_state: GameState) -> GameState {
    let check_snake_out_of_bounds = |snake: Battlesnake| {
        if snake.has_gone_oob(game_state.width as i32, game_state.height as i32) {
            Battlesnake::new_dead()
        } else {
            snake
        }
    };
    let player = check_snake_out_of_bounds(game_state.player);
    let enemies = game_state
        .enemies
        .into_iter()
        .map(check_snake_out_of_bounds)
        .collect();
    GameState {
        player,
        enemies,
        ..game_state
    }
}

fn check_win_lose(game_state: &GameState) -> GameStatus {
    if !game_state.player.is_alive() {
        GameStatus::Lose
    } else if !game_state.enemies.is_empty() && game_state.enemies.iter().all(|s| !s.is_alive()) {
        GameStatus::Win
    } else {
        GameStatus::Continue
    }
}

fn heuristic_score(_game_state: &GameState) -> f32 {
    0.5
}

struct MovePermutations {
    next: Vec<Move>,
    num_yielded: usize,
}

impl MovePermutations {
    fn new(num_snakes: usize) -> Self {
        Self {
            next: vec![Move::Up; num_snakes],
            num_yielded: 0,
        }
    }

    fn rotate(v: Move) -> Move {
        match v {
            Move::Up => Move::Down,
            Move::Down => Move::Left,
            Move::Left => Move::Right,
            Move::Right => Move::Up,
        }
    }

    fn step(values: &mut [Move]) -> bool {
        match values.first_mut() {
            Some(Move::Right) => {
                values[0] = Move::Up;
                Self::step(&mut values[1..])
            }
            Some(v) => {
                *v = Self::rotate(*v);
                true
            }
            None => false,
        }
    }
}

impl Iterator for MovePermutations {
    type Item = Vec<Move>;
    fn next(&mut self) -> Option<Vec<Move>> {
        let result = self.next.clone();
        if result.is_empty() {
            None
        } else {
            if !Self::step(&mut self.next) {
                self.next = vec![];
            }
            self.num_yielded += 1;
            Some(result)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let v = (4_usize).pow(self.next.len() as u32) - self.num_yielded;
        (v, Some(v))
    }
}

impl ExactSizeIterator for MovePermutations {
    fn len(&self) -> usize {
        self.size_hint().0
    }
}

fn get_possible_next_states(
    game_state: &GameState,
    player_move: Move,
) -> impl ExactSizeIterator<Item = GameState> {
    MovePermutations::new(game_state.enemies.len().max(1)).map(move |enemy_moves| GameState {
        height: game_state.height,
        width: game_state.width,
        player: game_state.player.update(player_move, &game_state.food),
        enemies: game_state
            .enemies
            .iter()
            .zip(enemy_moves)
            .map(|(s, m)| s.update(m, &game_state.food))
            .filter(|s| !s.is_alive())
            .collect(),
        food: game_state.food.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_permutations() {
        let output: Vec<_> = MovePermutations::new(3).collect();
        assert_eq!(4 * 4 * 4, output.len());
        let directions = [Move::Up, Move::Down, Move::Left, Move::Right];
        for e0 in directions {
            for e1 in directions {
                for e2 in directions {
                    assert!(output.contains(&vec![e0, e1, e2]));
                }
            }
        }
    }

    #[test]
    fn move_permutations_size_hint() {
        for size in 0..16 {
            let target = MovePermutations::new(size);
            let v = (4 as usize).pow(size as u32);
            assert_eq!((v, Some(v)), target.size_hint());
        }
    }

    #[test]
    fn move_permutations_size_len() {
        for size in 0..16 {
            let mut target = MovePermutations::new(size);
            let v = (4 as usize).pow(size as u32);
            assert_eq!(v, target.len());
            target.next();
            if size > 0 {
                assert_eq!(v - 1, target.len());
            }
            if size > 1 {
                target.next();
                assert_eq!(v - 2, target.len());
            }
            if size > 2 {
                target.next();
                assert_eq!(v - 3, target.len());
            }
        }
    }

    #[test]
    fn combine_scores_yields_average() {
        assert_eq!(combine_scores([0.0, 3.5, -3.5, 10.0].iter().copied()), 2.5);
    }

    #[test]
    fn check_win_lose_detects_win_state() {
        let game_state = GameState {
            height: 11,
            width: 11,
            player: Battlesnake::new(&[(1, 2), (1, 3), (1, 4)]),
            enemies: vec![Battlesnake::new_dead(), Battlesnake::new_dead()],
            food: vec![],
        };
        assert_eq!(check_win_lose(&game_state), GameStatus::Win);
    }

    #[test]
    fn check_win_lose_detects_lose_state() {
        let game_state = GameState {
            height: 11,
            width: 11,
            player: Battlesnake::new_dead(),
            enemies: vec![
                Battlesnake::new(&[(4, 2), (4, 3), (4, 4)]),
                Battlesnake::new(&[(3, 5), (3, 6), (4, 6)]),
            ],
            food: vec![],
        };
        assert_eq!(check_win_lose(&game_state), GameStatus::Lose);

        let game_state = GameState {
            enemies: vec![Battlesnake::new_dead(), Battlesnake::new_dead()],
            ..game_state
        };
        assert_eq!(check_win_lose(&game_state), GameStatus::Lose);
    }

    #[test]
    fn check_win_lose_detects_continue_state() {
        let game_state = GameState {
            height: 11,
            width: 11,
            player: Battlesnake::new(&[(1, 2), (1, 3), (1, 4)]),
            enemies: vec![
                Battlesnake::new(&[(4, 2), (4, 3), (4, 4)]),
                Battlesnake::new(&[(3, 5), (3, 6), (4, 6)]),
            ],
            food: vec![],
        };
        assert_eq!(check_win_lose(&game_state), GameStatus::Continue);

        let game_state = GameState {
            enemies: vec![],
            ..game_state
        };
        assert_eq!(check_win_lose(&game_state), GameStatus::Continue);
    }
}
