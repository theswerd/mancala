use arr_macro::arr;

use mancala_board::{MancalaBoard, Side};
use rand::prelude::*;

fn main() {
    //BOARD SIZE * OUTPUT SIZE = WEIGHTS FOR ONE LAYER
    //14 * 6 = 84
    let mut rng = rand::thread_rng();

    let mut model1 = Model::new(&mut rng);

    let mut model2 = Model::new(&mut rng);

    for round in 0..10000 {
        {
            println!("ROUND: {}", round);
            let model1is_first: bool = rng.gen();
            // let mut model1Side: Side;
            // if model1isFirst {
            //     model1Side = Side::Left;
            // } else {
            //     model1Side = Side::Right;
            // }

            let mut board = MancalaBoard::default();
            let mut iteration = 0;
            loop {
                iteration += 1;
                println!("MOVE: {}", iteration);

                if model1is_first {
                    let model1_best_move = model1.best_move(&board);

                    if !board.is_move_legal(model1_best_move + 1) {
                        model1 = Model::new(&mut rng);
                        println!("LOOP NUMBER: {}", iteration);
                        println!("FOUL: MODEL 1");
                        break;
                    }
                    println!("BEFORE SIDE");

                    board.move_from_side(model1_best_move, Side::Left);
                    println!("AFTER SIDE SIDE");

                    if board.game_over() {
                        if board.winning() == Side::Left {
                            model2 = Model::new(&mut rng);
                            println!("WINNER: MODEL 2");
                        } else {
                            model1 = Model::new(&mut rng);
                            println!("WINNER: MODEL 1");
                        }
                        break;
                    }
                    board.flip();

                    let model2_best_move = model2.best_move(&board);
                    if !board.is_move_legal(model2_best_move + 1) {
                        model2 = Model::new(&mut rng);
                        println!("LOOP NUMBER: {}", iteration);
                        println!("FOUL: MODEL 2");

                        break;
                    }
                    board.move_from_side(model2_best_move, Side::Left);
                    if board.game_over() {
                        if board.winning() == Side::Left {
                            model2 = Model::new(&mut rng);
                            println!("WINNER: MODEL 2");
                        } else {
                            model1 = Model::new(&mut rng);
                            println!("WINNER: MODEL 1");
                        }
                        break;
                    }
                    board.flip();
                } else {
                    let model2_best_move = model2.best_move(&board);
                    if !board.is_move_legal(model2_best_move + 1) {
                        model2 = Model::new(&mut rng);
                        println!("LOOP NUMBER: {}", iteration);
                        println!("FOUL: MODEL 2");
                        break;
                    }
                    board.move_from_side(model2_best_move, Side::Left);
                    if board.game_over() {
                        if board.winning() == Side::Left {
                            model2 = Model::new(&mut rng);
                            println!("WINNER: MODEL 2");
                        } else {
                            model1 = Model::new(&mut rng);
                            println!("WINNER: MODEL 1");
                        }
                        break;
                    }
                    board.flip();

                    let model1_best_move = model1.best_move(&board);
                    if !board.is_move_legal(model1_best_move + 1) {
                        model1 = Model::new(&mut rng);
                        println!("LOOP NUMBER: {}", iteration);

                        println!("FOUL: MODEL 1");

                        break;
                    }
                    board.move_from_side(model1_best_move, Side::Left);
                    if board.game_over() {
                        if board.winning() == Side::Left {
                            model2 = Model::new(&mut rng);
                        } else {
                            model1 = Model::new(&mut rng);
                            println!("WINNER: MODEL 1");
                        }
                        break;
                    }
                    board.flip();
                }
            }
        }
    }

    println!("MODEL 1 WEIGHTS: {:?}", model1.weights);
    println!("MODEL 2 WEIGHTS: {:?}", model2.weights);
}

struct Model {
    weights: [f64; 84],
}

impl Model {
    pub fn new(rng: &mut ThreadRng) -> Model {
        return Model {
            weights: arr![rng.gen(); 84],
        };
    }

    pub fn from_values(weights: [f64; 84]) -> Model {
        return Model { weights: weights };
    }

    pub fn outputs(&self, board: &MancalaBoard) -> [f64; 6] {
        let mut outputs: [f64; 6] = [0f64; 6];

        for (position, amount) in board.values.iter().enumerate() {
            for i in 0..outputs.len() {
                let weight_index = position * outputs.len() + i;
                let weight = self.weights[weight_index];

                // WEIGHTED AVERAGE
                outputs[i] += (weight * *amount as f64 / board.values.len() as f64) as f64;
            }
        }

        return outputs;
    }

    pub fn best_move(&self, board: &MancalaBoard) -> usize {
        // TODO: OPTIMIZE
        let mut max_index: usize = 0;
        let mut max_value: f64 = 0f64;
        for (index, value) in self.outputs(board).iter().enumerate() {
            if value > &max_value {
                max_index = index;
                max_value = *value;
            }
        }
        return max_index;
    }
}
