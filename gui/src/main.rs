#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::Arc, thread::JoinHandle};

use bank_bird::Algorithm;
use eframe::{egui::mutex::Mutex, HardwareAcceleration, NativeOptions};
use lazy_static::lazy_static;
use mancala_board::{MancalaBoard, Side};

mod game;
mod gui;

pub const APP_NAME: &str = "Mancala GUI";
pub const BOARD_SIZE: usize = 6;

fn main() -> Result<(), eframe::Error> {
    let board = MancalaBoard::<BOARD_SIZE>::new(4);

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
    side: Side,
    moves_history: Vec<(Side, usize)>,
}

lazy_static!{
    static ref LEFT_PLAYER: Arc<Mutex<PlayerKind>> = Arc::new(Mutex::new(PlayerKind::Human));
    static ref RIGHT_PLAYER: Arc<Mutex<PlayerKind>> = Arc::new(Mutex::new(PlayerKind::Bot(Box::new(bank_bird::bank_bird::BankBird2(18)), BotState::Ready)));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Playing,
    Editing,
    Finished,
}

pub enum PlayerKind {
    Human,
    Bot(Box<dyn Algorithm<BOARD_SIZE>>, BotState),
}

pub enum BotState {
    Ready,
    Calculating(JoinHandle<usize>),
    Play(usize),
}
