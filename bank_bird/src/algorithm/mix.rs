use super::Algorithm;
use arrayvec::ArrayVec;
use mancala_board::{MancalaBoard, Side, MoveResult};

#[derive(Debug, Clone, Copy)]
pub struct CaptureAndExtraTurn();
impl<const S: usize> Algorithm<S> for CaptureAndExtraTurn { 
    fn min_games(&self) -> usize { 1 }
    fn name(&self) -> String { "Capture & Extra Turn".to_string() }
    fn play_move(&mut self, board: &MancalaBoard<S>, side: Side) -> usize {
        let mut captures: ArrayVec<(usize, u32), S> = ArrayVec::new();
        let mut extra_turns: ArrayVec<usize, S> = ArrayVec::new();

        for i in 0..S {
            let mut board = *board;
            let result = board.move_piece_kalah(side, i);
            match result {
                MoveResult::Capture(_, _) => {
                    let profit = board.side_to_dishes(side)[i] + board.side_to_dishes(!side)[board.opposite_dish_index(i)];
                    captures.push((i, profit))
                }
                MoveResult::ExtraTurn => extra_turns.push(i),
                _ => {}
            }
        }

        // capture the biggest if possible
        if let Some((i, _)) = captures.into_iter().max_by_key(|x| x.1) {
            return i;
        }

        // get an extra turn if possible
        if let Some(&i) = extra_turns.last() {
            return i;
        }

        // pick the last move
        for i in (0..S).rev() {
            if board.side_to_dishes(side)[i] > 0 { return i }
        }

        unreachable!()
    }
    fn dyn_clone(&self) -> Box<dyn Algorithm<S>> { Box::new(*self) }
}
