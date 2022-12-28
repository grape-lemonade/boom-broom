use game_state::GameState;
use rusty_glasses;
use sdl2::pixels::Color;

use std::fs;

mod game_board;
mod game_config;
mod game_state;
mod util;

pub fn main() {
    // TODO: load config
    let con = fs::read_to_string("config.ron").expect("Couldn't load config file.");
    let gs = GameState::init(ron::from_str(&con).expect("Couldn't serialize config"));

    //println!("{:?}", gs);

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
