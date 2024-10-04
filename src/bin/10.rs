advent_of_code::solution!(10);
use advent_of_code::utils::position::Position;

#[derive(Clone, Debug)]
struct Tile {
    position: Position,
    tile_type: char,
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
            return Position::new(x, i);
        };
    }
    panic!("No starting position found!");
}

fn get_tile_type(position: Position, map: &TileMap) -> u8 {
    let (x, y) = position.get_position();
    map[y].as_bytes()[x]
}

fn get_starting_char(position: &Position, map: &TileMap) -> char {
    let (x, y) = position.get_position();
    if x > 0 {
        let left = get_tile_type(Position::new(x - 1, y), map);
        match left {
            b'-' | b'F' | b'L' => {
                if let Some(_type) = get_if_j_or_7(&position, &map) {
                    return _type;
                }
                if x + 1 < map[0].len() {
                    let right = get_tile_type(Position::new(x + 1, y), &map);
                    if right == b'-' || right == b'7' || right == b'J' {
                        return '-';
                    }
                }
                panic!("Invalid input!");
            }
            _ => {
                let below = get_tile_type(Position::new(x, y + 1), &map);
                match below {
                    b'L' | b'|' | b'J' => {
                        if x + 1 < map[0].len() {
                            let right = get_tile_type(Position::new(x + 1, y), &map);
                            if right == b'-' || right == b'7' || right == b'J' {
                                return 'F';
                            }
                        }
                        if y > 0 {
                            let above = get_tile_type(Position::new(x, y - 1), &map);
                            if above == b'7' || above == b'F' || above == b'|' {
                                return '7';
                            }
                        }
                        panic!("Invalid input!");
                    }
                    _ => {
                        if y > 0 {
                            let above = get_tile_type(Position::new(x, y - 1), &map);
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
        let right = get_tile_type(Position::new(x + 1, y), map);
        match right {
            b'7' | b'-' | b'J' => return get_if_j_or_7(&position, &map).unwrap(),
            _ => panic!("Invalid input"),
        }
    }
}

fn get_if_j_or_7(position: &Position, map: &TileMap) -> Option<char> {
    let (x, y) = position.get_position();
    if y > 0 {
        let above = get_tile_type(Position::new(x, y - 1), &map);
        if above == b'F' || above == b'7' || above == b'|' {
            return Some('J');
        }
    }
    if y + 1 < map.len() {
        let below = get_tile_type(Position::new(x, y + 1), &map);
        if below == b'J' || below == b'L' || below == b'|' {
            return Some('7');
        }
    }
    None
}
fn next_position(previous_pos: &Position, current: &Tile) -> Position {
    let current_position = current.position;
    match current.tile_type {
        '|' => current_position.move_vertical(previous_pos),
        '-' => current_position.move_horizontal(previous_pos),
        'L' => current_position.move_north_east(previous_pos),
        'J' => current_position.move_north_west(previous_pos),
        '7' => current_position.move_south_west(previous_pos),
        'F' => current_position.move_south_east(previous_pos),
        _ => panic!("Impossible type :{:?}", current.tile_type),
    }
}

fn find_loop(map: &TileMap) -> Vec<Position> {
    let starting_position = get_starting_position(map);
    let mut loop_positions: Vec<Position> = vec![starting_position];
    let mut previous_position = starting_position;
    let mut current_tile = Tile {
        position: starting_position,
        tile_type: get_starting_char(&starting_position, map),
    };
    loop {
        let next_position = next_position(&previous_position, &current_tile);
        if next_position == starting_position {
            return loop_positions;
        }
        previous_position = current_tile.position;
        let tile_type = get_tile_type(next_position, map) as char;
        current_tile = Tile {
            position: next_position,
            tile_type,
        };
        loop_positions.push(current_tile.position.clone());
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = create_map(input);
    let resulting_loop = find_loop(&map);
    Some((resulting_loop.len() / 2) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = create_map(input);
    let width = map[0].len();
    let closed_loop = find_loop(&map);
    let mut count = 0;
    let mut is_inside = false;
    for y in 0..map.len() {
        let mut half_cross = b'.';
        for x in 0..width {
            let current_position = Position::new(x, y);
            let mut tile_type = get_tile_type(current_position, &map);
            if tile_type == b'S' {
                tile_type = get_starting_char(&current_position, &map) as u8;
            }

            if (tile_type == b'L' || tile_type == b'F') && closed_loop.contains(&current_position) {
                half_cross = tile_type;
            }

            if (half_cross == b'L' && tile_type == b'J')
                || (half_cross == b'F' && tile_type == b'7')
                    && closed_loop.contains(&current_position)
            {
                half_cross = tile_type;
            }

            if (half_cross == b'L' && tile_type == b'7')
                || (half_cross == b'F' && tile_type == b'J')
                    && closed_loop.contains(&current_position)
            {
                is_inside = !is_inside;
                half_cross = tile_type;
            }

            if (tile_type == b'|') && closed_loop.contains(&current_position) {
                is_inside = !is_inside;
            }

            if is_inside && !closed_loop.contains(&current_position) {
                count += 1;
            }
        }
    }

    Some(count)
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_b() {
        let result = part_two(
            "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        );
        assert_eq!(result, Some(10));
    }
}

// #1 Part 1: 6768 (1.9ms)
// #2 Part 1: 6768 (2.5ms) Part 2: 351 (4.0s)
