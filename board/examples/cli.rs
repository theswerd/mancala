use std::io::stdin;

use mancala_board::{MancalaBoard, MoveResult, Side};

enum Variant {
    Kalah,
    Oware,
}

fn main() {
    let stdin = stdin();
    let mut board = MancalaBoard::<6>::new(4);
    let mut side = Side::Left;

    let variant = loop {
        println!("what variant? kalah / oware");
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        match buffer.to_lowercase().trim() {
            "kalah" => break Variant::Kalah,
            "oware" => break Variant::Oware,
            _ => {},
        }
    };

    while !board.game_over() {
        board.print_horizontal();
        println!("Current turn: {side:?}");

        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();

        if let Ok(i) = buffer.trim().parse::<usize>() {
            if i == 0 || i > 6 { continue }

            let result_move = match variant {
                Variant::Kalah => board.move_piece_kalah(side, i - 1),
                Variant::Oware => board.move_piece_oware(side, i - 1),
            };
            match result_move {
                MoveResult::Done(s, i) => {
                    if let Variant::Oware = variant {
                        board.oware_collect(s, i, side);
                    }
                },
                MoveResult::Capture(s, i) => {
                    if let Variant::Kalah = variant {
                        board.capture_kalah(s, i);
                    }
                },
                MoveResult::IllegalMove|MoveResult::ExtraTurn => continue,
            }
            side = !side;
        }
    }

    board.print_vertical();

    println!("Winner: {:?}", board.winner());
}
