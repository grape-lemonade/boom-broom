mod gamestate;
mod tile;
mod toolbox;

use crate::gamestate::*;
use crate::toolbox::{EventCode, Toolbox};
use once_cell::sync::{Lazy, OnceCell};
use std::sync::mpsc;

fn launch() {
    // Launches the game, initializing the window and loading the sprites,
    let (tx, rx): (mpsc::Sender<EventCode>, mpsc::Receiver<EventCode>) = mpsc::channel();
    let mut TOOLBOX = Toolbox::new(tx.clone());

    game_loop(TOOLBOX, rx);
}

fn game_loop(tool_box: Toolbox, rec: mpsc::Receiver<EventCode>) {
    // The actual main game loop

    let mut res = true;

    while res {
        // debug ways of dropping out of game loop, remove later
        match &tool_box.game_state.get_play_status() {
            PlayStatus::WIN => res = false,
            PlayStatus::PLAYING => res = true,
            PlayStatus::LOSS => res = false,
        }

        // Game loop stuff here

        // Pre-update

        //for now just poll for a single message a frame, eventually the goal will be to clear the queue each frame, or at least as much as possible
        let mut msg: EventCode = EventCode::EXAMPLE;

        while !matches!(msg, EventCode::EMPTY) {
            msg = rec.try_recv().unwrap_or_else(|err| match err {
                mpsc::TryRecvError::Empty => EventCode::EMPTY,
                mpsc::TryRecvError::Disconnected => unreachable!(), // mpsc disconnect shouldnt be possible
            });
            if matches!(msg, EventCode::EMPTY) {
                //temp just making sure event queue isnt empty here
                break;
            }

            // Event handling

            println!("{:?}", msg);
        }

        update();

        // Post-update, pre-render

        render(&tool_box);

        // Post-frame
    }
}

fn update() {
    // The game loop, should process all computation needed between frames
}

fn render(tool_box: &Toolbox) {
    // Draw all game objects based on GameState, run after update()
}

fn main() {
    println!("Initializing...");
    launch();
}
