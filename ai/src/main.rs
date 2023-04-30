use arr_macro::arr;

use mancala_board::{MancalaBoard, Side, Winner};
use rand::prelude::*;

fn main() {
    //BOARD SIZE * OUTPUT SIZE = WEIGHTS FOR ONE LAYER
    //14 * 6 = 84
    let mut rng = rand::thread_rng();

    let mut model1 = Model::new();
    let mut model2 = Model::new();

    for round in 0..10000 {
        println!("ROUND: {}", round);
        let model1is_first: bool = rng.gen();

        let ((vm1_num, vm1), (vm2_num, vm2)) = if model1is_first {
            ((1, &mut model1), (2, &mut model2))
        } else {
            ((2, &mut model2), (1, &mut model1))
        };

        let mut board = MancalaBoard::default();
        let mut iteration = 0;
        loop {
            iteration += 1;
            println!("MOVE: {}", iteration);

            let vm1_best_move = vm1.best_move(&board);

            if !board.is_move_legal(vm1_best_move + 1) {
                *vm1 = Model::from_rng(&mut rng);
                println!("LOOP NUMBER: {}", iteration);
                println!("FOUL: MODEL {}", vm1_num);
                break;
            }
            println!("BEFORE SIDE");

            board.move_from_side(vm1_best_move, Side::Left);
            println!("AFTER SIDE SIDE");

            if board.game_over() {
                if board.winner() == Winner::Side(Side::Left) {
                    *vm2 = Model::from_rng(&mut rng);
                    println!("WINNER: MODEL {}", vm2_num);
                } else {
                    *vm1 = Model::from_rng(&mut rng);
                    println!("WINNER: MODEL {}", vm1_num);
                }
                break;
            }
            board.flip();

            let vm2_best_move = vm2.best_move(&board);
            if !board.is_move_legal(vm2_best_move + 1) {
                *vm2 = Model::from_rng(&mut rng);
                println!("LOOP NUMBER: {}", iteration);
                println!("FOUL: MODEL {}", vm2_num);

                break;
            }
            board.move_from_side(vm2_best_move, Side::Left);
            if board.game_over() {
                if board.winner() == Winner::Side(Side::Left) {
                    *vm2 = Model::from_rng(&mut rng);
                    println!("WINNER: MODEL {}", vm2_num);
                } else {
                    *vm1 = Model::from_rng(&mut rng);
                    println!("WINNER: MODEL {}", vm1_num);
                }
                break;
            }
            board.flip();
        }
    }

    println!("MODEL 1 WEIGHTS: {:?}", model1.weights);
    println!("MODEL 2 WEIGHTS: {:?}", model2.weights);
}

struct Model {
    weights: [f64; 84],
}

impl Model {
    pub fn new() -> Model {
        Model {
            weights: arr![0.0; 84],
        }
    }

    pub fn from_rng(rng: &mut ThreadRng) -> Model {
        Model {
            weights: arr![rng.gen(); 84],
        }
    }

    pub fn from_values(weights: [f64; 84]) -> Model {
        Model { weights }
    }

    pub fn outputs(&self, board: &MancalaBoard) -> [f64; 6] {
        let mut outputs: [f64; 6] = [0f64; 6];

        for (position, &amount) in board.values.iter().enumerate() {
            for i in 0..outputs.len() {
                let weight_index = position * outputs.len() + i;
                let weight = self.weights[weight_index];

                // WEIGHTED AVERAGE
                outputs[i] += (weight * amount as f64 / board.values.len() as f64) as f64;
            }
        }

        outputs
    }

    pub fn best_move(&self, board: &MancalaBoard) -> usize {
        self.outputs(board).iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .unwrap()
            .0
    }
}
