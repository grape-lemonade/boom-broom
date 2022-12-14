use crate::tile::Tile;

// Game State
pub struct GameState {
    play_status: PlayStatus,

    tiles: Vec<Vec<Tile>>,
}

impl GameState {
    pub fn get_flag_count(&self) -> u32 {
        todo!();
    }

    pub fn get_mine_count(&self) -> u32 {
        todo!();
    }

    pub fn get_tile(&self, x: u32, y: u32) -> Option<&Tile> {
        todo!();
    }

    pub fn get_play_status(&self) -> &PlayStatus {
        &self.play_status
    }
    pub fn set_play_status(mut self, new: PlayStatus) {
        self.play_status = new;
    }

    pub fn new() -> GameState {
        todo!();
    }
}

pub enum PlayStatus {
    WIN,
    LOSS,
    PLAYING,
}
