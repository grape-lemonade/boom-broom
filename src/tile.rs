#[derive(Debug)]
pub struct Tile {
    state: TileState,
    pos: (u32, u32), //0 is x, 1 is y
}
impl Tile {
    pub fn new(ipos: (u32, u32)) -> Tile {
        Tile {
            state: TileState::HIDDEN,
            pos: ipos,
        }
    }

    pub fn get_location(&self) -> &(u32, u32) {
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
