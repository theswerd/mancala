use crate::{MUInt, MancalaBoard, MoveResult, Side, Winner};

impl<const S: usize> MancalaBoard<S> {
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
    pub fn clear_dish(&mut self, side: Side, index: usize) -> MUInt {
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

    /// Collects all the dishes and places them in their respective side's bank.
    pub fn collect_dishes(&mut self) {
        self.left_bank += self.left.iter().sum::<MUInt>();
        self.left = [0; S];
        self.right_bank += self.right.iter().sum::<MUInt>();
        self.right = [0; S];
    }

    /// Checks if the side is empty
    #[inline]
    pub fn is_side_empty(&self, side: Side) -> bool {
        let side_dishes = self.side_to_dishes(side);
        side_dishes.iter().all(|d| d == &0)
    }

    #[inline]
    pub const fn side_to_dishes(&self, side: Side) -> &[MUInt; S] {
        match side {
            Side::Left => &self.left,
            Side::Right => &self.right,
        }
    }

    pub fn side_to_dishes_mut(&mut self, side: Side) -> &mut [MUInt; S] {
        match side {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        }
    }

    #[inline]
    pub fn side_bank(&mut self, side: Side) -> &mut MUInt {
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
        let left = self.left.iter().sum::<MUInt>() + self.left_bank;
        let right = self.right.iter().sum::<MUInt>() + self.right_bank;

        use core::cmp::Ordering::*;
        match left.cmp(&right) {
            Greater => Winner::Side(Side::Left),
            Equal => Winner::Tie,
            Less => Winner::Side(Side::Right),
        }
    }

    #[inline]
    pub const fn is_move_legal(&self, side: Side, index: usize) -> bool {
        index < S && self.side_to_dishes(side)[index] > 0
    }

    #[inline]
    pub const fn opposite_dish_index(&self, index: usize) -> usize {
        S - 1 - index
    }

    #[cfg(feature = "std")]
    pub fn print_vertical(&self) {
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

    #[cfg(feature = "std")]
    pub fn print_horizontal(&self) {
        let top = self.right.iter().rev().map(|v| format!("[{:2}]", v)).collect::<Vec<_>>().join("");
        let bottom = self.left.iter().map(|v| format!("[{:2}]", v)).collect::<Vec<_>>().join("");
        println!("┌──┐{}┌──┐", top);
        println!("│{:2}│{}│{:2}│", self.right_bank, " ".repeat(top.len()), self.left_bank);
        println!("└──┘{}└──┘", bottom);
    }
}

impl MoveResult {
    /// Returns if the playing side should be changed depending on the resulting move type (i.e. on `Capture`s and `Done`s)
    #[inline]
    pub fn change_side(&self) -> bool {
        match self {
            MoveResult::Capture(_, _)|MoveResult::Done(_, _) => true,
            MoveResult::ExtraTurn|MoveResult::IllegalMove => false,
        }
    }
}
