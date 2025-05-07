use std::sync::Arc;

use eframe::egui::{mutex::Mutex, Rect, Response, Sense, Ui, Vec2};
use mancala_board::Side;

use crate::{/*BotState,*/ MancalaGui, PlayerKind, LEFT_PLAYER, RIGHT_PLAYER};

pub fn empty_space(ui: &mut Ui) -> (Rect, Response) {
    ui.allocate_at_least(Vec2::ZERO, Sense { click: false, drag: false, focusable: false })
}

impl MancalaGui {
    pub fn get_player(&mut self, side: Side) -> Arc<Mutex<PlayerKind>> {
        match side {
            Side::Left => LEFT_PLAYER.clone(),
            Side::Right => RIGHT_PLAYER.clone(),
        }
    }

    // pub fn request_repaint(&mut self, ui: &mut Ui) {
        // if self.bot_state != BotState::Nothing {
        //     ui.ctx().request_repaint();
        // }
    // }
}
