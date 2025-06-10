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
    Right,
    Down,
    Left,
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

impl Direction {
    /// Rotate the direction by 90 degrees based on input (clockwise/counterclockwise).
    pub fn rotate(self, rotation: Rotation) -> Self {
        let i = self as usize;
        match rotation {
            Rotation::Clockwise => Self::from((i + 1) % 4),
            Rotation::CounterClockwise => Self::from((i + 3) % 4),
        }
    }
}


