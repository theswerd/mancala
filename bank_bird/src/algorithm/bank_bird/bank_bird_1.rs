use super::super::Algorithm;

use mancala_board::{MancalaBoard, Side, MoveResult, Winner};

/// # BankBird v1
/// in a nutshell:
/// - it recursively runs all possible games at a specific depth
/// - each depth-ending gives a f64 of the progress of the board
/// - every move gets averaged
/// - it selects the move with the highest score
#[derive(Debug, Clone, Copy)]
pub struct BankBird1(pub usize); // depth
impl<const S: usize> Algorithm<S> for BankBird1 {
    fn name(&self) -> String { format!("Bank Bird v1 (d:{})", self.0) }
    fn min_games(&self) -> usize { 1 }
    fn play_move(&mut self, board: &MancalaBoard<S>, side: Side) -> usize {
        fn recursive<const S: usize>(mut board: MancalaBoard<S>, side: Side, depth: usize) -> Vec<f64> {
            if depth == 0 {
                return vec![calculate_side_score_v1(&mut board, side)];
            }

            if board.game_over() {
                return vec![match board.winner() {
                    Winner::Side(winner) => {
                        if winner == side {
                            20.0
                        } else {
                            -20.0
                        }
                    },
                    Winner::Tie => -10.0,
                }];
            }

            let mut values = vec![];

            for i in 0..S {
                let mut board = board;

                match board.move_piece_kalah(side, i) {
                    MoveResult::Done(_,_) => {
                        values.push(calculate_side_score_v1(&mut board, side) - average_v1(recursive(board, !side, depth - 1)))
                    }
                    MoveResult::Capture(side, ci) => {
                        board.capture_kalah(side, ci); // TODO: FIX CAPTURING SIDE
                        values.push(calculate_side_score_v1(&mut board, side) - average_v1(recursive(board, !side, depth - 1)))
                    }
                    MoveResult::ExtraTurn => {
                        values.push(calculate_side_score_v1(&mut board, side) + average_v1(recursive(board, side, depth - 1)))
                    }
                    MoveResult::IllegalMove => {
                        values.push(0.0)
                    }
                }
            }

            values
        }

        let scores = recursive(*board, side, self.0);
        let mut scores = scores.iter().enumerate().collect::<Vec<_>>();
        scores.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

        for score in scores {
            let index = score.0;

            if board.side_to_dishes(side)[index] > 0 {
                return index
            }
        }

        unreachable!("theoretically if it wasn't able to play anything, it means that there was a game over")
    }
    fn dyn_clone(&self) -> Box<dyn Algorithm<S>> { Box::new(*self) }
}

pub fn average_v1(vector: Vec<f64>) -> f64 {
    if vector.is_empty() {
        0.0
    } else {
        vector.iter().sum::<f64>() / vector.len() as f64
    }
}

pub fn calculate_side_score_v1<const S: usize>(board: &mut MancalaBoard<S>, side: Side) -> f64 {
    let bank_side = *board.side_bank(side);
    let other_bank_side = *board.side_bank(side);

    let board_side = &board.side_to_dishes(side)[0..S];
    let other_board_side = &board.side_to_dishes(!side)[0..S];

    let mut score = (bank_side as f64).powf(0.3) - (other_bank_side as f64).powf(0.4); // I have no fucking idea what I'm doing

    for (index, quantity) in board_side.iter().enumerate() { 
        score += *quantity as f64 / ((index + 1) as f64 / 3.0);
        score -= other_board_side[index] as f64 / 4.0;
    }

    score
}
