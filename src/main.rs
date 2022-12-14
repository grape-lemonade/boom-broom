mod gamestate;
mod tile;
mod toolbox;

use crate::gamestate::*;
use crate::toolbox::Toolbox;
use once_cell::sync::{Lazy, OnceCell};
use std::sync::mpsc;

const TOOLBOX: OnceCell<Toolbox> = OnceCell::new();

enum EventCode {
    EXAMPLE,
}

fn launch() {
    // Launches the game, initializing the window and loading the sprites,
    let (tx, rx): (mpsc::Sender<EventCode>, mpsc::Receiver<EventCode>) = mpsc::channel();
    TOOLBOX.set(Toolbox::new(tx.clone()));
    game_loop(rx);
}

fn game_loop(rec: mpsc::Receiver<EventCode>) {
    // The actual main game loop

    let mut res = true;

    while res {
        // debug ways of dropping out of game loop, remove later
        match TOOLBOX.get().unwrap().game_state.get_play_status() {
            PlayStatus::WIN => res = false,
            PlayStatus::PLAYING => res = true,
            PlayStatus::LOSS => res = false,
        }

        // Game loop stuff here

        // Pre-update

        update();

        // Post-update, pre-render

        render();

        // Post-frame
        todo!();
    }
}

fn update() {
    // The game loop, should process all computation needed between frames
}

fn render() {
    // Draw all game objects based on GameState, run after update()
}

fn main() {
    println!("Initializing...");
    launch();
}
