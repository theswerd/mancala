#[cfg(test)]
mod test;

#[deprecated(
    since = "0.1.2",
    note = "Please use the MancalaBoard::default() instead"
)]
pub fn basic_board() -> MancalaBoard {
    MancalaBoard {
        values: [0, 4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4],
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MoveResult {
    Capture(usize),
    ExtraTurn,
    IllegalMove,
    Done,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Winner {
    Side(Side),
    Tie,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct MancalaBoard {
    pub values: [u32; 14],
}

impl MancalaBoard {
    pub fn new() -> MancalaBoard {
        MancalaBoard { values: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }
    }

    pub fn default() -> MancalaBoard {
        MancalaBoard {
            values: [0, 4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4],
        }
    }

    pub fn from_position(values: [u32; 14]) -> MancalaBoard {
        MancalaBoard { values }
    }

    pub fn print(&self) {
        println!(
            " _____________\n(     {:2}      )\n ‾‾‾‾‾‾‾‾‾‾‾‾‾",
            self.values[0]
        );
        for index in 1..7 {
            println!(" ____  |  ____");
            println!(
                "( {:2} ) | ( {:2} )",
                self.values[index],
                self.values[14 - index]
            );
            println!(" ‾‾‾‾  |  ‾‾‾‾");
        }
        println!(
            " _____________\n(     {:2}      )\n ‾‾‾‾‾‾‾‾‾‾‾‾‾",
            self.values[7]
        );
    }

    pub fn flip(&mut self) {
        for i in 0..7 {
            self.values.swap(i, i + 7)
        }
    }

    /// Clears the selected dish and returns it's contents
    pub fn clear_dish(&mut self, index: usize) -> u32 {
        let dish = self.values[index];
        self.values[index] = 0;
        dish
    }

    // Fixes indexing for a specific side, so you can always input between 0 and 6
    pub fn move_from_side(&mut self, index: usize, side: Side) -> MoveResult {
        let offset = match side {
            Side::Left => 0,
            Side::Right => 7,
        };
        self.move_piece(index + offset, side)
    }

    /// Moves the selected dish into the hand moves them in anti-clockwise direction (by incrementing index).
    pub fn move_piece(&mut self, index: usize, side: Side) -> MoveResult {
        if !self.is_move_legal(index) {
            return MoveResult::IllegalMove
        }

        let mut hand = self.clear_dish(index);

        let mut offset = 1;

        while hand > 0 {
            let current_index = (index + offset) % 14;

            if !(current_index == 0 && side == Side::Left || current_index == 7 && side == Side::Right) {
                self.values[current_index] += 1;
                hand -= 1;
            }

            if hand == 0 {
                if ![0, 7].contains(&current_index) && self.values[current_index] == 1 {
                    return MoveResult::Capture(current_index)
                }
                
                if current_index == 0 && side == Side::Right || current_index == 7 && side == Side::Left {
                    return MoveResult::ExtraTurn
                }
            }

            offset += 1;
        }

        MoveResult::Done
    }

    /// Captures the selected and the opposing dish, and places them in the selected side
    pub fn capture(&mut self, index: usize, side: Side) {
        let other_side_index = (index + 7) % 14;

        let bank_index = match side {
            Side::Left => 7,
            Side::Right => 0,
        };
        
        self.values[bank_index] += self.clear_dish(index) + self.clear_dish(other_side_index);
    }

    /// Collects all the dishes and places them in their respective side's bank.
    pub fn collect_dishes(&mut self) {
        for i in 1..7 { // left side
            self.values[7] += self.clear_dish(i);
        }
        for i in 8..14 { // right side
            self.values[0] += self.clear_dish(i);
        }
    }

    /// Checks if the side is empty
    pub fn is_side_empty(&self, side: Side) -> bool {
        let slice = match side {
            Side::Left => 1..7,
            Side::Right => 8..14,
        };

        self.values[slice]
            .iter()
            .all(|&quantity| quantity == 0)
    }

    pub fn game_over(&self) -> bool {
        self.is_side_empty(Side::Left) || self.is_side_empty(Side::Right)
    }

    pub fn winner(&mut self) -> Winner {
        use std::cmp::Ordering::*;

        match self.values[0].cmp(&self.values[7]) {
            Less => Winner::Side(Side::Left),
            Greater => Winner::Side(Side::Right),
            Equal => Winner::Tie,
        }
    }

    pub fn is_move_legal(&self, index: usize) -> bool {
        ![0, 7].contains(&index) && self.values[index] > 0
    }
}
