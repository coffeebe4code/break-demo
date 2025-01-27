// Coords are the indexes of the board
pub struct Coord {
    pub x: u32,
    pub y: u32,
}

impl Coord {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}
