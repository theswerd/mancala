use eframe::egui::Ui;

use crate::MancalaGui;

impl MancalaGui {
    pub fn draw_move_history(&self, ui: &mut Ui) {
        for (side, index) in &self.moves_history {
            ui.label(format!("{side:?} {index}"));
        }
    }
}
