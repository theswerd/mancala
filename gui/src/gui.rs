use eframe::{egui::{CentralPanel, Context}, App, Frame, NativeOptions};
use mancala_board::{MancalaBoard, Side};

use crate::{MancalaGui, PlayerKind, State, APP_NAME, BOARD_SIZE};

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
                    side: Side::Left,
                    moves_history: vec![],
                })
            )),
        )
    }
}
