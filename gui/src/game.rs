use std::{sync::Arc, thread::spawn};

use eframe::egui::Response;
use mancala_board::{MoveResult, Side};

use crate::{BotState, MancalaGui, PlayerKind, State};

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
        let player = self.get_player(self.side);
        match &mut *player.lock() {
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
            PlayerKind::Bot(algorithm, bot_state) => {
                if self.board.game_over() { return }

                match bot_state {
                    BotState::Ready => {
                        // let algorithm = algorithm.clone();
                        // find a way to run this in async 💀
                        // FOUNT IT!
                        let mut algorithm = algorithm.dyn_clone();
                        let player = Arc::clone(&player);
                        let board = self.board;
                        let side = self.side;

                        let joinhandle = spawn(move || {
                            let index: usize = algorithm.play_move(&board, side);
                            *player.clone().lock() = PlayerKind::Bot(algorithm, BotState::Play(index));
                            index
                        });
                        *bot_state = BotState::Calculating(joinhandle);
                    }
                    BotState::Calculating(_joinhandle) => {
                        // if joinhandle.is_finished() {
                        //     let index = std::mem::replace(joinhandle, spawn(||0)).join().unwrap();
                            
                        // }
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
                        *bot_state = BotState::Ready;
                    }
                };
            }
        };
    }
}
