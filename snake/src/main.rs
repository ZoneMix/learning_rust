extern crate rand;
extern crate piston_window;

mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;
use draw::to_coord_u32;

use crate::game::Game;

const BACKGROUND_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (30, 30);

    let mut window: PistonWindow = WindowSettings::new(
        "Snake",
        [to_coord_u32(width), to_coord_u32(height)],
    ).exit_on_esc(true)
        .build()
        .unwrap();
    
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {

        });
    }
}
