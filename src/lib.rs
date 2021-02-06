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
    values: [u32; 14],
}

impl MancalaBoard {
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

    pub fn move_piece(&mut self, index: usize, side: Side) -> bool {
        let mut amount = self.values[index];

        self.values[index] = 0;
        let mut add_index = index + 1;
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
        println!("END INDEX: {}", index);

        self.print();

        if end_index == 0 && end_index == 7 {
            return true;
        } else if self.values[end_index] > 1 {
            return self.move_piece(end_index, side);
        } else {
            return false;
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
}

fn get_end_index(start_index: usize, amount: u32) -> usize {
    let mut total = start_index + amount as usize;
    while total > 13 {
        total -= 14
    }

    return total;
}
