use crate::{game_board::GameBoard, game_config::GameConfig, util::Vec2D};
use parking_lot::FairMutex;
use rusty_glasses::{sprite::Sprite, GlassContext};
use sdl2::{pixels::Color, rect::Rect};

pub struct GameState {
    config: GameConfig,
    board: GameBoard,
    assets: Vec<FairMutex<Sprite>>,
    glass: GlassContext,
}

impl GameState {
    pub fn init(config: GameConfig) -> Self {
        let mut assets: Vec<FairMutex<Sprite>> = Vec::new();
        let mut temp: Vec<(String, String)> = Vec::new();

        for asset in &config.assets {
            assets.push(FairMutex::new(Sprite::from_path(asset.to_tup())));
            temp.push((
                String::from(asset.to_tup().0),
                String::from(asset.to_tup().1),
            ));
        }

        let glass = rusty_glasses::init(
            config.window_title.as_str(),
            config.window_size.0,
            config.window_size.1,
            temp,
        )
        .unwrap();

        GameState {
            board: GameBoard::new(config.board_size, config.mine_count),
            config,
            assets,
            glass,
        }
    }

    pub fn get_sprite(&self, sp: &str) -> Option<&FairMutex<Sprite>> {
        //TODO: have sprite loaded from hashmap of sprites using asset name

        for sprite in &self.assets {
            if sprite.lock().name == sp {
                return Some(sprite);
            }
        }
        None
    }

    pub fn update(&self) {
        // Game loop here
        let events = self.glass.handle_events();

        for event in events {
            match event {
                sdl2::event::Event::MouseButtonDown {
                    timestamp,
                    window_id,
                    which,
                    mouse_btn,
                    clicks,
                    x,
                    y,
                } => {
                    if (x / 32) < self.config.board_size.x && (y / 32) < self.config.board_size.y {
                        println!("Mouse clicked at: {}, {}", x / 32, y / 32);
                        let mut mb: bool = false;
                        match mouse_btn {
                            sdl2::mouse::MouseButton::Left => mb = true,
                            sdl2::mouse::MouseButton::Right => mb = false,
                            _ => (),
                        }

                        self.board.handle_click(
                            Vec2D {
                                x: x / 32,
                                y: y / 32,
                            },
                            mb,
                        )
                    }
                    // if (x / 32) < dims.x.into() && (y / 32) < dims.y.into() {
                    //     println!("Mouse Clicked at: {}, {}", x / 32, y / 32);
                    //     BOARD
                    //         .get()
                    //         .unwrap()
                    //         .get_tile(Coords2d::from_tuple(((x / 32).into(), (y / 32).into())))
                    //         .expect("Unable to get tile in click event")
                    //         .on_click(mouse_btn);
                    // }
                }
                _ => (), //TODO: handle rest of events
            }
        }
    }

    pub fn draw(&self) {
        self.glass.use_canvas(|canvas| {
            canvas.set_draw_color(Color::RGB(21, 21, 21));
            canvas.clear();

            for tile in &self.board.tile_map {
                let t = tile.lock();

                let sp = t.draw(self.board.get_mine(t.pos));

                for sp_name in sp {
                    let tex1 = &self.get_sprite(&sp_name).unwrap().lock();

                    let temp = self.glass.get_texture(&tex1.name);

                    canvas.copy(&temp, None, Rect::new(t.pos.x * 32, t.pos.y * 32, 32, 32));
                }
            }

            canvas.present();
        });
    }
}
