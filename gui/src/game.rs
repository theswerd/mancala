use std::{sync::Arc, thread::spawn};

use bank_bird::Algorithm;
use eframe::egui::{mutex::Mutex, Response};
use mancala_board::{MoveResult, Side};
use once_cell::sync::Lazy;

use crate::{BotState, MancalaGui, PlayerKind, State, BOARD_SIZE};

impl MancalaGui {
    pub fn handle_board(&mut self) {
        match self.state {
            State::Playing => {
                if self.board.game_over() {
                    self.board.collect_dishes();
                    self.state = State::Finished;
                }
            }
            _ => {}
        }
    }

    pub fn handle_player_button(&mut self, button: &Response, side: Side, index: usize) {
        let player = match self.side {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        };
        match player {
            PlayerKind::Human => {
                if button.clicked() {
                    let move_result = self.board.move_piece_kalah(side, index);
                    if !move_result.is_illegal() {
                        self.moves_history.push((side,index));
                        if let MoveResult::Capture(cs, ci) = move_result {
                            self.board.capture_from_side(cs, ci, side);
                        }
                        if move_result.change_side() {
                            self.side = !self.side;
                        }

                    }
                }
            }
            PlayerKind::Bot(algorithm) => {
                if self.board.game_over() { return }

                for bot_state in [&mut self.bot_state_left] {
                    match bot_state {
                        BotState::Nothing => {
                            let algorithm = algorithm.clone();
                            // find a way to run this in async 💀
                            // spawn(|| {
                                let index: usize = algorithm.lock().play_move(&self.board, self.side);
                                *bot_state = BotState::Play(index);
                                // algorithm
                            // });
                        }
                        BotState::Play(index) => {
                            let move_result = self.board.move_piece_kalah(self.side, *index);
                            if !move_result.is_illegal() {
                                self.moves_history.push((self.side,*index));
                                if let MoveResult::Capture(cs, ci) = move_result {
                                    self.board.capture_from_side(cs, ci, side);
                                }
                                if move_result.change_side() {
                                    self.side = !self.side;
                                }
                            }
                            *bot_state = BotState::Nothing;
                        }
                        BotState::Calculating => {}
                    };
                }
            }
        }
    }
}
