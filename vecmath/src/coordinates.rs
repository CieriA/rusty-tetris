use crate::Direction;
use std::{
    fmt::Display,
    ops::{Add, AddAssign},
};

/// Struct that represents a 2D point used to index a matrix or a vector to move a point.
#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    #[inline(always)]
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    /// Rotates `self` around a given `center` to the given `direction`
    pub fn rotate_around(self, center: Point, direction: Direction) -> Self {
        let dx = self.x - center.x;
        let dy = self.y - center.y;
        let (rx, ry) = match direction {
            Direction::Up => (dx, dy),
            Direction::Right => (-dy, dx),
            Direction::Down => (-dx, -dy),
            Direction::Left => (dy, -dx),
        };
        Self::new(center.x + rx, center.y + ry)
    }
}

impl Display for Point {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(isize, isize)> for Point {
    #[inline(always)]
    fn from(tuple: (isize, isize)) -> Self {
        Self::new(tuple.0, tuple.1)
    }
}

impl Add for Point {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Point {
    #[inline(always)]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}
