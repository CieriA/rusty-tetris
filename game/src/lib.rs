//! # Game
//! Contains the struct to play the game

use matrix::Matrix;
use pieces::Tetromino;
use vecmath::{
    Direction,
    Rotation
};
use std::{
    time::Duration,
    process
};
use rand::random_range;
use sdl2::keyboard::Keycode;

/// Game struct. Handles the game logic and the score.
#[derive(Clone)]
pub struct Game {
    pub matrix: Matrix,
    pub cur_tetromino: Option<Tetromino>,
    pub bag: Vec<Tetromino>,
    pub score: u32,
    pub tick: Duration,
}

impl Game {
    pub const BASE_TICK: Duration = Duration::from_millis(600);
    const FAST_TICK: Duration = Duration::from_millis(Self::BASE_TICK.as_millis() as u64 / 20);
    
    pub fn new() -> Self {
        let mut game = Self {
            matrix: Matrix::new(),
            cur_tetromino: None,
            bag: Vec::with_capacity(Tetromino::SIZE),
            tick: Self::BASE_TICK,
            score: 0,
        };
        game.refill_bag();
        game.put_tetromino();
        game
    }
    
    /// Refills the bag when it is empty with all the 7 new variants shuffled
    /// # Panics
    /// when the `bag` is not empty
    pub fn refill_bag(&mut self) {
        assert!(self.bag.is_empty());
        self.bag.extend_from_slice(&Tetromino::bag());
    }
    
    /// Replace the `cur_tetromino` to the last tetromino in the bag if it is `None`
    fn put_tetromino(&mut self) {
        self.cur_tetromino = self.bag.pop();
    }
    
    /// Moves/Rotates the `cur_tetromino` left, right, clockwise or counterclockwise.
    pub fn update(&mut self, key: Keycode) {
        match key {
            Keycode::Left | Keycode::A => {                // Move left
                self.try_move(Direction::Left);
            }
            Keycode::Right | Keycode::D => {               // Move right
                self.try_move(Direction::Right);
            }
            Keycode::Down | Keycode::S => {                // Soft drop
                self.tick = Self::FAST_TICK;
            }
            Keycode::Space => {                            // Hard drop
                self.hard_drop();
            }
            Keycode::Up | Keycode::W | Keycode::RCTRL => { // Rotate clockwise
                self.try_rotate(Rotation::Clockwise);
            }
            Keycode::Q | Keycode::LCTRL => {               // Rotate counterclockwise
                self.try_rotate(Rotation::CounterClockwise);
            }
            _ => {}
        }
    }
    
    fn check_rows(&mut self) {
        for (i, row) in self.matrix.iter().enumerate().rev() {
            if row.iter().all(|block| matches!(block, Some(_))) {
                self.score += 100;
                self.matrix.push_down(i);
                break;
            }
        }
    }
    
    /// Tries to rotate `cur_tetromino` or does nothing if it can't.
    fn try_rotate(&mut self, rotation: Rotation) {
        let mut new = self.cur_tetromino.clone().unwrap();
        new.rotate(rotation);
        if self.matrix.will_collide(&new) {
            return;
        }
        self.cur_tetromino = Some(new);
    }
    
    /// Tries to move `cur_tetromino` left/right or does nothing if it can't.
    fn try_move(&mut self, direction: Direction) {
        let mut new = self.cur_tetromino.clone().unwrap();
        new.move_lr(direction);
        if self.matrix.will_collide(&new) {
            return;
        }
        self.cur_tetromino = Some(new);
    }
    
    /// Drop the `cur_tetromino`'s y down by 1 if possible.
    /// # Returns
    /// `true` if it can drop and `false` otherwise
    pub fn try_drop(&mut self) -> bool {
        let mut new = self.cur_tetromino.clone().unwrap();
        new.drop();
        if self.matrix.will_collide(&new) {
            return false;
        }
        self.cur_tetromino = Some(new);
        true
    }
    
    /// Moves the `cur_tetromino` into the matrix when it touches the ground.
    pub fn place(&mut self) {
        let Some(tetromino) = self.cur_tetromino.take() else {
            panic!("cur_tetromino is None");
        };
        if tetromino.position().iter().any(|point| point.y < 0 || point.x < 0) {
            println!("You lost");
            process::exit(1);
        }
        self.matrix.place_piece(tetromino);
        self.score += random_range(3..=8);
        while self.matrix.iter().any(|row| row.iter().all(|block| matches!(block, Some(_)))) {
            self.check_rows();
        }
        self.put_tetromino();
    }
    /// Instantly drops the `cur_tetromino` as down as possible and `place` it.
    fn hard_drop(&mut self) {
        if self.cur_tetromino.as_ref().unwrap().position().iter().any(|point| point.y < 0 || point.x < 0) {
            return;
        }
        self.cur_tetromino = self.get_ghost();
        self.place();
        self.score += 30;
    }
    /// Returns the shadow of the `cur_tetromino` placed as down as possible in the `matrix`.
    pub fn get_ghost(&self) -> Option<Tetromino> {
        let mut ghost = self.cur_tetromino.clone()?;
        while !self.matrix.will_collide(&ghost) {
            ghost.drop();
        }
        ghost.offset.y -= 1;
        Some(ghost)
    }
}
