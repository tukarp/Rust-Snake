// Importing crates
extern crate piston_window;
extern crate rand;

// Importing functions from crates
use crate::piston_window::types::Color;
use crate::piston_window::*;

// Importing functions and objects
use crate::draw::to_coordinates_u32;
use crate::game::Game;

// Using other files
mod game;
mod snake;
mod draw;

// Back color constant
const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    // Allocating variables
    // Width and height
    let (width, height) = (30, 30);

    // Game object
    let mut game = Game::new(width, height);

    // Piston window
    let mut window: PistonWindow = WindowSettings::new(
        "Snake",  // Title
        [to_coordinates_u32(width), to_coordinates_u32(height)],  // Window size
    ).exit_on_esc(true)  // Escape exits game
        .build()  // Build window
        .unwrap();  // Unwrap values

    // Game loop
    while let Some(event) = window.next() {
        // Button press handling
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);  // Key pressed flag
        }
        // Drawing game
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);  // Clear screen
            game.draw(&c, g);  // Draw contents
        });
        // Updating game
        event.update(|arg| {
            game.update(arg.dt);  // Game update method
        });
    }
}
