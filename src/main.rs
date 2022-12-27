use gameloop::GameLoop;
use rusty_glasses;
use sdl2::pixels::Color;

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
    // TODO: reorganize the gameloop and tile modules for easier development
    'running: loop {
        let res = GameLoop::run_loop(&glass);
        res.expect("Game loop failed to run");
    }
}
