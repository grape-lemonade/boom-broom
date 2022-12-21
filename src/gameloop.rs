use std::{error::Error, fmt::Result, num::ParseFloatError, sync::RwLock};

use once_cell::sync::Lazy;
use sdl2::{event::Event, keyboard::Keycode};

use crate::tile::{Coords2d, GameBoard};

pub struct GameLoop {
    //Configs
    framerate: u8,
    board: Option<GameBoard>,
}

static LOOPER: RwLock<Lazy<GameLoop>> = RwLock::new(Lazy::new(|| GameLoop {
    framerate: 60,
    board: None,
}));

impl GameLoop {
    pub fn handle_event(event: Event) {}

    pub fn init(dims: Coords2d) {
        let mut tempLoop = LOOPER.try_write().expect("Couldnt get gameloop writer");
        tempLoop.board = Some(GameBoard::new(dims));
    }

    pub fn try_render() -> Result {
        Ok(())
    }
    pub fn try_update() -> Result {
        Ok(())
    }
}
