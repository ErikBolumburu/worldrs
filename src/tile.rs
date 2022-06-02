pub enum TileType{
    GRASS,
    WATER,
}

pub struct Tile {
    x_pos: i32,
    y_pos: i32,
    rect: sdl2::rect::Rect,
    tile_type: TileType,
}