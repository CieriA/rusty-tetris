//! # Pieces
//! Contains the struct def of the `TetrominoKind` and the `Tetromino`

pub mod tetromino;

pub use tetromino::Tetromino;
use vecmath::{Direction, Point};

/// Each type of pieces of the `Tetris` game.
#[derive(Debug, Clone, Copy, Default)]
enum TetrominoKind {
    O,
    #[default]
    I,
    T,
    S,
    Z,
    L,
    J,
}

impl TetrominoKind {
    /// `Up` facing of each tetromino.
    fn basic_shape(&self) -> [Point; 4] {
        match self {
            Self::O => [(1, 0), (1, 1), (2, 1), (2, 0)].map(Point::from),
            Self::I => [(0, 1), (1, 1), (2, 1), (3, 1)].map(Point::from),
            Self::J => [(1, 1), (0, 0), (0, 1), (2, 1)].map(Point::from),
            Self::L => [(1, 1), (0, 1), (2, 0), (2, 1)].map(Point::from),
            Self::S => [(1, 1), (0, 1), (1, 0), (2, 0)].map(Point::from),
            Self::Z => [(1, 1), (0, 0), (1, 0), (2, 1)].map(Point::from),
            Self::T => [(1, 1), (0, 1), (1, 0), (2, 1)].map(Point::from),
        }
    }
    /// Rotates self based on the given direction.
    /// # Returns
    /// its new coordinates.
    fn rotated_shape(&self, direction: Direction) -> [Point; 4] {
        let center = match self {
            Self::O => return self.basic_shape(),
            Self::I => Point::new(1, 2),
            _ => Point::new(1, 1),
        };
        self.basic_shape()
            .map(|p| p.rotate_around(center, direction))
    }
}
