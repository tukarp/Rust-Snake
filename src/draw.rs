// Importing functions from crates
use piston_window::{G2d, Context, rectangle};
use piston_window::types::Color;

// Block size constant
const BLOCK_SIZE: f64 = 25.0;

// Getting coordinates as float_64
pub fn to_coordinates(game_coordinates: i32) -> f64 {
    (game_coordinates as f64) * BLOCK_SIZE
}

// Getting coordinates as unsigned_int_32
pub fn to_coordinates_u32(game_coordinates: i32) -> u32 {
    to_coordinates(game_coordinates) as u32
}

// Drawing block
pub fn draw_block(color: Color, x: i32, y: i32, context: &Context, g: &mut G2d) {
    // Graphical user interface
    let gui_x: f64 = to_coordinates(x);
    let gui_y: f64 = to_coordinates(y);

    // Rectangle object
    rectangle (
        color,                  // Color
        [gui_x,                 // GUI X coordinate
        gui_y,                  // GUI Y coordinate
        BLOCK_SIZE,             // Block size width
        BLOCK_SIZE],            // Block size height
        context.transform,      // Context
        g,                      // Graphics
    );
}

// Drawing rectangle
pub fn draw_rectangle(
    color: Color,       // Color
    x: i32,             // X coordinate
    y: i32,             // Y coordinate
    width: i32,         // Rectangle width
    height: i32,        // Rectangle Height
    context: &Context,  // Context
    g: &mut G2d,        // Graphics
) {
    // Graphical user interface
    let x: f64 = to_coordinates(x);
    let y: f64 = to_coordinates(y);

    // Rectangle object
    rectangle(
        color,                              // Color
        [x,                                 // X coordinate
        y,                                  // Y coordinate
        BLOCK_SIZE * (width as f64),        // Block size width
        BLOCK_SIZE * (height as f64),],     // Block size height
        context.transform,                  // Context
        g,                                  // Graphics
    );
}