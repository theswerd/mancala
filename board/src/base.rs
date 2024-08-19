use crate::{MancalaBoard, MoveResult, Side, BankCollector};

impl<const S: usize> MancalaBoard<S> {
    /// Allows you to move an arbitrary dish on the board while it getting collected by an arbitrary player.
    /// Optimal for avalache and puzzle implementations.
    pub fn move_from_side(&mut self, dish_side: Side, dish_index: usize, collector_side: BankCollector) -> MoveResult {
        if !self.is_move_legal(dish_side, dish_index) {
            return MoveResult::IllegalMove
        }

        let placeable_slots = 2 * S + collector_side.quantity();

        let mut hand = self.clear_dish(dish_side, dish_index);
        let mut current_index = dish_index + 1;
        let mut current_side = dish_side;

        macro_rules! calculate_bulk {
            () => {
                // // THERE'S SOME BUG WITH THIS
                // if hand >= placeable_slots { // only run division if you're sure that there's more than 1 value
                //     hand / placeable_slots
                // } else {
                    1
                // }
            };
        }

        macro_rules! check_current_side {
            () => {
                match collector_side {
                    BankCollector::None => false,
                    BankCollector::Side(side) => current_side == side,
                    BankCollector::Both => true,
                }
            };
        }

        macro_rules! check_bank {
            ($bank:expr) => {
                {
                    if check_current_side!() {
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
                            check_current_side!() && // if it's your side
                            $dishes[current_index] == 1 && // if it was previously empty
                            self.side_to_dishes(match collector_side {
                                BankCollector::Side(side) => side,
                                _ => unreachable!(),
                            })[self.opposite_dish_index(current_index)] > 0 // if the other side has something in it
                        {
                            break MoveResult::Capture(current_side, current_index)
                        }
                    }
                    current_index += 1;
                }
            };
        }

        loop {
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

    /// Allows you to capture a specific dish while it getting collected into an arbitrary side's bank
    pub fn capture_from_side(&mut self, dish_side: Side, dish_index: usize, collector_side: Side) {
        let other_side_index = self.opposite_dish_index(dish_index);
        let current_side = self.clear_dish(dish_side, dish_index);
        let other_side = self.clear_dish(!dish_side, other_side_index);
        let bank = self.side_bank(collector_side);
        *bank += current_side + other_side;
    }
}
