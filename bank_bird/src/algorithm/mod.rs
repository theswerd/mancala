use mancala_board::{MancalaBoard, Side};

pub mod deterministic;
pub mod random;
pub mod mix;
pub mod bank_bird;

pub trait Algorithm<const S: usize>: Sync+Send {
    fn name(&self) -> String;
    fn min_games(&self) -> usize;
    fn play_move(&mut self, board: &MancalaBoard<S>, side: Side) -> usize;
    fn dyn_clone(&self) -> Box<dyn Algorithm<S>>;
}
