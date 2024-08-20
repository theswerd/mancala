use crate::{BankCollector, MUInt, MancalaBoard, MoveResult, Side};

impl<const S: usize> MancalaBoard<S> {
    /// Moves the selected dish into the hand and moves them in an anti-clockwise direction, while using the side's bank.
    pub fn move_piece_kalah(&mut self, side: Side, index: usize) -> MoveResult {
        self.move_from_side(side, index, BankCollector::Side(side))
    }

    /// Moves the selected dish into the hand and moves them in an anti-clockwise direction, without using banks.
    pub fn move_piece_oware(&mut self, side: Side, index: usize) -> MoveResult {
        self.move_from_side(side, index, BankCollector::None)
    }

    /// Moves the selected dish into the hand and moves them in an anti-clockwise direction, if it ends on a non-empty dish it will repeat.
    pub fn move_piece_avalache(&mut self, mut side: Side, mut index: usize, bank_collector: BankCollector) -> MoveResult {
        loop {
            let current_move = self.move_from_side(side, index, bank_collector);
            match current_move {
                MoveResult::IllegalMove | MoveResult::ExtraTurn => break current_move,
                MoveResult::Done(s, i) | MoveResult::Capture(s, i) => {
                    if self.side_to_dishes(s)[i] <= 1 {
                        break current_move
                    }
                    side = s;
                    index = i;
                },
            }
        }
    }

    /// Captures the selected and the opposing dish, and places them in the selected side
    pub fn capture_kalah(&mut self, side: Side, index: usize) {
        self.capture_from_side(side, index, side);
    }

    /// Collects all dishes that contain 2 or 3 pieces consecutively in a clockwise direction
    /// Returns the total amount of pieces collected
    pub fn oware_collect(&mut self, mut side: Side, mut index: usize, collector_side: Side) -> MUInt {
        let mut output = 0;
        loop {
            let dishes = self.side_to_dishes(side);
            if (2..=3).contains(&dishes[index]) {
                output += dishes[index];
                *self.side_bank(collector_side) += dishes[index];
                self.side_to_dishes_mut(side)[index] = 0;
                if index == 0 {
                    side = !side;
                    index = S - 1;
                } else {
                    index -= 1;
                }
            } else {
                break
            }
        }
        output
    }
}
