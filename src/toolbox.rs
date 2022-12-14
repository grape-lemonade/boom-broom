use crate::gamestate::GameState;
use std::sync::mpsc;

pub struct Toolbox {
    pub game_state: GameState,
    sender: mpsc::Sender<EventCode>,
}

pub enum EventCode {
    EXAMPLE,
}

impl Toolbox {
    pub fn new(tx: mpsc::Sender<EventCode>) -> Toolbox {
        Toolbox {
            game_state: GameState::new(),
            sender: tx,
        }
    }
}
