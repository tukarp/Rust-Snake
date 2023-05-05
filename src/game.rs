// Importing functions from crates
use crate::piston_window::*;
use crate::piston_window::types::Color;

// Importing functions and objects
use crate::rand::{thread_rng, Rng};
use crate::snake::{Direction, Snake};
use crate::draw::{draw_block, draw_rectangle};

// Color constants
const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

// Game constans
const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

// Game structure
pub struct Game {
    snake: Snake,       // Snake
    food_exists: bool,  // Does food exists
    food_x: i32,        // Food x coordinate
    food_y: i32,        // Food y coordinate
    score: i32,         // Score
    width: i32,         // Window width
    height: i32,        // Window height
    game_over: bool,    // Game over flag
    waiting_time: f64,  // Waiting time
}

// Game structure implementation
impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        // Setting starting values
        Game {
            snake: Snake::new(2, 2),    // Snake with his starting position
            food_exists: true,          // Does food already exists
            food_x: 6,                  // Food x coordinate
            food_y: 4,                  // Food y coordinate
            score: 0,                   // Players score
            width,                      // Game window width
            height,                     // Game window height
            game_over: false,           // Game over flag
            waiting_time: 0.0,          // Waiting time
        }
    }

    // Checking if key is pressed
    pub fn key_pressed(&mut self, key: Key) {
        // If game over is true return
        if self.game_over {
            return;
        }

        // Checking for matching inputs with arrow keys
        let direction = match key {
            Key::Up => Some(Direction::Up),         // Up
            Key::Down => Some(Direction::Down),     // Down
            Key::Left => Some(Direction::Left),     // Left
            Key::Right => Some(Direction::Right),   // Right
            _ => Some(self.snake.head_direction())  // None
        };

        // Reverse snakes direction if needed
        if let Some(direction) = direction {
            if direction == self.snake.head_direction().opposite() {
                return;
            }
        }

        // Updating snake direction
        self.update_snake(direction);
    }

    // Drawing contents
    pub fn draw(&self, context: &Context, g: &mut G2d) {
        // Drawing snake
        self.snake.draw(context, g);

        // If food exists - draw it
        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, context, g);
        }

        // Drawing borders
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, context, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, context, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, context, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0 , 1, self.height, context, g);
        
        // If game over flag is true - draw with different colors
        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, context, g);
        }
    }

    // Game update
    pub fn update(&mut self, delta_time: f64) {
        // Waiting time update
        self.waiting_time += delta_time;

        // Checking game over flag
        if self.game_over {
            // If game is over and waiting timer is done
            if self.waiting_time > RESTART_TIME {
                // Restart game
                self.restart();
            }
            return;
        }

        // If there isnt any food
        if !self.food_exists {
            // Add food
            self.add_food();
        }

        // If its possible to move
        if self.waiting_time > MOVING_PERIOD {
            // Update snakes position
            self.update_snake(None);
        }
    }

    // Check if snake ate food
    fn check_eating(& mut self) {
        // Getting snakes head position
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        // If head position matches food position
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            // Mark food_exists flag as false
            self.food_exists = false;
            // Expand snakes tail
            self.snake.restore_tail();
            // self.update_score();
        }
    }

    // Checking if snake is alive
    fn check_if_snake_alive(&self, direction: Option<Direction>) -> bool {
        // Getting next snakes head position
        let (next_x, next_y) = self.snake.next_head(direction);

        // If next snakes position overlaps tail
        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        // Assign boolean value to snake alive flag
        // to check wheter snake hit the wall or not
        let snake_alive_flag: bool = next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1;

        // Return is snake alive flag
        snake_alive_flag
    }

    // Spawning food in random places
    fn add_food(&mut self) {
        // Assigning rng
        let mut rng = thread_rng();

        // Generate new food coordinates
        let mut new_x = rng.gen_range(1..self.width - 1);
        let mut new_y = rng.gen_range(1..self.height - 1);

        // If snakes tail overlaps food
        while self.snake.overlap_tail(new_x, new_y) {
            // Generate new food coordinates
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.height - 1);
        }

        // Assign new food to game food object
        self.food_x = new_x;
        self.food_y = new_y;

        // Mark food exists flag as true
        self.food_exists = true;
    }

    // Updating snake position
    fn update_snake(&mut self, direction: Option<Direction>) {
        // If snake is alive
        if self.check_if_snake_alive(direction) {
            // Move snake forward
            self.snake.move_forward(direction);
            // Check if he ate food
            self.check_eating();
        } else {
            // Mark game over flag as true
            self.game_over = true;
        }
        // Set waiting time to 0
        self.waiting_time = 0.0;
    }

    // Unfinished score handling
/*
    fn update_score(&mut self) {
        self.score += 1;
        self.draw_score()
    }

    fn draw_score(&mut self) {
        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32).draw(
            &self.score.to_string(),
        );
    }
*/

    // Setting values to starting ones
    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);      // Snake with his starting position
        self.food_exists = true;            // Marking food exists flag as true
        self.food_x = 6;                    // Food x coordinate
        self.food_y = 4;                    // Food y coordinate
        self.score = 0;                     // Players score
        self.game_over = false;             // Marking game over flag as false
    }
}