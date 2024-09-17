advent_of_code::solution!(16);

type CaveMap = Vec<Vec<Space>>;

trait Navigation {
    fn split_vertical(&mut self, pos: Position);
    fn split_horizontal(&mut self, pos: Position);
    fn visit_pos(&mut self, pos: Position, direction: VisitDirection);
    fn light_left_map(&self, pos: Position) -> bool;
    fn move_down(&mut self, prev_pos: Position);
    fn move_up(&mut self, prev_pos: Position);
    fn move_right(&mut self, prev_pos: Position);
    fn move_left(&mut self, prev_pos: Position);
    fn count_visited(&self) -> usize;
    fn has_visited_before(&self, pos: Position, direction: VisitDirection) -> bool;
}

#[derive(Copy, Clone, Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Debug)]
enum SpaceType {
    Empty,
    HorizontalSplitter,
    VerticalSplitter,
    LeftRightMirror,
    LeftDownMirror,
}

#[derive(PartialEq)]
enum VisitDirection {
    Up,
    Down,
    Left,
    Right,
}

struct Space {
    space_type: SpaceType,
    visited: Vec<VisitDirection>,
}

impl SpaceType {
    fn from_char(c: char) -> Option<Space> {
        if let Some(space_type) = match c {
            '.' => Some(SpaceType::Empty),
            '-' => Some(SpaceType::HorizontalSplitter),
            '|' => Some(SpaceType::VerticalSplitter),
            '/' => Some(SpaceType::LeftRightMirror),
            '\\' => Some(SpaceType::LeftDownMirror),
            _ => None,
        } {
            return Some(Space {
                space_type,
                visited: vec![],
            });
        }
        None
    }
}

impl Navigation for CaveMap {
    fn light_left_map(&self, pos: Position) -> bool {
        let max_rows = self.len();
        let max_cols = self[0].len();
        pos.x >= max_cols || pos.y >= max_rows
    }

    fn has_visited_before(&self, pos: Position, direction: VisitDirection) -> bool {
        self[pos.y][pos.x].visited.contains(&direction)
    }

    fn move_down(&mut self, prev_pos: Position) {
        let next_pos = Position {
            x: prev_pos.x,
            y: prev_pos.y + 1,
        };
        println!("Prev: {:?} , Befor light is at: {:?}", prev_pos, next_pos);

        if self.light_left_map(next_pos) {
            return;
        }

        self.visit_pos(next_pos, VisitDirection::Down);

        match self[next_pos.y][next_pos.x].space_type {
            SpaceType::Empty | SpaceType::VerticalSplitter => self.move_down(next_pos),
            SpaceType::HorizontalSplitter => self.split_horizontal(next_pos),
            SpaceType::LeftRightMirror => self.move_left(next_pos),
            SpaceType::LeftDownMirror => self.move_right(next_pos),
        }
    }

    fn move_up(&mut self, prev_pos: Position) {
        // leaves map
        if prev_pos.y == 0 {
            return;
        }

        let next_pos = Position {
            x: prev_pos.x,
            y: prev_pos.y - 1,
        };
        let direction = VisitDirection::Up;

        if self[next_pos.y][next_pos.x].visited.contains(&direction) {
            return;
        }
        self.visit_pos(next_pos, direction);

        match self[next_pos.y][next_pos.x].space_type {
            SpaceType::Empty | SpaceType::VerticalSplitter => self.move_up(next_pos),
            SpaceType::HorizontalSplitter => self.split_horizontal(next_pos),
            SpaceType::LeftRightMirror => self.move_right(next_pos),
            SpaceType::LeftDownMirror => self.move_left(next_pos),
        }
    }

    fn move_right(&mut self, prev_pos: Position) {
        let next_pos = Position {
            x: prev_pos.x + 1,
            y: prev_pos.y,
        };

        if self.light_left_map(next_pos) {
            return;
        }

        let direction = VisitDirection::Right;

        if self[next_pos.y][next_pos.x].visited.contains(&direction) {
            return;
        }
        self.visit_pos(next_pos, direction);

        let space_type = self[next_pos.y][next_pos.x].space_type;
        println!("Space_type: {:?}", space_type);
        match space_type {
            SpaceType::Empty | SpaceType::HorizontalSplitter => self.move_right(next_pos),
            SpaceType::VerticalSplitter => self.split_vertical(next_pos),
            SpaceType::LeftRightMirror => self.move_up(next_pos),
            SpaceType::LeftDownMirror => self.move_down(next_pos),
        }
    }

    fn move_left(&mut self, prev_pos: Position) {
        // leaves map
        if prev_pos.x == 0 {
            return;
        }

        let next_pos = Position {
            x: prev_pos.x - 1,
            y: prev_pos.y,
        };
        let direction = VisitDirection::Left;

        if self[next_pos.y][next_pos.x].visited.contains(&direction) {
            return;
        }
        self.visit_pos(next_pos, direction);

        match self[next_pos.y][next_pos.x].space_type {
            SpaceType::Empty | SpaceType::HorizontalSplitter => self.move_left(next_pos),
            SpaceType::VerticalSplitter => self.split_vertical(next_pos),
            SpaceType::LeftRightMirror => self.move_down(next_pos),
            SpaceType::LeftDownMirror => self.move_up(next_pos),
        }
    }

    fn count_visited(&self) -> usize {
        let mut count = 0;
        self.iter().for_each(|rows| {
            rows.iter().for_each(|s| {
                if s.visited.len() > 0 {
                    count += 1;
                }
            })
        });
        count
    }

    fn visit_pos(&mut self, pos: Position, direction: VisitDirection) {
        let Position { x, y } = pos;

        self[y][x].visited.push(direction)
    }

    fn split_horizontal(&mut self, pos: Position) {
        println!("Split horizontal {:?}", pos);
        self.move_right(pos);
        println!("After right move {:?}", pos);
        self.move_left(pos);
    }

    fn split_vertical(&mut self, pos: Position) {
        self.move_down(pos);
        self.move_up(pos);
    }
}

fn parse_input(input: &str) -> CaveMap {
    let mut map: CaveMap = vec![];
    input.lines().for_each(|s| {
        let mut row = vec![];
        s.chars()
            .for_each(|c| row.push(SpaceType::from_char(c).unwrap()));
        map.push(row);
    });
    map
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut map = parse_input(input);
    let starting_pos = Position { x: 0, y: 0 };
    map.visit_pos(starting_pos, VisitDirection::Right);
    map.move_right(Position { x: 0, y: 0 });
    Some(map.count_visited())
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        let result = part_one(
            ".|.
|.-",
        );
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_split_infinite_iteration() {
        let result = part_one(
            ".-..\\
.|.-/
.|...
..//.",
        );
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
