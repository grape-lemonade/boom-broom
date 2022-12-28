use crate::util::Vec2D;

#[derive(Debug)]
pub struct Tile {
    pub pos: Vec2D,
    pub state: TileState,
    pub is_mine: bool,
}

impl Tile {
    pub fn new(pos: Vec2D, mine: bool) -> Self {
        Tile {
            pos,
            state: TileState::HIDDEN,
            is_mine: mine,
        }
    }

    pub fn draw(&self, mines: i32) -> Vec<String> {
        //TODO: have tile query for its sprite and return it here, drawing is done elsewhere this just tells what to draw and where

        match self.state {
            TileState::REVEALED => {
                if mines > 0 {
                    vec![String::from("tile_revealed"), format!("num_{}", mines)]
                } else {
                    vec![String::from("tile_revealed")]
                }
            }
            TileState::HIDDEN => vec![String::from("tile_hidden")],
            TileState::FLAGGED => vec![String::from("tile_hidden"), String::from("flag")],
            TileState::EXPLODED => vec![String::from("tile_revealed"), String::from("mine")],
        }
    }

    pub fn left_click(&mut self) {
        println!("Tile clicked");
        match self.state {
            TileState::REVEALED => (), // If clicking on revealed tile do nothing
            TileState::HIDDEN => {
                if self.is_mine {
                    self.state = TileState::EXPLODED;
                } else {
                    self.state = TileState::REVEALED;
                }
            }
            TileState::FLAGGED => (), // If clicking on flag do nothing
            TileState::EXPLODED => self.state = TileState::HIDDEN,
        }
    }

    pub fn right_click(&mut self) {
        println!("Tile right clicked");
        match self.state {
            TileState::REVEALED => (),
            TileState::HIDDEN => self.state = TileState::FLAGGED,
            TileState::FLAGGED => self.state = TileState::HIDDEN,
            TileState::EXPLODED => (),
        }
    }
}

#[derive(Debug)]
pub enum TileState {
    REVEALED,
    HIDDEN,
    FLAGGED,
    EXPLODED,
}
