pub struct Tile {
    state: TileState,
    pos: (u32, u32), //0 is x, 1 is y
}
impl Tile {
    
}

pub enum TileState {
    REVEALED,
    HIDDEN,
    FLAGGED,
    EXPLODED,
}