use crate::tile::Tile;
use rstar::{RTree, AABB};

// Game State
#[derive(Debug)]
pub struct GameState {
    play_status: PlayStatus,
    mine_count: i32,
    dims: (i32, i32),
    tiles: RTree<Tile>,
}

impl GameState {
    pub fn get_flag_count(&self) -> i32 {
        todo!();
    }

    pub fn get_mine_count(&self) -> i32 {
        self.mine_count
    }

    pub fn get_dims(&self) -> (i32, i32) {
        self.dims
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<&Tile> {
        self.tiles
            .locate_in_envelope(&AABB::from_point([x, y]))
            .next()
    }

    pub fn get_play_status(&self) -> &PlayStatus {
        &self.play_status
    }
    pub fn set_play_status(mut self, new: PlayStatus) {
        self.play_status = new;
    }

    pub fn new(dims: Option<(i32, i32)>, num_of_mines: Option<i32>) -> GameState {
        let mut new = GameState {
            play_status: PlayStatus::PLAYING,
            mine_count: num_of_mines.unwrap_or_else(|| 50),
            dims: dims.unwrap_or_else(|| (25, 16)),
            tiles: RTree::new(),
        };

        println!("generating tiles...");
        let mut temp: Vec<Tile> = Vec::new();
        for pos_x in (0..new.dims.0) {
            for pos_y in (0..new.dims.1) {
                temp.push(Tile::new((pos_x, pos_y)));
            }
        }
        println!("packing tree...");
        new.tiles = RTree::bulk_load(temp);
        println!("done loading rtree");

        //println!("{:?}", new);

        new
    }
}
#[derive(Debug)]
pub enum PlayStatus {
    WIN,
    LOSS,
    PLAYING,
}
