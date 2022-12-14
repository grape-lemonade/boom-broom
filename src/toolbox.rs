use crate::gamestate::GameState;

pub struct Toolbox {
    pub game_state: GameState,
}

impl Toolbox {
    pub fn new() -> Toolbox {
        Toolbox {
            game_state: GameState::new(),
        }
    }
}
