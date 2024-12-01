use std::sync::Arc;

use eframe::{egui::{mutex::Mutex, CentralPanel, Context}, App, Frame, NativeOptions};
use mancala_board::{MancalaBoard, Side};

use crate::{BotState, MancalaGui, PlayerKind, State, APP_NAME, BOARD_SIZE};

mod display;
mod board;
mod util;

impl App for MancalaGui {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.handle_board();

        CentralPanel::default()
        .show(ctx, |ui| {
            self.draw_board(ui);
            self.draw_move_history(ui);
        });
    }
}

impl MancalaGui {
    pub fn run(board: MancalaBoard<BOARD_SIZE>, options: NativeOptions) -> Result<(), eframe::Error> {
        eframe::run_native(
            APP_NAME,
            options,
            Box::new(|_cc| Ok(
                Box::new(Self {
                    board,
                    state: State::Playing,
                    left: PlayerKind::Human,
                    right: PlayerKind::Bot(
                        Arc::new(
                            Mutex::new(
                                Box::new(bank_bird::bank_bird::BankBird1(10))
                            )
                        )
                    ),
                    side: Side::Left,
                    moves_history: vec![],
                    bot_state_left: BotState::Nothing,
                    bot_state_right: BotState::Nothing,
                })
            )),
        )
    }
}
