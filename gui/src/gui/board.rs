use eframe::egui::{DragValue, Grid, Ui};
use mancala_board::Side;

use crate::gui::{util::empty_space, MancalaGui, PlayerKind};
use crate::{State, BOARD_SIZE};

#[derive(Debug, Clone, Copy)]
pub enum DisableButtonsMode {
    AutoDisable,
    Disable,
    Enable,
}

impl MancalaGui {
    pub fn draw_board(&mut self, ui: &mut Ui) {
        let disable_mode = match self.state {
            State::Editing => DisableButtonsMode::Enable,
            State::Playing => DisableButtonsMode::AutoDisable,
            State::Finished => DisableButtonsMode::Disable,
        };

        Grid::new("board")
        .min_col_width(3.0)
        .max_col_width(3.0)
        .show(ui, |ui| {
            self.draw_side(ui, disable_mode, Side::Right);

            ui.end_row();

            self.draw_banks(ui);

            ui.end_row();

            self.draw_side(ui, disable_mode, Side::Left);
        });
    }

    fn draw_banks(&mut self, ui: &mut Ui) {
        ui.label(self.board.right_bank.to_string());
        for _ in 0..BOARD_SIZE {
            empty_space(ui);
        }
        ui.label(self.board.left_bank.to_string());
    }

    fn draw_side(&mut self, ui: &mut Ui, disable_mode: DisableButtonsMode, side: Side) {
        let enabled = match disable_mode {
            DisableButtonsMode::Enable => true,
            DisableButtonsMode::AutoDisable => self.side == side && matches!(*self.get_player(self.side).lock(), PlayerKind::Human),
            DisableButtonsMode::Disable => false,
        };
        empty_space(ui);
        ui.add_enabled_ui(enabled, |ui: &mut Ui| {
            // no idea if this is the best solution 💀
            let range = (0..BOARD_SIZE).map(|i| match side {
                Side::Left => i,
                Side::Right => BOARD_SIZE - i - 1,
            });
            for i in range {
                let dish = &mut self.board.side_to_dishes_mut(side)[i];

                match self.state {
                    State::Editing => {
                        ui.add(DragValue::new(dish));
                    }
                    State::Playing | State::Finished => {
                        let button = ui.button(dish.to_string());
                        self.handle_player_button(&button, side, i);
                    }
                }

            }
        });
        empty_space(ui);
    }
}
