use arr_macro::arr;

use mancala_board::MancalaBoard;
use rand::prelude::*;

fn main() {
    let board = MancalaBoard::default();

    //BOARD SIZE * OUTPUT SIZE = WEIGHTS FOR ONE LAYER
    //14 * 6 = 84

    let mut model1 = Model::new();
    let mut model2 = Model::new();

    println!("MODEL WEIGHTS: {:?}", model.weights);
    println!("CURRENT OUTPUT: {:?}", model.get_outputs(board))


    for i in 0..10000 {
        
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

    pub fn get_outputs(&self, board: MancalaBoard) -> [f64; 6] {
        board.print();

        let mut outputs: [f64; 6] = [0f64; 6];

        for (position, amount) in board.values.iter().enumerate() {
            for i in 0..outputs.len() {
                let weight_index = position * outputs.len() + i;
                let weight = self.weights[weight_index];

                // WEIGHTED AVERAGE
                outputs[i] += (weight * *amount as f64 / board.values.len()as f64) as f64;
            }
        }

        return outputs;
    }
}
