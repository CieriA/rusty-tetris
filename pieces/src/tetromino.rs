use crate::TetrominoKind as Kind;

use sdl2::pixels::Color;
use vecmath::{Rotation, Direction, Point};

use rand::seq::SliceRandom;

/// A tetromino is a shape that can be placed on the board.
#[derive(Debug, Clone, Default)]
pub struct Tetromino {
    kind: Kind,
    pub cells: [Point; 4],
    pub offset: Point,
    direction: Direction,
}

impl Tetromino {
    pub const SIZE: usize = 7;
    /// Constructor of `Tetromino`
    /// # Args
    /// `kind` is the type of Tetromino constructed.
    fn new(kind: Kind) -> Self {
        let offset = match kind {
            Kind::I => Point::new(2, -1),
            _ => Point::new(3, -1),
        };
        Self {
            cells: kind.basic_shape(),
            kind,
            offset,
            ..Default::default()
        }
    }
    /// # Returns
    /// an array of all the 7 variants shuffled
    pub fn bag() -> [Tetromino; Self::SIZE] {
        let mut pieces = [
            Tetromino::new(Kind::O),
            Tetromino::new(Kind::I),
            Tetromino::new(Kind::T),
            Tetromino::new(Kind::S),
            Tetromino::new(Kind::Z),
            Tetromino::new(Kind::L),
            Tetromino::new(Kind::J),
        ];
        pieces.shuffle(&mut rand::rng());
        pieces
    }
    /// Returns the color of `self` (each tetromino has its own color)
    pub fn get_color(&self) -> Color {
        match self.kind {
            Kind::I => Color::RGB(0, 231, 254),     // LIGHT BLUE
            Kind::J => Color::BLUE,                        // BLUE
            Kind::L => Color::RGB(255, 127, 0),     // ORANGE
            Kind::O => Color::RGB(255, 200, 0),     // YELLOW
            Kind::S => Color::GREEN,                       // GREEN
            Kind::Z => Color::RED,                         // RED
            Kind::T => Color::MAGENTA,                     // MAGENTA
        }
    }

    /// Drops `self`'s y **in place** by 1.
    pub fn drop(&mut self) {
        self.offset += Point::new(0, 1);
    }
    
    /// Moves `self` **in place** left/right based on the input
    pub fn move_lr(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                self.offset += Point::new(-1, 0);
            }
            Direction::Right => {
                self.offset += Point::new(1, 0);
            }
            _ => {}
        }
    }

    // TODO refactor ('cause functions used inside it will be refactored)
    /// Rotates `self` **in place** clockwise or counterclockwise based on the input.
    pub fn rotate(&mut self, rotation: Rotation) {
        self.direction = self.direction.rotate(rotation);
        self.cells = self.kind.rotated_shape(self.direction);
    }
    
    pub fn position(&self) -> [Point; 4] {
        self.cells.map(|cell| cell + self.offset)
    }
}
