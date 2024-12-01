use eframe::egui::{Rect, Response, Sense, Ui, Vec2};
use mancala_board::Side;

use crate::{/*BotState,*/ MancalaGui, PlayerKind};

pub fn empty_space(ui: &mut Ui) -> (Rect, Response) {
    ui.allocate_at_least(Vec2::ZERO, Sense { click: false, drag: false, focusable: false })
}

impl MancalaGui {
    pub fn get_player(&mut self, side: Side) -> &mut PlayerKind {
        match side {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        }
    }

    pub fn request_repaint(&mut self, ui: &mut Ui) {
        // if self.bot_state != BotState::Nothing {
        //     ui.ctx().request_repaint();
        // }
    }
}
