#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

pub struct Map {}

pub trait Navigate {
    fn down(self) -> Self;
    fn up(self) -> Self;
    fn right(self) -> Self;
    fn left(self) -> Self;
}

impl Navigate for Position {
    fn down(self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn up(self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn right(self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn left(self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
}
