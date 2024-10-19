use std::cmp;
use std::io;

// Mancala board representation
struct MancalaBoard {
    pits: usize,
    stones: usize,
    board: Vec<usize>,
}

impl MancalaBoard {
    fn new(pits: usize, stones: usize) -> MancalaBoard {
        let mut board = vec![stones; 2 * pits + 2];
        MancalaBoard { pits, stones, board }
    }

    fn make_move(&mut self, player: usize, pit: usize) -> usize {
        let mut stones = self.board[pit];
        self.board[pit] = 0;

        let mut pit = pit;

        while stones > 0 {
            pit = (pit + 1) % self.board.len();

            if (player == 1 && pit == self.pits) || (player == 2 && pit == 0) {
                continue;
            }

            self.board[pit] += 1;
            stones -= 1;
        }

        if (player == 1 && pit == 0) || (player == 2 && pit == self.pits) {
            1 // Extra turn
        } else {
            0 // Switch players
        }
    }

    fn is_game_over(&self) -> bool {
        self.board[1..self.pits].iter().all(|&stones| stones == 0)
            && self.board[self.pits + 2..2 * self.pits + 1].iter().all(|&stones| stones == 0)
    }

    fn get_score(&self) -> isize {
        (self.board[self.pits] as isize) - (self.board[0] as isize)
    }
}

fn minimax(board: MancalaBoard, player: usize, depth: usize, alpha: isize, beta: isize) -> isize {
    if depth == 0 || board.is_game_over() {
        return board.get_score();
    }

    if player == 1 {
        let mut max_eval = isize::min_value();
        for pit in 1..=board.pits {
            if board.board[pit] == 0 {
                continue;
            }
            let mut copy_board = board.clone();
            let extra_turn = copy_board.make_move(player, pit);
            let eval = minimax(copy_board, 3 - player, depth - 1, alpha, beta);
            max_eval = cmp::max(max_eval, eval);
            let alpha = cmp::max(alpha, eval);
            if beta <= alpha {
                break;
            }
        }
        max_eval
    } else {
        let mut min_eval = isize::max_value();
        for pit in (board.pits + 2)..(2 * board.pits + 2) {
            if board.board[pit] == 0 {
                continue;
            }
            let mut copy_board = board.clone();
            let extra_turn = copy_board.make_move(player, pit);
            let eval = minimax(copy_board, 3 - player, depth - 1, alpha, beta);
            min_eval = cmp::min(min_eval, eval);
            let beta = cmp::min(beta, eval);
            if beta <= alpha {
                break;
            }
        }
        min_eval
    }
}

fn find_best_move(board: &MancalaBoard, player: usize, depth: usize) -> usize {
    let mut best_move = 0;
    let mut best_eval = if player == 1 { isize::min_value() } else { isize::max_value() };
    let mut alpha = isize::min_value();
    let mut beta = isize::max_value();

    for pit in 1..=board.pits {
        if board.board[pit] == 0 {
            continue;
        }
        let mut copy_board = board.clone();
        let extra_turn = copy_board.make_move(player, pit);
        let eval = minimax(copy_board, 3 - player, depth - 1, alpha, beta);
        if (player == 1 && eval > best_eval) || (player == 2 && eval < best_eval) {
            best_eval = eval;
            best_move = pit;
        }
        if player == 1 {
            alpha = cmp::max(alpha, eval);
        } else {
            beta = cmp::min(beta, eval);
        }
        if beta <= alpha {
            break;
        }
    }

    best_move
}
/*
fn main() {
    let pits = 6;
    let stones = 4;
    let depth = 3; // Adjust the search depth for desired AI strength
    let mut board = MancalaBoard::new(pits, stones);

    while !board.is_game_over() {
        println!("Current board:");
        for i in 1..=pits {
            print!("{:2} ", board.board[pits + i]);
        }
        println!();
        for i in (pits + 2)..=(2 * pits + 1) {
            print!("{:2} ", board.board[i]);
        }
        println!();

        if board.pits == board.board.len() / 2 - 2 {
            println!("Player 1's turn");
        } else {
            println!("Player 2's turn");
        }

        let move = find_best_move(&board, 1, depth);
        println!("Chose pit {}", move);

        let extra_turn = board.make_move(1, move);
        if extra_turn == 0 {
            board.board[0] = 0; // Clear the Mancala of the other player
        }
    }

    println!("Game over! Final board:");
    for i in 1..=pits {
        print!("{:2} ", board.board[pits + i]);
    }
    println!();
    for i in (pits + 2)..=(2 * pits + 1) {
        print!("{:2} ", board.board[i]);
    }
    println!();

    let score = board.get_score();
    if score > 0 {
        println!("Player 1 wins!");
    } else if score < 0 {
        println!("Player 2 wins!");
    } else {
        println!("It's a tie!");
    }
}
*/