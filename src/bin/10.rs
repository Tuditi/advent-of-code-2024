advent_of_code::solution!(10);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug)]
struct Tile {
    position: Position,
    tile_type: char,
}

impl Tile {
    fn move_vertical(&self, previous_pos: &Position) -> Position {
        let mut y_coord = self.position.y;
        if previous_pos.y < y_coord {
            y_coord += 1;
        } else {
            y_coord -= 1;
        }
        Position {
            x: self.position.x,
            y: y_coord,
        }
    }

    fn move_horizontal(&self, previous_pos: &Position) -> Position {
        let mut x_coord = self.position.x;
        if previous_pos.x < x_coord {
            x_coord += 1;
        } else {
            x_coord -= 1;
        }
        Position {
            x: x_coord,
            y: self.position.y,
        }
    }

    fn move_north_east(&self, previous_pos: &Position) -> Position {
        let Position { mut y, mut x } = self.position;
        if previous_pos.y < y {
            x += 1;
        } else {
            y -= 1;
        }
        Position { x, y }
    }

    fn move_north_west(&self, previous_pos: &Position) -> Position {
        let Position { mut y, mut x } = self.position;
        if previous_pos.y < y {
            x -= 1;
        } else {
            y -= 1;
        }
        Position { x, y }
    }

    fn move_south_west(&self, previous_pos: &Position) -> Position {
        let Position { mut y, mut x } = self.position;
        if previous_pos.y > y {
            x -= 1;
        } else {
            y += 1;
        }
        Position { x, y }
    }

    fn move_south_east(&self, previous_pos: &Position) -> Position {
        let Position { mut y, mut x } = self.position;
        if previous_pos.y > y {
            x += 1;
        } else {
            y += 1;
        }
        Position { x, y }
    }
}

type TileMap<'a> = Vec<&'a str>;

fn create_map<'a>(input: &'a str) -> TileMap<'a> {
    let mut map: TileMap = vec![];
    input.lines().for_each(|l| map.push(l));
    map
}

fn get_starting_position(map: &TileMap) -> Position {
    for i in 0..map.len() {
        if let Some(x) = map[i].find(|c| c == 'S') {
            return Position { x, y: i };
        };
    }
    panic!("No starting position found!");
}

fn get_tile_type(position: &Position, map: &TileMap) -> u8 {
    let Position { x, y } = *position;
    map[y].as_bytes()[x]
}

fn get_starting_char(position: &Position, map: &TileMap) -> char {
    let Position { x, y } = *position;
    if x > 0 {
        let left = get_tile_type(&Position { x: x - 1, y }, map);
        match left {
            b'-' | b'F' | b'L' => {
                if let Some(_type) = get_if_j_or_7(&position, &map) {
                    return _type;
                }
                if x + 1 < map[0].len() {
                    let right = get_tile_type(&Position { x: x + 1, y }, &map);
                    if right == b'-' || right == b'7' || right == b'J' {
                        return '-';
                    }
                }
                panic!("Invalid input!");
            }
            _ => {
                let below = get_tile_type(&Position { x, y: y + 1 }, &map);
                match below {
                    b'L' | b'|' | b'J' => {
                        if x + 1 < map[0].len() {
                            let right = get_tile_type(&Position { x: x + 1, y }, &map);
                            if right == b'-' || right == b'7' || right == b'J' {
                                return 'F';
                            }
                        }
                        if y > 0 {
                            let above = get_tile_type(&Position { x, y: y - 1 }, &map);
                            if above == b'7' || above == b'F' || above == b'|' {
                                return '7';
                            }
                        }
                        panic!("Invalid input!");
                    }
                    _ => {
                        if y > 0 {
                            let above = get_tile_type(&Position { x, y: y - 1 }, &map);
                            if above == b'7' || above == b'F' || above == b'|' {
                                'L';
                            }
                        }
                        panic!("Invalid Input!");
                    }
                };
            }
        };
    } else {
        let right = get_tile_type(&Position { x: x + 1, y }, map);
        match right {
            b'7' | b'-' | b'J' => return get_if_j_or_7(&position, &map).unwrap(),
            _ => panic!("Invalid input"),
        }
    }
}

fn get_if_j_or_7(position: &Position, map: &TileMap) -> Option<char> {
    let Position { x, y } = *position;
    if y > 0 {
        let above = get_tile_type(&Position { x, y: y - 1 }, &map);
        if above == b'F' || above == b'7' || above == b'|' {
            return Some('J');
        }
    }
    if y + 1 < map.len() {
        let below = get_tile_type(&Position { x, y: y + 1 }, &map);
        if below == b'J' || below == b'L' || below == b'|' {
            return Some('7');
        }
    }
    None
}
fn next_position(previous_pos: &Position, current: &Tile) -> Position {
    match current.tile_type {
        '|' => current.move_vertical(previous_pos),
        '-' => current.move_horizontal(previous_pos),
        'L' => current.move_north_east(previous_pos),
        'J' => current.move_north_west(previous_pos),
        '7' => current.move_south_west(previous_pos),
        'F' => current.move_south_east(previous_pos),
        _ => panic!("Impossible type :{:?}", current.tile_type),
    }
}

fn find_loop_length(starting_position: Position, map: TileMap) -> u32 {
    let mut count = 1;
    let mut previous_position = starting_position.clone();
    let mut current_tile = Tile {
        position: starting_position.clone(),
        tile_type: get_starting_char(&starting_position, &map),
    };
    loop {
        let next_position = next_position(&previous_position, &current_tile);
        if next_position == starting_position {
            return count / 2;
        }
        previous_position = current_tile.position;
        let tile_type = get_tile_type(&next_position, &map) as char;
        current_tile = Tile {
            position: next_position,
            tile_type,
        };
        count += 1;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = create_map(input);
    let starting_position = get_starting_position(&map);
    Some(find_loop_length(starting_position, map))
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = create_map(input);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_simple() {
        let result = part_one(
            ".....
.S-7.
.|.|.
.L-J.
.....
",
        );
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
",
        );
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_a() {
        let result = part_two(
            ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        );
        assert_eq!(result, Some(10));
    }
}

// #1 Part 1: 6768 (1.9ms)
