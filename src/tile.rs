use std::{collections::HashMap, sync::Mutex};

use sdl2::{
    mouse::MouseButton,
    rect::Rect,
    render::{Canvas, Texture},
};

use crate::BOARD;

#[derive(Debug, Copy, Clone)]
pub struct Coords2d {
    pub x: i32,
    pub y: i32,
}
impl Coords2d {
    pub fn flatten(&self, dims: &Coords2d) -> i32 {
        (&self.x + (&self.y * dims.x)).into()
    }

    pub fn copy_expand(&self, width: i32, height: i32) -> Self {
        Coords2d {
            x: &self.x * width,
            y: &self.y * height,
        }
    }

    pub fn from_tuple(i: (i32, i32)) -> Self {
        Coords2d { x: i.0, y: i.1 }
    }
}

#[derive(Debug)]
pub struct GameBoard {
    tile_grid: Vec<Tile>, // A flattened 2 grid of tiles, split by row_count
    dims: Coords2d,
}

impl GameBoard {
    pub fn new(dims: Coords2d) -> Self {
        let mut __ = GameBoard {
            tile_grid: Vec::new(),
            dims: dims,
        };
        for x in 0..__.dims.x.into() {
            for y in 0..__.dims.y {
                __.tile_grid.push(Tile::new(Coords2d::from_tuple((x, y))));
            }
        }
        println!("{:?}", __);
        __
    }

    pub fn get_tile(&self, pos: Coords2d) -> Option<&Tile> {
        self.tile_grid.get(pos.flatten(&self.dims) as usize)
    }

    pub fn get_dims(&self) -> &Coords2d {
        &self.dims
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
    EMPTY,
    MINE,
}

struct RenderContext<'a> {
    rect: Rect,
    texture: Texture<'a>,
}

#[derive(Debug)]
pub struct Tile {
    pos: Coords2d,
    state: TileState,
    contains: TileType,
}

impl Tile {
    pub fn new(pos: Coords2d) -> Self {
        Tile {
            pos: pos,
            state: TileState::HIDDEN,
            contains: TileType::EMPTY,
        }
    }

    pub fn get_state(&self) -> &TileState {
        &self.state
    }
    // pub fn set_state(&self, st: TileState) {
    //     self.state = st;
    //     let __ = self.clone();
    //     __.set_state(st)
    // }

    pub fn to_save(&self) -> u8 {
        // Returns the 3 bit save format of this tile in a u8, only 3 first bits used
        todo!();
    }

    pub fn draw(&self, mut canvas: &mut Canvas<sdl2::video::Window>, tex: &HashMap<&str, Texture>) {
        match &self.get_state() {
            TileState::REVEALED => {
                let texture = tex.get("tile_revealed").expect("Failed to pull texture");
                let tru_pos = self.pos.copy_expand(32, 32);
                canvas.copy(
                    texture,
                    None,
                    Rect::new(tru_pos.x.into(), tru_pos.y, 32, 32),
                );
            }
            TileState::HIDDEN => {
                let texture = tex.get("tile_hidden").expect("Failed to pull texture");
                let tru_pos = self.pos.copy_expand(32, 32);
                canvas.copy(
                    texture,
                    None,
                    Rect::new(tru_pos.x.into(), tru_pos.y.into(), 32, 32),
                );
            }
            TileState::FLAGGED => todo!(),
            TileState::EXPLODED => todo!(),
        };
        //println!("Tile called draw at {}, {}", &self.pos.x, &self.pos.y)
    }

    pub fn on_click(&self, m: MouseButton) {
        match m {
            MouseButton::Unknown => todo!(),
            MouseButton::Left => {
                // set to revealed if not flagged
                todo!();
                //self.set_state(TileState::REVEALED);
            }
            MouseButton::Right => todo!(),
            _ => {}
        }
    }
}
