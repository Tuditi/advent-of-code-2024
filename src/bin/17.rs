advent_of_code::solution!(17);

use advent_of_code::utils::map::*;

#[derive(Debug)]
struct Maze {
    map: Vec<Vec<u8>>,
    ending_point: Position,
}

impl Maze {
    fn new(input: &str) -> Self {
        let mut map = vec![];
        input.lines().for_each(|l| {
            let mut row: Vec<u8> = vec![];
            l.chars().for_each(|c| row.push(c as u8));
            map.push(row);
        });

        let ending_point = Position {
            x: (map[0].len() - 1) as isize,
            y: (map.len() - 1) as isize,
        };
        Self { map, ending_point }
    }

    fn get_value(self: &Self, pos: &Position) -> u32 {
        self.map[pos.y as usize][pos.x as usize] as u32
    }
}

fn find_next_position(
    maze: &Maze,
    prev_pos: &Position,
    current_pos: &Position,
    current_heat_score: u32,
    prev_minimal_heat_score: u32,
    counter: u8,
) -> u32 {
    if *current_pos == maze.ending_point {
        // Finish
        return current_heat_score;
    }

    if prev_minimal_heat_score < current_heat_score + maze.get_value(current_pos) {
        // Wrong path
        return current_heat_score;
    }

    if prev_pos.x == current_pos.x && counter < 2 {
        if prev_pos.y > current_pos.y {}
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let maze = Maze::new(input);
    println!("{:?}", maze);
    let starting_point = Position { x: 0, y: 0 };
    let mut counter = 0;

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
