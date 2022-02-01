extern crate rand;
extern crate piston_window;
extern crate find_folder;

mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;
use draw::to_coord_u32;

use crate::game::Game;

const BACKGROUND_COLOR: Color = [0.2, 0.2, 0.2, 1.0];
//const TEXT_COLOR: Color = [1.0, 1.0, 1.0, 1.0];

fn main() {
    let (width, height) = (30, 30);

    let mut window: PistonWindow = 
    WindowSettings::new("Snake",[to_coord_u32(width), to_coord_u32(height)])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(width, height);

    /* For rendering text
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let font = assets.join("DejaVuSans.ttf");
    let factory = window.factory.clone();
    let command_buffer = window.factory.create_command_buffer().into();

    let mut glyphs = Glyphs::new(
        font,
        TextureContext {
            factory: factory,
            encoder: command_buffer,
        }, 
        TextureSettings::new(),
    ).unwrap();
    */
    
    while let Some(e) = window.next() {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&e, |c, g, _| {
            clear(BACKGROUND_COLOR, g);
            game.draw(&c, g);


            /* Tried to print score to gui, but the text
                library is kind of wonky

            let transform = c.transform.trans(22.0, 50.0);
            let score: u16 = game.get_score();
            let score_s: String = score.to_string();
            let score_slice: &str = &score_s;

            text::Text::new_color(TEXT_COLOR, 18)
                .draw(score_slice, &mut glyphs, &c.draw_state, transform, g)
                .unwrap();
            */
        });

        e.update(|arg| {
            game.update(arg.dt);
        });


    }
}
