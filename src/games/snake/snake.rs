use crate::games::snake::{direction::Direction, food::Food, NUM_COLS, NUM_ROWS};
use crate::utils::frame::{Drawable, Frame};
use rusty_time::prelude::Timer;
use std::{cmp::max, collections::VecDeque, time::Duration};

pub struct Snake {
    pub body: VecDeque<(i32, i32)>,
    direction: Direction,
    length: usize,
    mover_timer: Timer,
    can_move: bool,
}

impl Snake {
    pub fn new() -> Self {
        let q: VecDeque<(i32, i32)> =
            VecDeque::from([((NUM_COLS / 2) as i32, (NUM_ROWS / 2) as i32)]);

        Self {
            body: q,
            direction: Direction::Right,
            length: 3,
            mover_timer: Timer::from_millis(500),
            can_move: true,
        }
    }
    pub fn move_up(&mut self) {
        if self.can_move && self.direction != Direction::Down && self.direction != Direction::Up {
            self.direction = Direction::Up;
            self.can_move = false;
        }
    }
    pub fn move_down(&mut self) {
        if self.can_move && self.direction != Direction::Down && self.direction != Direction::Up {
            self.direction = Direction::Down;
            self.can_move = false;
        }
    }
    pub fn move_left(&mut self) {
        if self.can_move && self.direction != Direction::Left && self.direction != Direction::Right
        {
            self.direction = Direction::Left;
            self.can_move = false;
        }
    }
    pub fn move_right(&mut self) {
        if self.can_move && self.direction != Direction::Left && self.direction != Direction::Right
        {
            self.direction = Direction::Right;
            self.can_move = false;
        }
    }
    pub fn detect_food(&mut self, food: &Food) -> bool {
        let head = self.body.front().unwrap();
        if head.0 == food.x as i32 && head.1 == food.y as i32 {
            self.length += 1;

            let new_duration = max(self.mover_timer.duration.as_millis() - 10, 50);
            self.mover_timer = Timer::from_millis(new_duration as u64);

            true
        } else {
            false
        }
    }
    pub fn update(&mut self, delta: Duration) -> bool {
        self.mover_timer.update(delta);
        if self.mover_timer.ready {
            self.mover_timer.reset();
            self.can_move = true;
            let head = self.body.front().unwrap();

            let new_head = match self.direction {
                Direction::Up => (head.0, head.1 - 1),
                Direction::Down => (head.0, head.1 + 1),
                Direction::Right => (head.0 + 1, head.1),
                Direction::Left => (head.0 - 1, head.1),
            };

            self.body.push_front(new_head);
            if self.body.len() - 1 == self.length {
                self.body.pop_back();
            }

            return true;
        }
        false
    }
    pub fn hit_something(&self) -> bool {
        let head = self.body.front().unwrap();

        if head.0 < 0 || head.0 == NUM_COLS as i32 || head.1 < 0 || head.1 == NUM_ROWS as i32 {
            return true;
        }

        let mut did_hit_self = false;

        for point in self.body.iter().skip(1) {
            if point.0 == head.0 && point.1 == head.1 {
                did_hit_self = true;
                break;
            }
        }

        did_hit_self
    }
    pub fn is_max_length(&self) -> bool {
        self.length == NUM_COLS * NUM_ROWS
    }
}

impl Drawable for Snake {
    fn draw(&self, frame: &mut Frame) {
        for (x, y) in self.body.iter() {
            if (0..NUM_COLS as i32).contains(&x) && (0..NUM_ROWS as i32).contains(&y) {
                frame[*x as usize][*y as usize] = "■";
            }
        }
    }
}
