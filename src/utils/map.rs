use num::abs;
use strum_macros::EnumIter;

pub type Position = (usize, usize);

#[derive(Copy, Clone, Eq, Hash, Debug, PartialEq, EnumIter)]
#[repr(u8)]
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

pub struct Map {}

impl Map {
    fn get_signed_position(position: &Position) -> (isize, isize) {
        (position.0 as isize, position.1 as isize)
    }

    pub fn get_distance(a: Position, b: Position) -> usize {
        let (x, y) = Self::get_signed_position(&a);
        let (x_other, y_other) = Self::get_signed_position(&b);
        let distance = abs(x_other - x) + abs(y_other - y);
        distance as usize
    }

    pub fn move_vertical(current_pos: &Position, previous_pos: &Position) -> Position {
        let y_coord = current_pos.1;
        if previous_pos.1 < y_coord {
            Direction::down(current_pos)
        } else {
            Direction::up(current_pos)
        }
    }

    pub fn move_horizontal(current_pos: &Position, previous_pos: &Position) -> Position {
        if previous_pos.0 < current_pos.0 {
            Direction::right(current_pos)
        } else {
            Direction::left(current_pos)
        }
    }

    pub fn move_north_east(current_pos: &Position, previous_pos: &Position) -> Position {
        let (_x, y) = current_pos;
        if previous_pos.1 < *y {
            Direction::right(current_pos)
        } else {
            Direction::up(current_pos)
        }
    }

    pub fn move_north_west(current_pos: &Position, previous_pos: &Position) -> Position {
        let (_x, y) = current_pos;
        if previous_pos.1 < *y {
            Direction::left(current_pos)
        } else {
            Direction::up(current_pos)
        }
    }

    pub fn move_south_west(current_pos: &Position, previous_pos: &Position) -> Position {
        let y = current_pos.1;
        if previous_pos.1 > y {
            Direction::left(current_pos)
        } else {
            Direction::down(current_pos)
        }
    }

    pub fn move_south_east(current_pos: &Position, previous_pos: &Position) -> Position {
        let y = current_pos.1;
        if previous_pos.1 > y {
            Direction::right(current_pos)
        } else {
            Direction::down(current_pos)
        }
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

    pub fn get_next_pos_x(&self, cur_pos: &Position) -> Position {
        match &self {
            Direction::Up => Direction::up(cur_pos),
            Direction::Down => Direction::down(cur_pos),
            Direction::Left => Direction::left(cur_pos),
            Direction::Right => Direction::right(cur_pos),
        }
    }

    pub fn up(cur_pos: &Position) -> Position {
        (cur_pos.0, cur_pos.1 - 1)
    }

    pub fn down(cur_pos: &Position) -> Position {
        (cur_pos.0, cur_pos.1 + 1)
    }

    pub fn left(cur_pos: &Position) -> Position {
        (cur_pos.0 - 1, cur_pos.1)
    }

    pub fn right(cur_pos: &Position) -> Position {
        (cur_pos.0 + 1, cur_pos.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_distance() {
        let point_5: Position = (1, 6);
        let point_9: Position = (5, 11);
        let result = Map::get_distance(point_5, point_9);
        assert_eq!(result, 9);
        // commutativity
        assert_eq!(result, Map::get_distance(point_5, point_9));

        let point_8: Position = (0, 11);
        let result = Map::get_distance(point_8, point_9);
        assert_eq!(result, 5);
    }
}
