use crate::gamestate::GameState;
use std::sync::mpsc;

#[derive(Debug)]
pub struct Toolbox {
    pub game_state: GameState,
    sender: mpsc::Sender<EventCode>,
}
#[derive(Debug)]
pub enum EventCode {
    EXAMPLE,
    EMPTY,
}

impl Toolbox {
    pub fn new(tx: mpsc::Sender<EventCode>) -> Toolbox {
        Toolbox {
            game_state: GameState::new(None, None),
            sender: tx,
        }
    }
}
