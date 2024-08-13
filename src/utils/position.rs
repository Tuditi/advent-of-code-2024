pub mod position {
    use num::abs;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Position {
        x: usize,
        y: usize,
    }

    impl Position {
        pub fn new(x: usize, y: usize) -> Self {
            Self { x, y }
        }

        pub fn get_position(&self) -> (usize, usize) {
            (self.x, self.y)
        }

        fn _get_signed_position(&self) -> (isize, isize) {
            (self.x as isize, self.y as isize)
        }

        pub fn get_distance(&self, other: Position) -> usize {
            let (x, y) = self._get_signed_position();
            let (x_other, y_other) = other._get_signed_position();
            let distance = abs(x_other - x) + abs(y_other - y);
            distance as usize
        }

        pub fn move_vertical(&self, previous_pos: &Position) -> Position {
            let mut y_coord = self.y;
            if previous_pos.y < y_coord {
                y_coord += 1;
            } else {
                y_coord -= 1;
            }
            Position {
                x: self.x,
                y: y_coord,
            }
        }

        pub fn move_horizontal(&self, previous_pos: &Position) -> Position {
            let mut x_coord = self.x;
            if previous_pos.x < x_coord {
                x_coord += 1;
            } else {
                x_coord -= 1;
            }
            Position {
                x: x_coord,
                y: self.y,
            }
        }

        pub fn move_north_east(&self, previous_pos: &Position) -> Position {
            let Position { mut y, mut x } = self;
            if previous_pos.y < y {
                x += 1;
            } else {
                y -= 1;
            }
            Position { x, y }
        }

        pub fn move_north_west(&self, previous_pos: &Position) -> Position {
            let Position { mut y, mut x } = self;
            if previous_pos.y < y {
                x -= 1;
            } else {
                y -= 1;
            }
            Position { x, y }
        }

        pub fn move_south_west(&self, previous_pos: &Position) -> Position {
            let Position { mut y, mut x } = self;
            if previous_pos.y > y {
                x -= 1;
            } else {
                y += 1;
            }
            Position { x, y }
        }

        pub fn move_south_east(&self, previous_pos: &Position) -> Position {
            let Position { mut y, mut x } = self;
            if previous_pos.y > y {
                x += 1;
            } else {
                y += 1;
            }
            Position { x, y }
        }
    }
}

#[cfg(test)]
mod tests {
    use position::Position;

    use super::*;

    #[test]
    fn test_get_distance() {
        let point_5 = Position::new(1, 6);
        let point_9 = Position::new(5, 11);
        let result = point_5.get_distance(point_9);
        assert_eq!(result, 9);
        // commutativity
        assert_eq!(result, point_9.get_distance(point_5));

        let point_8 = Position::new(0, 11);
        let result = point_8.get_distance(point_9);
        assert_eq!(result, 5);
    }
}
