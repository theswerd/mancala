use super::Algorithm;
use mancala_board::{MancalaBoard, Side, MoveResult};

#[derive(Debug, Clone, Copy)]
pub struct CaptureAndExtraTurn();
impl<const S: usize> Algorithm<S> for CaptureAndExtraTurn { 
    fn min_games(&self) -> usize { 1 }
    fn name(&self) -> String { "Capture & Extra Turn".to_string() }
    fn play_move(&mut self, board: &MancalaBoard<S>, side: Side) -> usize {
        let mut captures = vec![];
        let mut extra_turns = vec![];

        for i in 0..S {
            let mut board = *board;
            let result = board.move_piece_kalah(side, i);
            match result {
                MoveResult::Capture(_, _) => {
                    let mut profit = *board.side_bank(side);
                    board.capture_kalah(side, i);
                    profit = *board.side_bank(side) - profit;

                    captures.push((i, profit))
                }
                MoveResult::ExtraTurn => extra_turns.push(i),
                _ => {}
            }
        }

        // capture the biggest if possible
        if !captures.is_empty() {
            let a = captures.iter().reduce(|a, b| {
                if b.1 > a.1 {
                    b
                } else {
                    a
                }
            }).unwrap();

            return a.0
        }

        // get an extra turn if possible
        if !extra_turns.is_empty() {
            return *extra_turns.last().unwrap()
        }

        // pick the last move
        for i in (0..S).rev() {
            if board.side_to_dishes(side)[i] > 0 { return i }
        }

        unreachable!()
    }
    fn dyn_clone(&self) -> Box<dyn Algorithm<S>> { Box::new(*self) }
}
