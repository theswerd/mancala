use arr_macro::arr;

use mancala_board::MancalaBoard;
use rand::prelude::*;

fn main() {
    //BOARD SIZE * OUTPUT SIZE = WEIGHTS FOR ONE LAYER
    //14 * 6 = 84

    println!("MODEL WEIGHTS: {:?}", model.weights);
    println!("CURRENT OUTPUT: {:?}", model.get_outputs(board));

    let mut model1 = Model::new();
    let mut model2 = Model::new();
    let mut rng = rand::thread_rng();

    for round in 0..10000 {
        let model1IsLeft: bool = rng.gen();
        let mut model1Side: Side;
        if (model1isLeft) {
            model1Side = Side::Left;
        } else {
            model1Side = Side::Right;
        }
        let board = MancalaBoard::default();

        loop {}
    }
}

struct Model {
    weights: [f64; 84],
}

impl Model {
    pub fn new() -> Model {
        let mut rng = rand::thread_rng();

        return Model {
            weights: arr![rng.gen(); 84],
        };
    }

    pub fn from_values(weights: [f64; 84]) -> Model {
        return Model { weights: weights };
    }

    pub fn outputs(&self, board: MancalaBoard) -> [f64; 6] {
        board.print();

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

    pub fn best_move(&self, board: MancalaBoard) -> usize {
        // TODO: OPTIMIZE
        let mut max_index: usize = 0;
        let mut max_value: f64 = 0;
        for (index, value) in self.outputs().iter.enumerate() {
            if (value > max_value) {
                max_index = index;
                max_value = value;
            }
        }
        return value;
    }
}
