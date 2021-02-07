#[deprecated(
    since = "0.1.2",
    note = "Please use the MancalaBoard::default() instead"
)]
pub fn basic_board() -> MancalaBoard {
    return MancalaBoard {
        values: [0, 4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4],
    };
}

#[derive(PartialEq)]
pub enum Side {
    Left,
    Right,
}

pub struct MancalaBoard {
    pub values: [u32; 14],
}

impl MancalaBoard {
    pub fn default() -> MancalaBoard {
        return MancalaBoard {
            values: [0, 4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4],
        };
    }

    pub fn from_position(values: [u32; 14]) -> MancalaBoard {
        return MancalaBoard { values: values };
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
        let left = &self.values[0..7];
        let right = &self.values[7..14];
        for (index, value) in [right, left].concat().iter().enumerate() {
            self.values[index] = *value;
        }
    }

    pub fn move_from_side(&mut self, index: usize, side: Side) -> bool {
        let mut addition: usize = 1;
        if side == Side::Right {
            addition = 8;
        }
        return self.move_piece(index + addition, side);
    }

    pub fn move_piece(&mut self, index: usize, side: Side) -> bool {
        let mut amount = self.values[index];

        self.values[index] = 0;
        let mut add_index = index + 1;

        return loop {
            while add_index < (index + amount as usize + 1) {
                let formatted_index = add_index - 14 * ((add_index / 14) as f32).ceil() as usize;
                if formatted_index == 0 && side == Side::Left
                    || formatted_index == 7 && side == Side::Right
                {
                    amount += 1;
                } else {
                    self.values[formatted_index] += 1;
                }
                add_index += 1;
            }
            let end_index = get_end_index(index, amount);
            if end_index == 0 && end_index == 7 {
                break true;
            } else if self.values[end_index] > 1 {
                add_index = end_index;
            } else {
                break false;
            }
        }
       
    }

    pub fn game_over(&self) -> bool {
        return self.values[1..7].iter().all(|&item| item == 0)
            || self.values[8..14].iter().all(|&item| item == 0);
    }
    pub fn winning(&self) -> Side {
        if self.values[0] > self.values[7] {
            return Side::Right;
        } else {
            return Side::Left;
        }
    }

    pub fn is_move_legal(&self, index: usize) -> bool {
        return index != 7 && index != 0 && self.values[index] != 0;
    }
}

fn get_end_index(start_index: usize, amount: u32) -> usize {
    let mut total = start_index + amount as usize;
    while total > 13 {
        total -= 14
    }

    return total;
}
