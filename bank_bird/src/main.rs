mod plot;

use std::time::{Duration, Instant};

use bank_bird::Algorithm;
use mancala_board::{MancalaBoard, Side, MoveResult, Winner};

#[derive(Default, Clone, Copy, derive_more::Add, derive_more::Sum)]
struct GamesResults {
    wins: usize,
    draws: usize,
    losses: usize,
    games: usize,
    left_time: Duration,
    right_time: Duration,
}

#[allow(dead_code)]
pub struct AlgorithmBattle<const S: usize> {
    first: Box<dyn Algorithm<S>>,
    second: Box<dyn Algorithm<S>>,
    results: GamesResults,
}

macro_rules! algs {
    ($($m:ident::$al:ident $b:tt,)*) => {
        vec![
            $(
                Box::new(bank_bird::$m::$al $b),
            )*
        ]
    };
}

fn main() {
    let algorithms: Vec<Box<dyn Algorithm<6>>> = algs![
        deterministic::First(),
        deterministic::Last(),
        deterministic::Smallest(false),
        deterministic::Smallest(true),
        deterministic::Biggest(false),
        deterministic::Biggest(true),
        deterministic::Pi(0),
        mix::CaptureAndExtraTurn(),
        // random::Random(),

        bank_bird::BankBird1(8),
        bank_bird::BankBird2(8),
        bank_bird::BankBird2(10),
    ];

    // (win, losses, tie)
    let mut results = vec![];

    // Box<dyn> can't be shared between threads safely :/
    for (f, first) in algorithms.iter().enumerate() {
        for (s, second) in algorithms.iter().enumerate() {
            if f == s {
                // let's ignore the games where an algorithm plays against itself 💀
                results.push(
                    AlgorithmBattle {
                        first: first.dyn_clone(),
                        second: second.dyn_clone(),
                        results: Default::default(),
                    }
                );
                continue;
            }

            let min_games = first.dyn_clone().min_games().max(second.min_games());

            let mut first = first.dyn_clone();
            let mut second = second.dyn_clone();

            let mut games_results = vec![];

            for _i in 0..min_games {
                let mut side = Side::Left;
                
                let mut wins = 0;
                let mut draws = 0;
                let mut losses = 0;
                let mut games = 0;
                let mut left_time = Duration::ZERO;
                let mut right_time = Duration::ZERO;

                for initial_board in INITIAL_BOARDS {
                    let mut board = MancalaBoard::from_side(*initial_board);

                    while !board.game_over() {
                        let current_turn = match side {
                            Side::Left => &mut first,
                            Side::Right => &mut second,
                        };

                        loop {
                            if board.game_over() { break }

                            let start_time = Instant::now();
                            
                            let move_index = current_turn.play_move(&board, side);
                            
                            let duration = start_time.elapsed();
                            match side {
                                Side::Left => left_time += duration,
                                Side::Right => right_time += duration,
                            }

                            let move_result = board.move_piece_kalah(side, move_index);

                            match move_result {
                                MoveResult::Capture(_, index) => { board.capture_kalah(side, index); break }
                                MoveResult::Done(_, _) => { break }
                                MoveResult::ExtraTurn => { continue }
                                MoveResult::IllegalMove => {
                                    println!(
                                        "{:?} {} {:?} {}",
                                        board.left,
                                        board.left_bank,
                                        board.right,
                                        board.right_bank,
                                    );

                                    panic!(
                                        "FOUL: {} [{}]",
                                        current_turn.name(),
                                        move_index,
                                    )
                                }
                            }
                        }

                        side = match side {
                            Side::Left => Side::Right,
                            Side::Right => Side::Left,
                        }
                    }

                    board.collect_dishes();

                    match board.winner() {
                        Winner::Side(side) => {
                            match side {
                                Side::Left => wins += 1,
                                Side::Right => losses += 1,
                            }
                        }
                        Winner::Tie => draws += 1,
                    }
                    games += 1;
                }

                games_results.push(GamesResults {
                    wins,
                    draws,
                    losses,
                    games,
                    left_time,
                    right_time,
                });
            }

            let games_result = games_results.into_iter().sum();

            results.push(AlgorithmBattle {
                first: first.dyn_clone(),
                second: second.dyn_clone(),
                results: games_result,
            })
        }
    }


    plot::draw_colors(results, algorithms.len());
}

const INITIAL_BOARDS: &[[MUInt; 6]] = &[
    [1; 6],
    [2; 6],
    [3; 6],
    [4; 6],
    [5; 6],
    [6; 6],
    [7; 6],
    [8; 6],
    [9; 6],
    [10; 6],
    
    [1,2,3,4,5,6],
    [6,5,4,3,2,1],
    
    [1,5,3,4,2,6],
    [6,2,4,3,5,1],

    [1,1,1,2,2,2],
    [2,2,2,3,3,3],
    [3,3,3,4,4,4],

    [1,1,1,1,1,19],
    [19,1,1,1,1,1],

    [4,1,3,1,2,1],

    [8,8,8,0,0,0],
    [6,6,6,6,0,0],
];
