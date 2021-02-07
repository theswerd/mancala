use arr_macro::arr;

use mancala_board::MancalaBoard;
use rand::prelude::*;

fn main() {
    println!("Hello, world!");

    let board = MancalaBoard::default();

    //BOARD SIZE * OUTPUT SIZE = WEIGHTS FOR ONE LAYER
    //14 * 6 = 84
    
    let model = Model::new();

    println!("MODEL WEIGHTS: {:?}", model.weights);
    println!("CURRENT OUTPUT: {:?}",model.get_outputs(board))
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
        
        let mut outputs: [f64; 6]= [0f64; 6];
        outputs[0] = 3f64;

        for (position, amount) in board.values.iter().enumerate() {
            for i in 0..outputs.len() {

                println!("TESTING ITERATION: {}",position * outputs.len()+i);
            }
        }

        return outputs;
    }
}
