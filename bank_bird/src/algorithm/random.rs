use super::Algorithm;
use mancala_board::{MancalaBoard, Side};
use rand::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Random();
impl<const S: usize> Algorithm<S> for Random {
    fn name(&self) -> String { "Random".to_string() }
    fn min_games(&self) -> usize { 1000 }
    fn play_move(&mut self, board: &MancalaBoard<S>, side: Side) -> usize {
        let range = 0..S;

        let mut indexes = vec![];

        for index in range {
            if board.side_to_dishes(side)[index] > 0 {
                indexes.push(index);
            }
        }

        indexes.choose(&mut rand::thread_rng()).unwrap().to_owned()
    }
    fn dyn_clone(&self) -> Box<dyn Algorithm<S>> { Box::new(*self) }
}

#[derive(Debug, Clone)]
pub struct SeedRandom { seed: u64, pub rng: StdRng }
impl<const S: usize> Algorithm<S> for SeedRandom {
    fn name(&self) -> String { format!("Seed Random (#{})", self.seed.to_ne_bytes().map(|b| format!("{:02X}", b)).join("")) }
    fn min_games(&self) -> usize { 100 }
    fn play_move(&mut self, board: &MancalaBoard<S>, side: Side) -> usize {
        *board.side_to_dishes(side)
        .iter()
        .enumerate()
        .filter_map(|(i, value)| if value > &0 { Some(i) } else { None })
        .collect::<Vec<_>>()
        .choose(&mut self.rng)
        .expect("the board shouldn't be empty")
    }
    fn dyn_clone(&self) -> Box<dyn Algorithm<S>> { Box::new(self.clone()) }
}

impl SeedRandom {
    pub fn new(seed: u64) -> Self {
        SeedRandom {
            seed,
            rng: rand::rngs::StdRng::seed_from_u64(seed),
        }
    }
}
