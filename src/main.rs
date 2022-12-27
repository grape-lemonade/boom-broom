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

pub fn render(mut canvas: &mut Canvas<sdl2::video::Window>, tex: &HashMap<&str, Texture>) {
    // let dims = BOARD.get().unwrap().get_dims();

    // for x in 0..dims.x {
    //     for y in 0..dims.y {
    //         BOARD
    //             .get()
    //             .unwrap()
    //             .get_tile(Coords2d::from_tuple((x, y)))
    //             .expect("Failed to get tile")
    //             .draw(canvas, &tex);
    //     }
    // }
}

pub fn update() {}

pub fn main() {
    let glass = rusty_glasses::init().unwrap();

    // Clear canvas beforehand
    glass.use_canvas(|canvas| {
        // Initial canvas clear
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();
    });

    // Game initialization
    let dims = Coords2d { x: 25, y: 16 };
    GameLoop::init(dims);

    // TODO: add lazy texture loading and reuse.
    'running: loop {
        // The rest of the game loop goes here...

        let events = glass.handle_events();

        for event in events {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    timestamp,
                    window_id,
                    keycode,
                    scancode,
                    keymod,
                    repeat,
                } => {
                    // Do key handling stuff here
                }
                Event::MouseButtonDown {
                    timestamp,
                    window_id,
                    which,
                    mouse_btn,
                    clicks,
                    x,
                    y,
                } => {
                    //Mouse click handling stuff here

                    // if (x / 32) < dims.x.into() && (y / 32) < dims.y.into() {
                    //     println!("Mouse Clicked at: {}, {}", x / 32, y / 32);
                    //     BOARD
                    //         .get()
                    //         .unwrap()
                    //         .get_tile(Coords2d::from_tuple(((x / 32).into(), (y / 32).into())))
                    //         .expect("Unable to get tile in click event")
                    //         .on_click(mouse_btn);
                    // }
                }
                _ => GameLoop::handle_event(event), //Eventually want all event handling here
            }

            GameLoop::try_update().expect("Failed to run update loop"); // Perform game updates
            GameLoop::try_render().expect("Failed to run render loop"); // Perform frame render

            //canvas.present(); // Move to be part of render loop
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // Enforce framerate
        }
    }
}
