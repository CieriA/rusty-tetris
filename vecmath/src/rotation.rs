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

impl Direction {
    /// Rotate the direction by 90 degrees based on input.
    pub fn rotate(&self, rotation: Rotation) -> Self {
        match rotation {
            Rotation::Clockwise => self.rotate_clock(),
            Rotation::CounterClockwise => self.rotate_counter_clock(),
        }
    }
    /// Rotate the direction by 90 degrees clockwise.
    fn rotate_clock(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    /// Rotate the direction by 90 degrees counter-clockwise.
    fn rotate_counter_clock(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}


