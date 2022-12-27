use gameloop::GameLoop;
use once_cell::sync::OnceCell;
use sdl2::event::Event;
use sdl2::image::*;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::{image, Sdl};
use std::collections::HashMap;
use std::time::Duration;

use rusty_glasses;

mod tile;
use tile::*;

mod gameloop;

pub fn main() {
    let glass = rusty_glasses::init("P4 - Minesweeper, Taylor Hiatt", 800, 600).unwrap();

    // Clear canvas beforehand
    glass.use_canvas(|canvas| {
        // Initial canvas clear
        canvas.set_draw_color(Color::RGB(21, 21, 21));
        canvas.clear();
        canvas.present();
    });

    // Game initialization
    let dims = Coords2d { x: 25, y: 16 };
    GameLoop::init(dims);

    // TODO: add lazy texture loading and reuse.
    'running: loop {
        let res = GameLoop::run_loop(&glass);
        res.expect("Game loop failed to run");
    }
}
