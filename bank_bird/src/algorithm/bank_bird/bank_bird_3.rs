use super::super::Algorithm;

use mancala_board::{MancalaBoard, Side, MoveResult};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

/// an attempt at getting some bitches
#[derive(Debug, Clone)]
pub struct BankBird3<const S: usize>(pub usize); // depth
impl<const S: usize> Algorithm<S> for BankBird3<S> {
    fn name(&self) -> String { format!("Bank Bird v3 (d:{})", self.0) }
    fn min_games(&self) -> usize { 1 }
    fn play_move(&mut self, board: &MancalaBoard<S>, side: Side) -> usize {
        let alpha = f64::NEG_INFINITY;
        let beta = f64::INFINITY;

        let moves: Vec<f64> = (0..S).into_par_iter().map(|i| {
            let mut board = *board;
            let mut side = side;
            match board.move_piece_kalah(side, i) {
                MoveResult::Done(_, _) => side = !side,
                MoveResult::Capture(_, i) => { board.capture_kalah(side, i); side = !side },
                MoveResult::ExtraTurn => {},
                MoveResult::IllegalMove => return f64::NAN,
            }
            minimax(board, alpha, beta, side, self.0)
        })
        .collect();

        // println!("{moves:?}");

        let iter = moves
        .iter()
        .enumerate()
        .filter(|(_, n)| n.is_finite());

        // TODO: discover why I need to swap the min_by and max_by
        match side {
            Side::Left => iter.min_by(|(_, a), (_, b)| a.total_cmp(b)),
            Side::Right => iter.max_by(|(_, a), (_, b)| a.total_cmp(b)),
        }
        .map(|(i, _)| i)
        .unwrap_or(S)
    }
    fn dyn_clone(&self) -> Box<dyn Algorithm<S>> { Box::new(self.clone()) }
}

fn minimax<const S: usize>(board: MancalaBoard<S>, mut alpha: f64, mut beta: f64, side: Side, depth: usize) -> f64 {
    if depth == 0 || board.game_over() {
        let score = calculate_side_score_v3(&board);
        return score
    }

    let mut max_eval = f64::NEG_INFINITY;
    let mut min_eval = f64::INFINITY;

    for i in 0..S {
        let mut board = board;

        let mut extra_turn = false;

        let current_move = board.move_piece_kalah(side, i);
        match current_move {
            MoveResult::Done(_, _) => {}
            MoveResult::Capture(_, index) => { board.capture_kalah(side, index) }
            MoveResult::ExtraTurn => { extra_turn = true }
            MoveResult::IllegalMove => continue,
        }

        let eval = minimax(
            board,
            alpha,
            beta,
            if extra_turn { side } else { !side },
            depth - 1, // if extra_turn { depth } else { depth - 1 },
        );

        match side {
            Side::Left => { // maximizing
                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);
            }
            Side::Right => { // minimizing
                min_eval = min_eval.min(eval);
                beta = beta.min(eval);
            }
        }
        // println!("({alpha:?}/{beta:?})");

        if beta <= alpha { break }
    }

    match side {
        Side::Left => max_eval,
        Side::Right => min_eval,
    }
}

pub fn calculate_side_score_v3<const S: usize>(board: &MancalaBoard<S>) -> f64 {
    let bank_score = board.left_bank as isize - board.right_bank as isize;
    let dishes_score = board.left.iter().sum::<u32>() as isize - board.right.iter().sum::<u32>() as isize;
    let left_dishes_difference = *board.left.iter().max().unwrap_or(&0) as isize - *board.left.iter().min().unwrap_or(&0) as isize;
    let right_dishes_difference = *board.right.iter().max().unwrap_or(&0) as isize - *board.right.iter().min().unwrap_or(&0) as isize;

    let total_pieces: u32 = board.left_bank + board.left.iter().sum::<u32>() + board.right_bank + board.right.iter().sum::<u32>();

    let left_win = if board.left_bank >= total_pieces / 2 { 100 } else { 0 };
    let right_win = if board.right_bank >= total_pieces / 2 { 100 } else { 0 };

    let score = bank_score + dishes_score - left_dishes_difference + right_dishes_difference + left_win + right_win;
    score as f64
}
