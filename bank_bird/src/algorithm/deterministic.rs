use std::cmp::Ordering;

use super::Algorithm;
use mancala_board::{MancalaBoard, Side};

#[derive(Debug, Clone, Copy)]
pub struct First();
impl<const S: usize> Algorithm<S> for First {
    fn name(&self) -> String { "First".to_string() }
    fn min_games(&self) -> usize { 1 }
    fn play_move(&mut self, board: &MancalaBoard<S>, side: Side) -> usize {
        board.side_to_dishes(side).iter().position(|&x| x > 0).unwrap_or_default()
    }
    fn dyn_clone(&self) -> Box<dyn Algorithm<S>> { Box::new(*self) }
}

#[derive(Debug, Clone, Copy)]
pub struct Last();
impl<const S: usize> Algorithm<S> for Last {
    fn name(&self) -> String { "Last".to_string() }
    fn min_games(&self) -> usize { 1 }
    fn play_move(&mut self, board: &MancalaBoard<S>, side: Side) -> usize {
        board.side_to_dishes(side).iter().enumerate().rev().find(|(_, &x)| x > 0).unwrap().0
    }
    fn dyn_clone(&self) -> Box<dyn Algorithm<S>> { Box::new(*self) }
}

#[derive(Debug, Clone, Copy)]
pub struct Biggest(pub bool); // false = first, true = last
impl<const S: usize> Algorithm<S> for Biggest {
    fn name(&self) -> String { format!("Biggest ({})", if self.0 { "last" } else { "first"}) }
    fn min_games(&self) -> usize { 1 }
    fn play_move(&mut self, board: &MancalaBoard<S>, side: Side) -> usize {
        board.side_to_dishes(side)
            .iter().enumerate()
            .max_by(|&(_, &a), &(_, &b)| a.cmp(&b).then(if self.0 { Ordering::Less } else { Ordering::Greater }))
            .map(|(i, _)| i).unwrap()
    }
    fn dyn_clone(&self) -> Box<dyn Algorithm<S>> { Box::new(*self) }
}

#[derive(Debug, Clone, Copy)]
pub struct Smallest(pub bool); // false = first, true = last
impl<const S: usize> Algorithm<S> for Smallest {
    fn name(&self) -> String { format!("Smallest ({})", if self.0 { "last" } else { "first"}) }
    fn min_games(&self) -> usize { 1 }
    fn play_move(&mut self, board: &MancalaBoard<S>, side: Side) -> usize {
        board.side_to_dishes(side)
            .iter().enumerate()
            .filter(|&(_, &v)| v > 0)
            .min_by(|&(_, &a), &(_, &b)| a.cmp(&b).then(if self.0 { Ordering::Greater } else { Ordering::Less }))
            .map(|(i, _)| i).unwrap()
    }
    fn dyn_clone(&self) -> Box<dyn Algorithm<S>> { Box::new(*self) }
}

const PI: &[u8] = b"31415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679821480865132823066470938446095505822317253594081284811174502841027019385211055596446229489549303819644288109756659334461284756482337867831652712019091456485669234603486104543266482133936072602491412737245870066063155881748815209209628292540917153643678925903600113305305488204665213841469519415116094330572703657595919530921861173819326117931051185480744623799627495673518857527248912279381830119491298336733624406566430860213949463952247371907021798609437027705392171762931767523846748184676694051320005681271452635608277857713427577896091736371787214684409012249534301465495853710507922796892589235420199561121290219608640344181598136297747713099605187072113499999983729780499510597317328160963185950244594553469083026425223082533446850352619311881710100031378387528865875332083814206171776691473035982534904287554687311595628638823537875937519577818577805321712268066130019278766111959092164201989";

pub struct Pi(Box<dyn Iterator<Item = usize> + Sync + Send>);
impl<const S: usize> Algorithm<S> for Pi {
    fn name(&self) -> String { String::from("Pi") }
    fn min_games(&self) -> usize { 1 }
    fn play_move(&mut self, board: &MancalaBoard<S>, side: Side) -> usize {
        self.0.find(|&x| board.is_move_legal(side, x)).unwrap()
    }
    fn dyn_clone(&self) -> Box<dyn Algorithm<S>> { Box::new(Self::default()) }
}

impl Default for Pi {
    fn default() -> Self {
        Self(Box::new(PI.iter().cycle().map(|b| usize::from(b - b'0'))))
    }
}
