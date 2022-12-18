use std::{sync::Mutex, collections::HashMap};

use sdl2::{rect::Rect, render::{Texture, Canvas}};

use crate::BOARD;

#[derive(Debug)]
pub struct Coords2d {
    pub x: u16,
    pub y: u16,
}
impl Coords2d {
    pub fn flatten(&self, dims: &Coords2d) -> usize {
        (&self.x+(&self.y*dims.x)).into()
    }

    pub fn copy_expand(&self, width: u16, height: u16) -> Self {
        Coords2d {
            x: &self.x*width,
            y: &self.y*height,
        }
    }

    pub fn from_tuple(i: (u16, u16)) -> Self {
        Coords2d {
            x: i.0,
           y: i.1,
        }
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
        for x in 0..__.dims.x {
            for y in 0..__.dims.y {
                __.tile_grid.push(Tile::new(Coords2d::from_tuple((x,y))));
            }
        }
        println!("{:?}", __);
        __
    }

    pub fn get_tile(&self, pos: Coords2d) -> Option<&Tile> {
        self.tile_grid.get(pos.flatten(&self.dims))
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

    pub fn draw(&self, mut canvas: &mut Canvas<sdl2::video::Window>, tex: Mutex<HashMap<&str, Texture>>) {
        match &self.get_state() {
            TileState::REVEALED => todo!(),
            TileState::HIDDEN => {
                let texture = tex.lock().unwrap().get("tile_hidden").expect("Failed to pull texture");
                canvas.copy(texture, None, Rect::new(self.pos.x.into(), self.pos.y.into(), 32, 32));
            },
            TileState::FLAGGED => todo!(),
            TileState::EXPLODED => todo!(),
        };
        println!("Tile called draw at {}, {}", &self.pos.x, &self.pos.y)
    }
}