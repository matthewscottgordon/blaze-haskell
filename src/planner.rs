use crate::game_state::{GameState, Move};

pub async fn devise_plan(game_state: GameState) -> Move {
    Move::enumerate()
        .map(|player_move| {
            (
                player_move,
                combine_scores(
                    get_possible_next_states(&game_state, player_move).map(|s| score_gamestate(&s)),
                ),
            )
        })
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap()
        .0
}

fn score_gamestate(_game_state: &GameState) -> f32 {
    todo!()
}

fn combine_scores(scores: impl ExactSizeIterator<Item = f32>) -> f32 {
    let count = scores.len() as f32;
    scores.sum::<f32>() / count
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
    MovePermutations::new(game_state.enemies.len()).map(move |enemy_moves| GameState {
        height: game_state.height,
        width: game_state.width,
        player: game_state.player.update(player_move),
        enemies: game_state
            .enemies
            .iter()
            .zip(enemy_moves)
            .map(|(s, m)| s.update(m))
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
                assert_eq!(v-1, target.len());
            }
            if size > 1 {
                target.next();
                assert_eq!(v-2, target.len());
            }
            if size > 2 {
                target.next();
                assert_eq!(v-3, target.len());
            }
        }
    }

    #[test]
    fn combine_scores_yields_average() {
        assert_eq!(combine_scores([0.0, 3.5, -3.5, 10.0].iter().copied()), 2.5);
    }
}
