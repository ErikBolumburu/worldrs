extern crate sdl2;

mod tile;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use tile::{Tile, TileType};
use std::time::Duration;
use noise::{Perlin,NoiseFn};
use ndarray::{Array, ArrayBase, OwnedRepr, Dim};

const WORLD_X_SIZE: usize = 800;
const WORLD_Y_SIZE: usize = 800;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("world-rs", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let world = generate_world();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        for tile in world.iter(){
            if tile.tile_type == TileType::WATER {
                canvas.set_draw_color(Color::RGBA(0, 0, 255, 255));
            }
            else if tile.tile_type == TileType::GRASS {
                canvas.set_draw_color(Color::RGBA(0, 255, 0, 255));
            }
            canvas.fill_rect(tile.rect).unwrap();
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn generate_world() -> ArrayBase<OwnedRepr<Tile>, Dim<[usize; 2]>>{
    let noise = Perlin::new();
    
    let init_tile: Tile = Tile{
        x_pos: 0,
        y_pos: 0,
        height: 0.0,
        rect: sdl2::rect::Rect::new(100, 100, 100, 100),
        tile_type: TileType::GRASS};

    let mut world: ArrayBase<OwnedRepr<Tile>, Dim<[usize; 2]>> 
    = Array::from_elem((WORLD_X_SIZE, WORLD_Y_SIZE), init_tile);

    for ((x, y), tile) in world.indexed_iter_mut(){
        tile.height = noise.get([x as f64 + 0.01, y as f64 + 0.01, 2.8]);
        tile.x_pos = x as i32;
        tile.y_pos = y as i32;
        tile.rect.x = x as i32 * 8;
        tile.rect.y = y as i32 * 8;
        tile.rect.w = 8;
        tile.rect.h = 8;
        if tile.height > 0.0 {
            tile.tile_type = TileType::GRASS;
        }
        else{
            tile.tile_type = TileType::WATER;
        }
    }

    return world;
}
