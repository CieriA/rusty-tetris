/// A type that represents the direction of a rotation.
#[derive(Debug, Copy, Clone, Default)]
pub enum Rotation {
    #[default]
    Clockwise,
    CounterClockwise,
}

/// A type that represents the direction of a movement.
#[derive(Debug, Copy, Clone, Default)]
pub enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        match value {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => panic!("Invalid num (`Direction` goes from 0 to 3): {}", value)
        }
    }
}
impl From<Direction> for usize {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
}

impl Direction {
    /// Rotate the direction by 90 degrees based on input (clockwise/counterclockwise).
    pub fn rotate(self, rotation: Rotation) -> Self {
        let i: usize = self.into();
        match rotation {
            Rotation::Clockwise => Self::from((i + 1) % 4),
            Rotation::CounterClockwise => Self::from((i + 3) % 4),
        }
    }
}


