#[cfg(test)]
mod test;

use std::ops::Not;

#[deprecated(
    since = "0.1.2",
    note = "Please use the MancalaBoard::<6>::default() instead"
)]
pub fn basic_board() -> MancalaBoard<6> {
    MancalaBoard {
        left: [4; 6],
        right: [4; 6],
        ..Default::default()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Side {
    Left,
    Right,
}

impl Not for Side {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MoveResult {
    Done(Side, usize),
    Capture(Side, usize),
    ExtraTurn,
    IllegalMove,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Winner {
    Side(Side),
    Tie,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MancalaBoard<const S: usize> {
    pub left: [usize; S],
    pub left_bank: usize,
    pub right: [usize; S],
    pub right_bank: usize,
}

impl<const S: usize> Default for MancalaBoard<S> {
    fn default() -> Self {
        Self {
            left: [4; S],
            right: [4; S],
            left_bank: 0,
            right_bank: 0,
        }
    }
}

impl<const S: usize> MancalaBoard<S> {
    pub fn new(initial: usize) -> MancalaBoard<S> {
        Self {
            left: [initial; S],
            right: [initial; S],
            ..Default::default()
        }
    }

    pub fn print(&self) {
        println!(
            " _____________\n(     {:2}      )\n ‾‾‾‾‾‾‾‾‾‾‾‾‾",
            self.right_bank,
        );
        for index in 0..S {
            println!(" ____  |  ____");
            println!(
                "( {:2} ) | ( {:2} )",
                self.left[index],
                self.right[self.opposite_dish_index(index)],
            );
            println!(" ‾‾‾‾  |  ‾‾‾‾");
        }
        println!(
            " _____________\n(     {:2}      )\n ‾‾‾‾‾‾‾‾‾‾‾‾‾",
            self.left_bank,
        );
    }

    /// Flips the board
    pub fn flip(&mut self) {
        // swap banks
        (self.left_bank, self.right_bank) = (self.right_bank, self.left_bank);

        for i in 0..S {
            // swap dishes
            (self.left[i], self.right[i]) = (self.right[i], self.left[i]);
        }
    }

    /// Returns a flipped version of the board
    pub fn flipped(&self) -> Self {
        let mut board = *self;
        board.flip();
        board
    }

    #[inline]
    /// Clears the selected dish and returns it's contents
    pub fn clear_dish(&mut self, side: Side, index: usize) -> usize {
        macro_rules! clear_dish {
            ($side:expr) => {
                {
                    let dish = $side[index];
                    $side[index] = 0;
                    dish
                }
            };
        }
        match side {
            Side::Left => clear_dish!(self.left),
            Side::Right => clear_dish!(self.right),
        }
    }

    /// Allows you to move an arbitrary dish on the board while it getting collected by an arbitrary player.
    /// Optiomal for avalache implementations. 
    pub fn move_from_side(&mut self, dish_side: Side, dish_index: usize, collector_side: Side) -> MoveResult {
        if !self.is_move_legal(dish_side, dish_index) {
            return MoveResult::IllegalMove
        }

        let placeable_slots = 2 * S + 1;

        let mut hand = self.clear_dish(dish_side, dish_index);
        let mut current_index = dish_index + 1;
        let mut current_side = dish_side;

        loop {
            macro_rules! calculate_bulk {
                () => {
                    if hand >= 2 * placeable_slots { // only run division if you're sure that there's more than 1 value
                        hand / placeable_slots
                    } else {
                        1
                    }
                };
            }

            macro_rules! check_bank {
                ($bank:expr) => {
                    {
                        if current_side == collector_side {
                            let bulk = calculate_bulk!();
                            $bank += bulk;
                            hand -= bulk;
                            if hand == 0 {
                                break MoveResult::ExtraTurn;
                            }
                        }
                        current_side = !current_side;
                        current_index = 0;
                    }
                };
            }

            macro_rules! check_dish {
                ($dishes:expr) => {
                    {
                        let bulk = calculate_bulk!();
                        $dishes[current_index] += bulk;
                        hand -= bulk;
                        if hand == 0 {
                            if
                                current_side == collector_side && // if it's your side
                                $dishes[current_index] == 1 && // if it was previously empty
                                self.side_to_dishes(!collector_side)[self.opposite_dish_index(current_index)] > 0 // if the other side has something in it
                            {
                                break MoveResult::Capture(current_side, current_index)
                            }
                        }
                        current_index += 1;
                    }
                };
            }

            match current_side {
                Side::Left if current_index >= S => check_bank!(self.left_bank),
                Side::Right if current_index >= S => check_bank!(self.right_bank),
                Side::Left => check_dish!(self.left),
                Side::Right => check_dish!(self.right),
            }

            if hand == 0 {
                break MoveResult::Done(current_side, current_index - 1); // underflow is unreachable
            }
        }
    }

    /// Moves the selected dish into the hand and moves them in an anti-clockwise direction.
    pub fn move_piece(&mut self, side: Side, index: usize) -> MoveResult {
        self.move_from_side(side, index, side)
    }

    /// Captures the selected and the opposing dish, and places them in the selected side
    pub fn capture(&mut self, side: Side, index: usize) {
        let other_side_index = self.opposite_dish_index(index);
        let current_side = self.clear_dish(side, index);
        let other_side = self.clear_dish(!side, other_side_index);
        let bank = self.side_bank(side);
        *bank += current_side + other_side;
    }

    /// Collects all the dishes and places them in their respective side's bank.
    pub fn collect_dishes(&mut self) {
        self.left_bank += self.left.iter().sum::<usize>();
        self.left = [0; S];
        self.right_bank += self.right.iter().sum::<usize>();
        self.right = [0; S];
    }

    /// Checks if the side is empty
    #[inline]
    pub fn is_side_empty(&self, side: Side) -> bool {
        let side_dishes = self.side_to_dishes(side).as_ref();
        side_dishes.iter().all(|d| d == &0)
    }

    #[inline]
    pub fn side_to_dishes(&self, side: Side) -> &[usize; S] {
        match side {
            Side::Left => &self.left,
            Side::Right => &self.right,
        }
    }

    #[inline]
    pub fn side_bank(&mut self, side: Side) -> &mut usize {
        match side {
            Side::Left => &mut self.left_bank,
            Side::Right => &mut self.right_bank,
        }
    }

    #[inline]
    pub fn game_over(&self) -> bool {
        self.is_side_empty(Side::Left) || self.is_side_empty(Side::Right)
    }

    #[inline]
    pub fn winner(&self) -> Winner {
        let left = self.left.iter().sum::<usize>() + self.left_bank;
        let right = self.right.iter().sum::<usize>() + self.right_bank;

        use std::cmp::Ordering::*;
        match left.cmp(&right) {
            Greater => Winner::Side(Side::Left),
            Equal => Winner::Tie,
            Less => Winner::Side(Side::Right),
        }
    }

    #[inline]
    pub fn is_move_legal(&mut self, side: Side, index: usize) -> bool {
        (0..S).contains(&index) && self.side_to_dishes(side)[index] > 0
    }

    #[inline]
    pub fn opposite_dish_index(&self, index: usize) -> usize {
        S - 1 - index
    }
}
