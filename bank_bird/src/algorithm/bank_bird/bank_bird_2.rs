use super::super::Algorithm;

use mancala_board::{MancalaBoard, Side, MoveResult, Winner};
use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
enum PlayerType {
    Maximizing,
    Minimizing,
}

fn opposite_type(typ: PlayerType) -> PlayerType {
    match typ {
        PlayerType::Maximizing => PlayerType::Minimizing,
        PlayerType::Minimizing => PlayerType::Maximizing,
    }
}

/// an attempt at minimax
#[derive(Debug, Clone, Copy)]
pub struct BankBird2(pub usize); // depth
impl<const S: usize> Algorithm<S> for BankBird2 {
    fn name(&self) -> String { format!("Bank Bird v2 (d:{})", self.0) }
    fn min_games(&self) -> usize { 1 }
    fn play_move(&mut self, board: &MancalaBoard<S>, side: Side) -> usize {
        let current_type = match side {
            Side::Left => PlayerType::Maximizing,
            Side::Right => PlayerType::Minimizing,
        };

        let a = recursive(*board, self.0, side, current_type);
        let chosen = choose(&a, current_type);
        // println!(
        //     "{:?} {} {:?} {}\n{:?}\n{:?}\n",
        //     board.left,
        //     board.left_bank,
        //     board.right,
        //     board.right_bank,
        //     a,
        //     chosen,
        // );
        chosen.0
    }
    fn dyn_clone(&self) -> Box<dyn Algorithm<S>> { Box::new(*self) }
}

fn choose(moves: &[Option<(usize, f64)>], typ: PlayerType) -> (usize, f64) {
    if moves.is_empty() {
        panic!("this shouldn never happen (hopefully)");
    }

    let output = moves
        .into_iter()
        .filter_map(|v| *v)
        .reduce(|curr, next|
            match typ {
                PlayerType::Maximizing => if next.1 > curr.1 { curr } else { next },
                PlayerType::Minimizing => if next.1 < curr.1 { curr } else { next },
            }
        )
        .expect("the vector shouldn't be empty");

        // println!("{:?} ({:?})", output, moves);

    output
}

fn recursive<const S: usize>(board: MancalaBoard<S>, depth: usize, side: Side, typ: PlayerType) -> [Option<(usize, f64)>; S] {
    if depth == 0 || board.game_over() {
        return [Some((0, calculate_side_score_v2(&board))); S]
    }

    (0..S)
    .into_par_iter()
    .map(|i| {
        let mut board = board;
        let output = board.move_piece_kalah(side, i);
        
        use MoveResult::*;
        match output {
            Done(_, _) => {
                let next_moves = recursive(board, depth - 1, !side, opposite_type(typ));
                Some((i, choose(&next_moves, typ).1))
            }
            Capture(cs, ci) => {
                if cs == side {
                    board.capture_kalah(cs, ci);
                } else {
                    board.capture_kalah(side, board.opposite_dish_index(ci));
                }
                let next_moves = recursive(board, depth - 1, !side, opposite_type(typ));
                Some((i, choose(&next_moves, typ).1))
            }
            ExtraTurn => {
                let next_moves = recursive(board, depth - 1, side, typ);
                Some((i, choose(&next_moves, typ).1))
            }
            IllegalMove => None,
        }
    })
    .collect::<Vec<_>>()
    .try_into().unwrap()
}

pub fn calculate_side_score_v2<const S: usize>(board: &MancalaBoard<S>) -> f64 {
    let total_pieces: u32 =
        board.left.iter().sum::<u32>()
        + board.right.iter().sum::<u32>()
        + board.left_bank
        + board.right_bank;

    let win_score = if board.game_over() || board.left_bank > total_pieces / 2 || board.right_bank > total_pieces / 2 {
        match board.winner() {
            Winner::Side(Side::Left) => 1000.0,
            Winner::Side(Side::Right) => -1000.0,
            Winner::Tie => 0.0,
        }
    } else { 0.0 };

    macro_rules! count_side {
        ($side:ident) => {
            f64::from(
                board
                .$side
                .iter()
                .map(|v| *v as u32)
                .sum::<u32>()
            )
        };
    }
    macro_rules! count_moves {
        ($side:expr) => {{
            let mut dones = 0;
            let mut captures = 0;
            let mut extra_turns = 0;
            let mut illegal_moves = 0;
    
            for i in 0..S {
                let mut board = *board;
                match board.move_piece_kalah($side, i) {
                    MoveResult::Done(_, _) => dones += 1,
                    MoveResult::Capture(_, _) => captures += 1,
                    MoveResult::ExtraTurn => extra_turns += 1,
                    MoveResult::IllegalMove => illegal_moves += 1,
                }
            }
    
            (f64::from(dones), f64::from(captures), f64::from(extra_turns), f64::from(illegal_moves))
        }};
    }

    let banks = f64::from(board.left_bank as i32 - board.right_bank as i32);

    let left_side = count_side!(left);
    let right_side = count_side!(right);
    
    let (
        left_dones,
        left_captures,
        left_extra_turns,
        left_illegal_moves,
    ) = count_moves!(Side::Left);

    let (
        right_dones,
        right_captures,
        right_extra_turns,
        right_illegal_moves,
    ) = count_moves!(Side::Right);

    40.0 * banks
    + 2.0 * (left_side - right_side)
    + 8.0 * (left_extra_turns - right_extra_turns)
    + 25.0 * (left_captures - right_captures)
    - 0.5 * (left_illegal_moves - right_illegal_moves) // this is a bad thing, so it gets a -
    + 1.0 * (left_dones - right_dones)
    + win_score
}
