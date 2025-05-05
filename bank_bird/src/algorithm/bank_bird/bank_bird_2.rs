use arrayvec::ArrayVec;
use mancala_board::{MancalaBoard, MoveResult, Side, Winner};

use crate::Algorithm;

pub struct SearchResult {
    pub score: f64,
    pub best_move: Option<usize>
}

/// an attempt at getting some bitches
#[derive(Debug, Clone)]
pub struct BankBird2<const S: usize>(pub usize); // depth
impl<const S: usize> Algorithm<S> for BankBird2<S> {
    fn name(&self) -> String { format!("Bank Bird v2 (d:{})", self.0) }
    fn min_games(&self) -> usize { 1 }
    fn play_move(&mut self, board: &MancalaBoard<S>, side: Side) -> usize {
        // probably can be computed instead of a lookup array
        if S >= VALUES.len() { panic!("Boards bigger than {} are not supported", VALUES.len()) }

        BankBird2::negamax(board, self.0, f64::NEG_INFINITY, f64::INFINITY, side).best_move.expect(format!("{board:?} at least a valid move").as_str())
    }
    fn dyn_clone(&self) -> Box<dyn Algorithm<S>> { Box::new(self.clone()) }
}

impl<const S: usize> BankBird2<S> {
    fn negamax(board: &MancalaBoard<S>, depth: usize, mut alpha: f64, beta: f64, side: Side) -> SearchResult {
        if depth == 0 || board.game_over() {
            return SearchResult {
                score: calculate_score_v2(&board) as f64 * (if side == Side::Left { 1.0 } else { -1.0 }),
                best_move: None
            }
        }

        let mut value: f64 = f64::NEG_INFINITY;
        let mut best_move: Option<usize> = None;
        let mut moves = (0..S).into_iter()
            .filter(|m| board.is_move_legal(side, *m))
            .collect::<ArrayVec<usize, S>>();

        moves.sort_unstable_by(|a, b| {
            let score_a = BankBird2::score_move(&board, *a, side);
            let score_b = BankBird2::score_move(&board, *b, side);
            score_b.total_cmp(&score_a)
        });

        for m in moves {
            let mut board = *board;
            
            let move_result = board.move_piece_kalah(side, m);
            match move_result {
                MoveResult::Capture(cs, ci) => board.capture_kalah(cs, ci),
                _ => {},
            }

            let result = if move_result.change_side() {
                -BankBird2::negamax(&board, depth - 1, -beta, -alpha, !side).score
            } else {
                BankBird2::negamax(&board, depth - 1, alpha, beta, side).score
            };

            if result > value {
                value = result;
                best_move = Some(m);
            }
            
            alpha = alpha.max(value);

            if alpha >= beta {
                break;
            }
        }

        SearchResult {
            score: value,
            best_move
        }
    }

    fn score_move(board: &MancalaBoard<S>, m: usize, side: Side) -> f64 {
        let mut board = *board;
        let result = board.move_piece_kalah(side, m);

        // these values could be tweaked, maybe some work better than others
        // they are very much arbitrary
        let value = match result {
            MoveResult::IllegalMove => -100.0,
            MoveResult::ExtraTurn => 3.0,
            MoveResult::Done(_, _) => 0.0,
            MoveResult::Capture(cs, ci) => {
                let current_side = board.side_to_dishes(cs)[ci];
                let other_side = board.side_to_dishes(!cs)[board.opposite_dish_index(ci)];
                (current_side as f64 + other_side as f64) * 0.75
            }
        };

        // i think its because of the calculate_v2 function
        // it uses map

        // oh its not used
        value + VALUES[m] * 0.3
    }
}

// not sure what the best way to do it is
// maybe kr8gz does know
// wjat

// S is usize, we need VALUES[S]

// here are 16 values but technically it could be any size

// those are positional values
// S is just how many whatever these things are called

// the VALUES is calculated with m.powf(1.5) + 0.6

// i dont think we need a mancala of size 2^32-1
// maybe just limit it to u8?
// S is the size of the board, thats way more than necessary

// yeah this engine supports mancala of size 2^64 - 1 if your computer allows it
// definitely overkill
// lets just treat it as u8 for now

// we should just use 16 and call it a day

const VALUES: [f64; 16] = [0.6, 0.62, 0.65, 0.69, 0.74, 0.80, 0.85, 0.92, 0.99, 1.06, 1.14, 1.23, 1.32, 1.40, 1.50, 1.60];

const BANK_MULT: u32 = 3;

// oh it wasnt used
// idk then
// i enabled the score_v2, but it didnt update yet
// oh there it is
// it should slow it down a bit too but i think it improves strength
// it was able to win with last more often

// ill figure out transposition tables later, that should speed it up more
// and maybe aspiration windows

fn calculate_score_v2<const S: usize>(board: &MancalaBoard<S>) -> f64 {
    let total_win_score: i32 = BANK_MULT as i32 * (S * 2 * 10) as i32;

    let win_score = if board.game_over() {
        match board.winner() {
            Winner::Side(Side::Left) => total_win_score,
            Winner::Side(Side::Right) => -total_win_score,
            Winner::Tie => 0,
        }
    } else { 0 };

    let left_side = board.left.iter()
    .zip(VALUES.iter())
    .map(|(&v, &pos_val)| pos_val * (v as f64))
    .sum::<f64>() + board.left_bank as f64 * BANK_MULT as f64;

    let right_side = board.right.iter()
        .zip(VALUES.iter())
        .map(|(&v, &pos_val)| pos_val * (v as f64))
        .sum::<f64>() + board.right_bank as f64 * BANK_MULT as f64;

    win_score as f64
    + left_side
    - right_side
}
