// Importing functions from crates
use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

// Importing functions and objects
use crate::draw::draw_block;

// Snake color constants
const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

// Snake enum directions
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Implemeting directions
impl Direction {
    pub fn opposite(&self) -> Direction {
        // Match directions
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

// Block structure
#[derive(Debug, Clone)]
struct Block {
    x: i32,     // X block coordinate
    y: i32,     // Y block coordinate
}

// Snake structure
pub struct Snake {
    direction: Direction,           // Snake direction
    body: LinkedList<Block>,        // Snake body - linked list made out of blocks
    tail: Option<Block>,            // Snake tail - extending after eating
}

// Snake implementation
impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        // Creating body
        let mut body: LinkedList<Block> = LinkedList::new();

        // Adding blocks to body
        body.push_back(Block {
            x: x + 2,
            y,
        });
        body.push_back(Block {
            x: x + 1,
            y,
        });
        body.push_back(Block {
            x,
            y,
        });

        // Creating snake
        Snake {
            direction: Direction::Right,        // Snakes starting direction
            body,                               // Snakes starting body
            tail: None,                         // Snakes starting tail
        }
    }

    // Draw snake function
    pub fn draw(&self, context: &Context, graphics: &mut G2d) {
        // For every block in body
        for block in &self.body {
            // Draw block body based on its coordinates
            draw_block(SNAKE_COLOR, block.x, block.y, context, graphics);
        }
    }

    // Getting head position
    pub fn head_position(&self) -> (i32, i32) {
        // Assigning head as first element of body
        let head_block = self.body.front().unwrap();
        // Returning heads coordinates
        (head_block.x, head_block.y)
    }

    // Snake moving forward
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        // Match snake directions
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }
        
        // Last snakes coordinates are its head position
        let (last_x, last_y): (i32, i32) = self.head_position();
        // Changing snake directions based on input
        let new_block = match self.direction {
            // Up
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
            // Down
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            // Left
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            // Right
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };

        // Add new block to the front of the snake
        self.body.push_front(new_block);

        // Get removed block from back of the snake
        let removed_block = self.body.pop_back().unwrap();
        
        // Add removed block to snakes tail
        self.tail = Some(removed_block);
    }

    // Getting snakes direction
    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    // Getting snakes next direction
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        // Assigning variables to head coordinates
        let (head_x, head_y): (i32, i32) = self.head_position();

        // Getting snakes direction
        let mut moving_direction = self.direction;
        // Matching direction
        match dir {
            Some(d) => moving_direction = d,
            None => {}
        }

        // Changing snakes head position based on moving direction
        match moving_direction {
            Direction::Up => (head_x, head_y - 1),          // Up
            Direction::Down => (head_x, head_y + 1),        // Down
            Direction::Left => (head_x - 1, head_y),        // Left
            Direction::Right => (head_x + 1, head_y),       // Right
        }
    }

    // Restore snakes tail
    pub fn restore_tail(&mut self) {
        // Cloning snakes tail
        let block = self.tail.clone().unwrap();
        // Push cloned block 
        self.body.push_back(block);
    }

    // Checking if snakes tail is overlaping
    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        // for every block in body
        for block in &self.body {
            // if x and y coordinates are equals to snakes tail x and y coordinates
            if x == block.x && y == block.y {
                // Snakes is overlaping its tail
                return true;
            }
            
            ch += 1;
            // If counter is size of snakes body
            if ch == self.body.len() - 1 {
                // Break loop
                break;
            }
        }
        // Snakes isnt overlaping its tail
        return false;
    }
}
