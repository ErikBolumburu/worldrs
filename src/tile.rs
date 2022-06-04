#[derive(Clone, Eq, PartialEq)]
pub enum TileType{
    GRASS,
    WATER,
}

#[derive(Clone)]
pub struct Tile {
    pub x_pos: i32,
    pub y_pos: i32,
    pub height: f64,
    pub rect: sdl2::rect::Rect,
    pub tile_type: TileType,
}
