use rusty_time::timer::Timer;
use crate::{NUM_COLS, NUM_ROWS};
use std::time::Duration;
use std::cmp::{min, max};
use crate::frame::{Drawable, Frame};

pub struct Invader {
    x: usize,
    y: usize,
}
pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        // spawning invaders
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if (x > 1)
                    && (x < NUM_COLS - 2)
                    && (y > 0)
                    && (y < NUM_ROWS/2)
                    && (x % 2 == 0)
                    && (y % 2 == 0) {
                    army.push(Invader { x, y });
                }
            }
        }
        Self {
            army,
            move_timer: Timer::from_millis(2000),
            direction: 1,
        }
    }

    /*
    If army reaches leftmost or rightmost of the screen:
        Move downwards, increase speed and change direction
    Else:
        Move left/right
    */
    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);
        if self.move_timer.ready {
            self.move_timer.reset();
            let mut downwards = false;
            if self.direction == -1 { // -1 = left
                let min_x = self.army.iter()
                    .map(|invader| invader.x)
                    .min().unwrap_or(0);
                if min_x == 0 { // reaches the leftmost
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                let max_x = self.army.iter()
                    .map(|invader| invader.x)
                    .max().unwrap_or(0);
                if max_x == NUM_COLS -1 {
                    self.direction = -1;
                    downwards = true;
                }
            }
            if downwards {
                // increase speed by decreasing Duration
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }
            return true;
        }
        false
    }
    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> bool { // Will be called in each 'frame'
        if let Some(idx) = self
            .army
            .iter()
            .position(|invader| invader.x == x && invader.y == y) {
            self.army.remove(idx);
            true
        } else {
            false
        }
    }
    // Win
    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }
    // Lose
    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|invader| invader.y).max().unwrap_or(0) >= NUM_ROWS - 1
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            // Animation:
            // for half of the time in Duration, it is "x", the other half of time is "+"
            frame[invader.x][invader.y] =
                if (self.move_timer.time_left.as_secs_f32()
                    / self.move_timer.duration.as_secs_f32()) > 0.5 {
                    "x"
                } else {
                    "+"
                }
        }
    }
}
