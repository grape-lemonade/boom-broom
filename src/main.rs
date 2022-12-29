use game_board::tile::Tile;
use game_state::GameState;
use rusty_glasses;
use sdl2::pixels::Color;

use std::fs;

use crate::{game_board::tile::TileState, util::Vec2D};

mod game_board;
mod game_config;
mod game_state;
mod util;

pub fn main() {
    // Program ARGS:
    // boom-broom.exe <test brd file to run>

    //println!("{:?}", gs);

    let con = fs::read_to_string("config.ron").expect("Couldn't load config file.");
    let gs = GameState::init(ron::from_str(&con).expect("Couldn't serialize config"));

    loop {
        gs.update();
        gs.draw();
    }

    // TODO: add lazy texture loading and reuse.
    // TODO: reorganize the gameloop and tile modules for easier development
    // 'running: loop {
    //     let res = GameLoop::run_loop(&glass);
    //     res.expect("Game loop failed to run");
    // }
}
