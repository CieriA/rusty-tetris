//! # Matrix
//! Contains the struct def of the game's matrix

use pieces::Tetromino;
use vecmath::Point;
use std::{
    ops::{
        Index,
        IndexMut,
    },
    default::Default,
    process,
};
use sdl2::pixels::Color;

/// 10*20 2D array.
/// 
/// Each cell is either empty (`None`) or filled with a piece (`Some<Color>`)
type Grid = [[Option<Color>; Matrix::WIDTH]; Matrix::HEIGHT];

/// The game's matrix.
/// 
/// Contains a representation of each already placed piece in a 2D array.
#[derive(Clone)]
pub struct Matrix(Grid);

impl Index<Point> for Matrix {
    type Output = Option<Color>;
    fn index(&self, index: Point) -> &Self::Output {
        assert!(
            index.y >= 0 &&
            index.x >= 0 &&
            index.y < Self::HEIGHT as isize &&
            index.x < Self::WIDTH as isize,
            "value of (x, y): {}", index);
        &self.0[index.y as usize][index.x as usize]
    }
}

impl IndexMut<Point> for Matrix {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        assert!(
            index.y >= 0 &&
            index.x >= 0 &&
            index.y < Self::HEIGHT as isize &&
            index.x < Self::WIDTH as isize,
            "value of (x, y): {}", index);
        &mut self.0[index.y as usize][index.x as usize]
    }   
}

impl Index<usize> for Matrix {
    type Output = [Option<Color>; Self::WIDTH];
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }  
}

impl Matrix {
    pub const WIDTH: usize = 10;
    pub const HEIGHT: usize = 20;
    
    pub fn new() -> Self {
        Self(Grid::default())
    }

    /// Returns an iterator over the matrix.
    /// 
    /// The iterator yields all items from start to end.
    pub fn iter(&self) -> core::slice::Iter<[Option<Color>; Self::WIDTH]> {
        self.0.iter()
    }
    
    /// Increases by 1 each cell's y that is lower than `index`.
    pub fn push_down(&mut self, index: usize) {
        for i in (1..=index).rev() {
            self[i] = self[i - 1];
        }
    }
    
    /// Takes a piece and place it into the matrix.
    /// # Errors
    /// if the place cannot be placed, it ends the game stating the player lost.
    pub fn place_piece(&mut self, tetromino: Tetromino) {
        for coord in tetromino.position() {
            if self[coord].is_some() {
                println!("You lost.");
                process::exit(1);
            }
            self[coord] = Some(tetromino.get_color());
        }
    }
    /// Returns `true` if the given coordinate is in the bounds of the matrix, `false` otherwise.
    fn in_bounds(&self, coord: Point) -> bool {
        coord.y >= 0 &&
        coord.x >= 0 &&
        coord.y < Self::HEIGHT as isize &&
        coord.x < Self::WIDTH as isize
    }
    /// Returns `true` if each coordinate of a given piece is **not** in the bounds of the matrix
    /// or if it is already occupied,
    /// `false` otherwise.
    pub fn will_collide(&self, tetromino: &Tetromino) -> bool {
        for coord in tetromino.position() {
            if !self.in_bounds(coord) {
                return true;
            }
            if let Some(_) = self[coord] {
                return true;
            }
        }
        false
    }
}
