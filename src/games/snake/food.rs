use crate::games::snake::{NUM_COLS, NUM_ROWS};
use crate::utils::{
    frame::{Drawable, Frame},
    generate_random_point::generate_random_point,
};
use std::collections::VecDeque;

#[derive(Copy, Clone)]

pub struct Food {
    pub x: usize,
    pub y: usize,
}

impl Food {
    pub fn new() -> Self {
        let (x, y) = generate_random_point((NUM_COLS, NUM_ROWS));

        Self { x, y }
    }
    pub fn eaten(&mut self, snake_body: &VecDeque<(i32, i32)>) -> bool {
        let mut point = generate_random_point((NUM_COLS, NUM_ROWS));
        let mut is_valid_point = false;

        while !is_valid_point {
            for part in snake_body.iter() {
                if part.0 == point.0 as i32 && part.1 == point.1 as i32 {
                    point = generate_random_point((NUM_COLS, NUM_ROWS));
                    break;
                }
            }
            is_valid_point = true;
        }

        self.x = point.0;
        self.y = point.1;

        true
    }
}

impl Drawable for Food {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "o";
    }
}
