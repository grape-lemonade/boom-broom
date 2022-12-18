mod gamestate;
mod tile;
mod toolbox;

use sfml::window::{self as sf};
use sf::{Window, Event, Style};
use crate::gamestate::*;
use crate::tile::{Tile, TileState};
use crate::toolbox::{EventCode, Toolbox};
use once_cell::sync::{Lazy, OnceCell};
use std::sync::mpsc;

fn launch() {
    // Launches the game, initializing the window and loading the sprites,
    let mut TOOLBOX = Toolbox::new();

    TOOLBOX.window.set_framerate_limit(60);

    while TOOLBOX.window.is_open() {
        TOOLBOX.window.set_active(true);
        TOOLBOX = TOOLBOX.GameLoop();
    }
}



fn main() {
    println!("Initializing...");
    launch();
}
