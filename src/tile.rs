use rstar::{RTreeObject, AABB};
use sfml::graphics::{Drawable, Image, Texture, Rect};

#[derive(Debug)]
pub struct Tile {
    state: TileState,
    pos: (i32, i32), //0 is x, 1 is y
    tile_type: TileType,
    img: Image,

}

impl RTreeObject for Tile {
    type Envelope = AABB<[i32; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point([self.pos.0, self.pos.1])
    }
}

impl Drawable for Tile {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn sfml::graphics::RenderTarget,
        states: &sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        &self.draw();
    }
}

impl Tile {
    pub fn new(ipos: (i32, i32), sprite: &str) -> Tile {
        Tile {
            state: TileState::HIDDEN,
            pos: ipos,
            tile_type: TileType::CLEAR,
            img: Image::from_file("./data/images/tile_hidden.png").unwrap(),
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

    pub fn draw(&self) {
        let mut tex = Texture::new().unwrap();
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
