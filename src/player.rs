use crate::{NUM_COLS, NUM_ROWS};
use crate::frame::{Drawable, Frame};
use crate::shot::Shot;
use std::time::Duration;
use crate::invader::Invaders;

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}

impl Player {
    pub fn new() -> Self {
        // init at center, bottom
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            shots: Vec::new(),
        }
    }
    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }
    pub fn move_right(&mut self) {
        if self.x < NUM_COLS -1 {
            self.x += 1;
        }
    }
    pub fn shoot(&mut self) -> bool {
        // only allows two shots
        if self.shots.len() < 2 {
            self.shots.push(Shot::new(self.x, self.y - 1));
            true
        } else {
            false
        }
    }
    pub fn update(&mut self, delta: Duration) {
        for s in self.shots.iter_mut() {
            s.update(delta);
        }
        // apply this filtering lambda to all elements
        self.shots.retain(|shot| !shot.dead());
    }
    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> bool {
        let mut hit = false;
        for s in self.shots.iter_mut() {
            if !s.exploding {
                if invaders.kill_invader_at(s.x, s.y) {
                    hit = true;
                    s.explode();
                }
            }
        }
        hit
    }
}

impl Drawable for Player { // implment trait
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A"; // "A" is the ship
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}