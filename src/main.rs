use once_cell::sync::OnceCell;
use sdl2::image::*;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::{Sdl, image};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, Texture};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

mod tile;
use tile::*;


pub fn render(mut canvas: &mut Canvas<sdl2::video::Window>, tex: Mutex<HashMap<&str, Texture>>) {
    let dims = BOARD.get().unwrap().get_dims();

    for x in 0..dims.x {
        for y in 0..dims.y {
            BOARD.get().unwrap().get_tile(Coords2d::from_tuple((x, y))).expect("Failed to get tile").draw(canvas, tex);
        }
    }
}

pub fn update() {

}

static BOARD: OnceCell<GameBoard> = OnceCell::new();

enum StaticTexture {
    Tile_Hidden,
    Tile_Revealed,
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let sdl_image = sdl2::image::init(InitFlag::PNG).unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("P4 - Minesweeper, Taylor Hiatt", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    
    // Clear canvas beforehand
    canvas.set_draw_color(Color::RGB(21, 21, 21));
    canvas.clear();
    canvas.present();

    // Game initialization
    let dims = Coords2d {x: 25, y: 16};
    BOARD.set(GameBoard::new(dims)).expect("Failed to create game board");

    let texture_creator = canvas.texture_creator();

    let mut tex: Mutex<HashMap<&str, Texture>> = Mutex::new(HashMap::new());

    //surface.from_fil
    tex.lock().unwrap().insert("tile_hidden", texture_creator.load_texture("./data/images/tile_hidden.png").unwrap());

    tex.lock().unwrap().insert("tile_revealed", texture_creator.load_texture("./data/images/tile_revealed.png").unwrap());
    tex.lock().unwrap().insert("number_1", texture_creator.load_texture("./data/images/number_1.png").unwrap());
    tex.lock().unwrap().insert("number_2", texture_creator.load_texture("./data/images/number_2.png").unwrap());
    tex.lock().unwrap().insert("number_3", texture_creator.load_texture("./data/images/number_3.png").unwrap());
    tex.lock().unwrap().insert("number_4", texture_creator.load_texture("./data/images/number_4.png").unwrap());
    tex.lock().unwrap().insert("number_5", texture_creator.load_texture("./data/images/number_5.png").unwrap());
    tex.lock().unwrap().insert("number_6", texture_creator.load_texture("./data/images/number_6.png").unwrap());
    tex.lock().unwrap().insert("number_7", texture_creator.load_texture("./data/images/number_7.png").unwrap());
    tex.lock().unwrap().insert("number_8", texture_creator.load_texture("./data/images/number_8.png").unwrap());
    tex.lock().unwrap().insert("flag", texture_creator.load_texture("./data/images/flag.png").unwrap());


    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { timestamp, window_id, keycode, scancode, keymod, repeat } => {
                    // Do key handling stuff here
                },
                Event::MouseButtonDown { timestamp, window_id, which, mouse_btn, clicks, x, y } => {
                    //Mouse click handling stuff here
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        update();                   // Perform game updates
        render(&mut canvas, tex);    // Actually render game

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // Enforce framerate
    }
}