use crate::gamestate::GameState;
use std::sync::mpsc;

use sfml::window as sf;
use sfml::graphics::*;
use sf::{Window, Style, Event};

#[derive(Debug)]
pub struct Toolbox {
    pub window: RenderWindow,
    pub game_state: GameState,
}
#[derive(Debug)]
pub enum EventCode {
    EXAMPLE,
    EMPTY,
}


impl Toolbox {
    pub fn new() -> Toolbox {
        Toolbox {
            window: RenderWindow::new((800,600), "P4 - Minesweeper, Taylor Hiatt", Style::CLOSE, &Default::default()),
            game_state: GameState::new(None, None),
        }
    }

    pub fn GameLoop(mut self) -> Self {

        while let Some(event) = self.window.poll_event() {
            //Event Handling

            match event {
                Event::Closed => self.window.close(),
                // Event::Resized { width, height } => todo!(),
                //Event::LostFocus => todo!(),
                // Event::GainedFocus => todo!(),
                // Event::TextEntered { unicode } => todo!(),
                Event::KeyPressed { code, alt, ctrl, shift, system } => println!("Key Pressed: {:?}", code),
                // Event::KeyReleased { code, alt, ctrl, shift, system } => todo!(),
                // Event::MouseWheelScrolled { wheel, delta, x, y } => todo!(),
                // Event::MouseButtonPressed { button, x, y } => todo!(),
                // Event::MouseButtonReleased { button, x, y } => todo!(),
                // Event::MouseMoved { x, y } => todo!(),
                //Event::MouseEntered => todo!(),
                // Event::MouseLeft => todo!(),
                // Event::JoystickButtonPressed { joystickid, button } => todo!(),
                // Event::JoystickButtonReleased { joystickid, button } => todo!(),
                // Event::JoystickMoved { joystickid, axis, position } => todo!(),
                // Event::JoystickConnected { joystickid } => todo!(),
                // Event::JoystickDisconnected { joystickid } => todo!(),
                // Event::TouchBegan { finger, x, y } => todo!(),
                // Event::TouchMoved { finger, x, y } => todo!(),
                // Event::TouchEnded { finger, x, y } => todo!(),
                // Event::SensorChanged { type_, x, y, z } => todo!(),
                _ => (), // Just do nothing if nothing is implemented for that event.
            }
            
        }        
        // debug ways of dropping out of game loop, remove later
        // match &tool_box.game_state.get_play_status() {
        //     PlayStatus::WIN => res = false,
        //     PlayStatus::PLAYING => res = true,
        //     PlayStatus::LOSS => res = false,
        // }

        // Game loop stuff here

        // Pre-update

        self = self.Update();

        // Post-update, pre-render
        self = self.Render();

        // Post-frame
        self.window.display();
        self
    }

    pub fn Update(self) -> Self {

        self        
    }

    pub fn Render(mut self) -> Self{
        let dims = &self.game_state.get_dims();

        for x in 0..dims.0 {
            for y in 0..dims.1 {
                self.window.draw(self.game_state.get_tile(x, y).unwrap());
            }
        }

        self
    }   

}
