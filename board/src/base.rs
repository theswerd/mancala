use crate::{BankCollector, MUInt, MancalaBoard, MoveResult, Side};

impl<const S: usize> MancalaBoard<S> {
    /// Allows you to move an arbitrary dish on the board while it getting collected by an arbitrary player.
    /// Optimal for avalache and puzzle implementations.
    pub fn move_from_side(&mut self, dish_side: Side, dish_index: usize, collector_side: impl Into<BankCollector>) -> MoveResult {
        if !self.is_move_legal(dish_side, dish_index) {
            return MoveResult::IllegalMove
        }

        let collector_side = collector_side.into();
        let placeable_slots = (2 * S + collector_side.quantity()) as MUInt;

        let initial_hand = self.clear_dish(dish_side, dish_index);
        let mut hand = initial_hand;
        let mut current_index = dish_index + 1;
        let mut current_side = dish_side;
        let mut index = 0;

        loop {
            if current_index >= S {
                if is_current_collecting_side(current_side, collector_side) {
                    let bank = self.side_bank(current_side);
                    let bulk = calculate_bulk(&mut index, initial_hand, placeable_slots);
                    *bank += bulk;
                    hand -= bulk;
                    if hand == 0 {
                        break MoveResult::ExtraTurn;
                    }
                }
                current_side = !current_side;
                current_index = 0;
            } else {
                let dishes = self.side_to_dishes_mut(current_side);
                let bulk = calculate_bulk(&mut index, initial_hand, placeable_slots);
                dishes[current_index] += bulk;
                hand -= bulk;
                if hand == 0 {
                    if
                        is_current_collecting_side(current_side, collector_side) && // if it's your side
                        dishes[current_index] == 1 && // if it was previously empty
                        self.side_to_dishes(match collector_side {
                            BankCollector::Side(side) => !side, // 💀 found the bug
                            BankCollector::Both => !current_side,
                            _ => unreachable!(),
                        })[self.opposite_dish_index(current_index)] > 0 // if the other side has something in it
                    {
                        break MoveResult::Capture(current_side, current_index)
                    }
                }
                current_index += 1;
            }

            if hand == 0 {
                break MoveResult::Done(current_side, current_index - 1); // underflow is unreachable
            }
        }
    }

    /// Allows you to capture a specific dish while it getting collected into an arbitrary side's bank
    pub fn capture_from_side(&mut self, dish_side: Side, dish_index: usize, collector_side: Side) {
        let other_side_index = self.opposite_dish_index(dish_index);
        let current_side = self.clear_dish(dish_side, dish_index);
        let other_side = self.clear_dish(!dish_side, other_side_index);
        let bank = self.side_bank(collector_side);
        *bank += current_side + other_side;
    }
}

#[inline]
fn is_current_collecting_side(current_side: Side, collector_side: BankCollector) -> bool {
    match collector_side {
        BankCollector::None => false,
        BankCollector::Side(side) => current_side == side,
        BankCollector::Both => true,
    }
}

#[inline]
fn calculate_bulk(index: &mut MUInt, initial_hand: MUInt, placeable_slots: MUInt) -> MUInt {
    *index += 1;
    (initial_hand + (placeable_slots - *index)) / placeable_slots
}
