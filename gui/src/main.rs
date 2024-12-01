use std::sync::Arc;

use bank_bird::Algorithm;
use eframe::{egui::mutex::Mutex, HardwareAcceleration, NativeOptions};
use mancala_board::{MancalaBoard, Side};

mod game;
mod gui;

pub const APP_NAME: &str = "Mancala GUI";
pub const BOARD_SIZE: usize = 6;

fn main() -> Result<(), eframe::Error> {
    let board = MancalaBoard::<6>::new(4);

    let options = NativeOptions {
        hardware_acceleration: HardwareAcceleration::Preferred,
        vsync: true,
        persist_window: true,

        ..Default::default()
    };

    MancalaGui::run(board, options)
}

pub struct MancalaGui {
    board: MancalaBoard<BOARD_SIZE>,
    state: State,
    left: PlayerKind,
    right: PlayerKind,
    side: Side,
    moves_history: Vec<(Side, usize)>,
    bot_state_left: BotState,
    bot_state_right: BotState,
}

pub enum PlayerKind {
    Human,
    Bot(Arc<Mutex<Box<dyn bank_bird::Algorithm<BOARD_SIZE> + Send>>>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Playing,
    Editing,
    Finished,
}

pub enum BotState {
    Nothing,
    Calculating,
    Play(usize),
}
