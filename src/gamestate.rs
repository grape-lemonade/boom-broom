use crate::tile::Tile;
use std::collections::HashMap;

// Game State
#[derive(Debug)]
pub struct GameState {
    play_status: PlayStatus,
    mine_count: u32,
    dims: (u32, u32),
    tiles: HashMap<(u32, u32), Tile>,
}

impl GameState {
    pub fn get_flag_count(&self) -> u32 {
        todo!();
    }

    pub fn get_mine_count(&self) -> u32 {
        self.mine_count
    }

    pub fn get_tile(&self, x: u32, y: u32) -> Option<&Tile> {
        self.tiles.get(&(x, y))
    }

    pub fn get_play_status(&self) -> &PlayStatus {
        &self.play_status
    }
    pub fn set_play_status(mut self, new: PlayStatus) {
        self.play_status = new;
    }

    pub fn new(dims: Option<(u32, u32)>, num_of_mines: Option<u32>) -> GameState {
        let mut new = GameState {
            play_status: PlayStatus::PLAYING,
            mine_count: num_of_mines.unwrap_or_else(|| 50),
            dims: dims.unwrap_or_else(|| (25, 16)),
            tiles: HashMap::new(),
        };

        for pos_x in (0..new.dims.0) {
            for pos_y in (0..new.dims.1) {
                new.tiles.insert((pos_x, pos_y), Tile::new((pos_x, pos_y)));
            }
        }

        new
    }
}
#[derive(Debug)]
pub enum PlayStatus {
    WIN,
    LOSS,
    PLAYING,
}
