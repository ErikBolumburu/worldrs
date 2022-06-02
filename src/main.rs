extern crate sdl2;

mod tile;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use tile::{Tile, TileType};
use std::time::Duration;
use noise::{NoiseFn, Perlin};
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
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
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

        canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
        canvas.fill_rect(Rect::new(50, 50, 50, 50)).unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn generate_world() -> ArrayBase<OwnedRepr<Tile>, Dim<[usize; 2]>>{
    let noise = Perlin::default();
    
    let mut init_tile: Tile = Tile{x_pos: 0, y_pos: 0, rect: sdl2::rect::Rect::new(100, 100, 100, 100), tile_type: TileType::GRASS};
    let world: ArrayBase<OwnedRepr<Tile>, Dim<[usize; 2]>> = Array::from_elem((WORLD_X_SIZE, WORLD_Y_SIZE), init_tile);

    return world;
}