use std::{borrow::Borrow, collections::HashMap, sync::Mutex};

use once_cell::sync::Lazy;
use rstar::{RTreeObject, AABB};
use sfml::{graphics::{Drawable, Image, Texture, Rect, Sprite}, SfBox};

const Textures: Lazy<Mutex<HashMap<&str, SfBox<Texture>>>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert("tile_hidden", Texture::from_file("./data/images/tile_hidden.png").unwrap());
    Mutex::new(m)
});

#[derive(Debug)]
pub struct Tile {
    state: TileState,
    pos: (i32, i32), //0 is x, 1 is y
    tile_type: TileType,
}

impl Drawable for Tile {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
            &'a self,
            target: &mut dyn sfml::graphics::RenderTarget,
            states: &sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>,
        ) {
            //todo
        match self.get_state() {
            TileState::REVEALED => {
                
            },
            TileState::HIDDEN => todo!(),
            TileState::FLAGGED => todo!(),
            TileState::EXPLODED => todo!(),
        }
    }
}

impl RTreeObject for Tile {
    type Envelope = AABB<[i32; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point([self.pos.0, self.pos.1])
    }
}

impl Tile {
    pub fn new(ipos: (i32, i32), sprite: &str) -> Tile {
        let mut tex = Texture::new().expect("error allocating tile texture");
        tex.load_from_file(sprite, Rect::new(0, 0, 32, 32));
        Tile {
            state: TileState::HIDDEN,
            pos: ipos,
            tile_type: TileType::CLEAR,
        }
    }

    pub fn get_location(&self) -> &(i32, i32) {
        &self.pos
    }

    pub fn get_state(&self) -> &TileState {
        &self.state
    }

    pub fn get_neighbors(&self) -> [Option<&Tile>; 8] {
        //Return the 8 tiles surrounding this one
        todo!();
    }

    pub fn set_state(mut self, new_state: TileState) {
        self.state = new_state;
    }
    pub fn set_neighbors(mut self, news: [Option<&Tile>; 8]) {
        // Change the 8 tiles surrouding with the new refs
        todo!();
    }

    pub fn on_click_left(&self) {
        // should be names onReveal
        todo!();
    }
    pub fn on_click_right(&self) {
        // should be names onFlag
        todo!();
    }

}
#[derive(Debug)]
pub enum TileState {
    REVEALED,
    HIDDEN,
    FLAGGED,
    EXPLODED,
}
#[derive(Debug)]
pub enum TileType {
    CLEAR,
    BOMB,
}
