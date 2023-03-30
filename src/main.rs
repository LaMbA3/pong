extern crate piston_window;

use game::Game;
use piston_window::*;

mod pong;
mod game;
mod ball;

const WINDOW_WIDTH: f64 = 640.0;
const WINDOW_HEIGHT:f64 = 800.0;

fn main() {

    let mut window: PistonWindow =
        WindowSettings::new("Pong", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true).build().unwrap();


    let mut game = Game::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    while let Some(event) = window.next() {

        if let Some(Button::Keyboard(key)) = event.press_args(){
            game.handle_key_press(key);
        }
        if let Some(Button::Keyboard(key)) = event.release_args(){
            game.handle_key_release(key);
        }
        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.0,0.0,0.0,1.0], graphics);

            game.draw(context, graphics);
        });
    }

}
