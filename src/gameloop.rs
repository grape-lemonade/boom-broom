use std::{error::Error, fmt::Result, num::ParseFloatError, sync::RwLock, time::Duration};

use once_cell::sync::Lazy;
use rusty_glasses::GlassContext;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

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

    fn handle_key(key: Event) {
        //todo
        match key {
            Event::KeyDown {
                timestamp: _,
                window_id: _,
                keycode: _,
                scancode: _,
                keymod: _,
                repeat: _,
            } => {}
            _ => (),
        }
    }

    fn handle_mouse(mouse: Event) {
        //Mouse click handling stuff here

        match mouse {
            Event::MouseButtonDown {
                timestamp: _,
                window_id: _,
                which: _,
                mouse_btn: _,
                clicks: _,
                x: _,
                y: _,
            } => {}
            _ => (),
        }

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

    pub fn run_loop(glass: &GlassContext) -> Result {
        // Stuff to be done in every loop no matter what

        for event in glass.handle_events() {
            match event {
                Event::KeyDown {
                    timestamp: _,
                    window_id: _,
                    keycode: _,
                    scancode: _,
                    keymod: _,
                    repeat: _,
                } => GameLoop::handle_key(event),
                Event::MouseButtonDown {
                    timestamp: _,
                    window_id: _,
                    which: _,
                    mouse_btn: _,
                    clicks: _,
                    x: _,
                    y: _,
                } => GameLoop::handle_mouse(event),
                _ => (), //handle remaining events,
            }
        }

        GameLoop::try_update().expect("Failed to run update loop"); // Perform game updates
        GameLoop::try_render(glass).expect("Failed to run render loop"); // Perform frame render

        //canvas.present(); // Move to be part of render loop
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // Enforce framerate
        Ok(())
    }

    pub fn try_render(glass: &GlassContext) -> Result {
        glass.use_canvas(|canvas| {
            // Clear canvas so no frame smear
            canvas.set_draw_color(Color::RGB(21, 21, 21));
            canvas.clear();

            canvas.set_draw_color(Color::RGB(0, 33, 255));
            canvas.draw_rect(Rect::new(0, 0, 5, 5));

            // Present canvas after render stage
            canvas.present();
        });
        Ok(())
    }
    pub fn try_update() -> Result {
        Ok(())
    }
}
