pub mod tile;
use std::fs;

use parking_lot::FairMutex;

use nanorand::{Rng, WyRand};

use self::tile::{Tile, TileState};
use crate::util::*;

#[derive(Debug)]
pub struct GameBoard {
    pub size: Vec2D,
    pub mine_count: u16,

    pub tile_map: Vec<FairMutex<Tile>>, // A flattened 2 grid of tiles, split by row_count

    pub time: u32,
}

impl GameBoard {
    pub fn new(size: Vec2D, mines: u16) -> Self {
        let mut tile_map: Vec<FairMutex<Tile>> = Vec::new();

        //todo: generate the tiles on the board
        for x in 0..size.x {
            for y in 0..size.y {
                tile_map.push(FairMutex::new(Tile::new(Vec2D { x, y }, false)));
            }
        }

        let mut rng = WyRand::new();
        for mc in 0..mines {
            let px: i32 = rng.generate_range(0..size.x);
            let py: i32 = rng.generate_range(0..size.y);

            tile_map[Vec2D {
                x: px.clamp(0, size.x - 1),
                y: py.clamp(0, size.y - 1),
            }
            .flatten(&size)
            .unwrap() as usize]
                .lock()
                .is_mine = true;
        }

        GameBoard {
            size,
            mine_count: mines,
            tile_map,
            time: 0,
        }
    }

    pub fn from_test(size: Vec2D, dat: &str) -> Self {
        let mut tile_map: Vec<FairMutex<Tile>> = Vec::new();
        let mut mine_count = 0;

        let mut y = 0;

        // Something in this block of code causes a seg fault when compiling in mingw

        // let data = dat.split('\n');
        // for row in data {
        //     for mut x in 0..size.x {
        //         let byte = row.as_bytes()[x as usize];

        //         let is_mine = match byte {
        //             0x00 => {
        //                 x += 1;
        //                 Some(false)
        //             }
        //             0x01 => {
        //                 x += 1;
        //                 mine_count += 1;
        //                 Some(true)
        //             }
        //             _ => None,
        //         };

        //         if is_mine.is_some() {
        //             println!("Is mine: {:?}", is_mine.unwrap());

        //             tile_map.push(FairMutex::new(Tile {
        //                 pos: Vec2D { x, y },
        //                 state: TileState::HIDDEN,
        //                 is_mine: is_mine.unwrap(),
        //             }))
        //         }
        //     }
        //     y += 1;
        // }

        GameBoard {
            size,
            mine_count,
            tile_map,
            time: 0,
        }
    }

    pub fn get_mine(&self, pos: Vec2D) -> i32 {
        let neighbors = self.get_neighbors(pos);

        let mut count = 0;
        for tile in neighbors {
            if tile.is_none() {
                count += 0;
            } else {
                if tile.unwrap().lock().is_mine == true {
                    count += 1;
                } else {
                    count += 0;
                }
            }
        }
        count
    }

    pub fn get_neighbors(&self, pos: Vec2D) -> [Option<&FairMutex<Tile>>; 8] {
        let array = [
            Vec2D {
                x: pos.x - 1,
                y: pos.y - 1,
            }, //0
            Vec2D {
                x: pos.x - 0,
                y: pos.y - 1,
            }, //1
            Vec2D {
                x: pos.x + 1,
                y: pos.y - 1,
            }, //2
            Vec2D {
                x: pos.x - 1,
                y: pos.y - 0,
            }, //3
            Vec2D {
                x: pos.x + 1,
                y: pos.y + 0,
            }, //4
            Vec2D {
                x: pos.x - 1,
                y: pos.y + 1,
            }, //5
            Vec2D {
                x: pos.x + 0,
                y: pos.y + 1,
            }, //6
            Vec2D {
                x: pos.x + 1,
                y: pos.y + 1,
            }, //7
        ];

        let mut result: [Option<&FairMutex<Tile>>; 8] = [None; 8];

        let mut i = 0;
        for tile in array {
            if tile.flatten(&self.size) == None {
                result[i] = None;
            } else {
                result[i] = Some(&self.tile_map[tile.flatten(&self.size).unwrap() as usize]);
            }
            i += 1;
        }
        result
        //self.tile_map[]
    }

    // pub fn get_mine_count(&self, pos: Vec2D) -> i32 {
    //     let mut count: i32 = 0;
    // }

    pub fn handle_click(&self, pos: Vec2D, button: bool) {
        //TODO: when revealing tile, if mine count is 0, run click function for all neighbors with mine count 0 recursively

        if self.get_mine(pos) == 0 {
            let n = self.get_neighbors(pos);

            for tile in n {
                if tile.is_some() {
                    if (self.get_mine(tile.unwrap().lock().pos) == 0) {
                        tile.unwrap().lock().left_click();
                        //self.handle_click(tile.unwrap().lock().pos, true);
                        // Propagate this
                    }
                }
            }
        }

        match button {
            true => self.tile_map[pos.flatten(&self.size).unwrap() as usize]
                .lock()
                .left_click(),
            false => self.tile_map[pos.flatten(&self.size).unwrap() as usize]
                .lock()
                .right_click(),
        }
    }
}
