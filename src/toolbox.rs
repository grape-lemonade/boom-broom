use crate::gamestate::GameState;
use std::sync::mpsc;

pub struct Toolbox {
    pub game_state: GameState,
    sender: mpsc::Sender<u32>,
}

impl Toolbox {
    pub fn new(tx: mpsc::Sender<u32>) -> Toolbox {
        Toolbox {
            game_state: GameState::new(),
            sender: tx,
        }
    }
}
