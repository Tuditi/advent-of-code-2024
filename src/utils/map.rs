use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position {
            x: x as isize,
            y: y as isize,
        }
    }
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

#[derive(Copy, Clone, Eq, Hash, Debug, PartialEq, EnumIter)]
pub enum Direction {
    Down,
    Right,
    Left,
    Up,
}

impl From<Direction> for usize {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
        }
    }
}

struct DirectionCollection(Vec<Direction>);
impl FromIterator<Direction> for DirectionCollection {
    fn from_iter<T: IntoIterator<Item = Direction>>(iter: T) -> Self {
        DirectionCollection(iter.into_iter().collect())
    }
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            &Direction::Down => Direction::Up,
            &Direction::Up => Direction::Down,
            &Direction::Left => Direction::Right,
            &Direction::Right => Direction::Left,
        }
    }

    pub fn get_next_pos(&self, cur_pos: &Position) -> Position {
        match &self {
            Direction::Up => cur_pos.up(),
            Direction::Down => cur_pos.down(),
            Direction::Left => cur_pos.left(),
            Direction::Right => cur_pos.right(),
        }
    }
}
