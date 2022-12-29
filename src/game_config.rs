use serde::{Deserialize, Serialize};

use crate::util::Vec2D;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GameConfig {
    pub window_size: (u32, u32),
    pub window_title: String,

    pub board_size: Vec2D,
    pub mine_count: u16,

    pub assets: Vec<AssetPath>,

    pub tests: Vec<AssetPath>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssetPath {
    pub name: String,
    pub path: String,
}
impl AssetPath {
    pub fn to_tup(&self) -> (&str, &str) {
        (&self.name, &self.path)
    }
}
