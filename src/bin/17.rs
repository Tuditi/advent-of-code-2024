advent_of_code::solution!(17);

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};

use advent_of_code::utils::map::*;
use strum::IntoEnumIterator;

#[derive(Debug)]
struct Maze {
    map: Vec<Vec<u8>>,
    ending_point: Position,
    visited_vertices: HashMap<Position, u32>,
    file: File,
}

impl Maze {
    fn new(input: &str) -> Self {
        std::fs::remove_file("test.txt").unwrap();
        let file = File::create("test.txt").unwrap();

        let mut map = vec![];
        input.lines().for_each(|l| {
            let mut row: Vec<u8> = vec![];
            l.chars()
                .for_each(|c| row.push(c.to_digit(10).unwrap() as u8));
            map.push(row);
        });

        let ending_point = Position {
            x: (map[0].len() - 1) as isize,
            y: (map.len() - 1) as isize,
        };

        let mut visited_vertices: HashMap<Position, u32> = HashMap::new();
        visited_vertices.insert(Position::new(0, 0), 0);

        Self {
            map,
            ending_point,
            visited_vertices,
            file,
        }
    }

    fn get_value(&self, pos: &Position) -> u32 {
        self.map[pos.y as usize][pos.x as usize] as u32
    }

    fn is_last_step(&self, pos: &Position) -> bool {
        *pos == self.ending_point
    }

    fn result(&self) -> Option<u32> {
        let result = self.visited_vertices.get(&self.ending_point)?;
        Some(*result)
    }

    fn get_next_steps(
        &self,
        prev_direction: &Option<&Direction>,
        cur_pos: &Position,
        counter: &i8,
    ) -> Vec<Direction> {
        // Starting point
        if prev_direction.is_none() {
            return vec![Direction::Down, Direction::Right];
        }

        let prev_direction = prev_direction.unwrap();
        let possible_directions = Direction::iter();

        let remove_left = cur_pos.x == 0;
        let remove_up = cur_pos.y == 0;

        let remove_right = cur_pos.x == self.ending_point.x;
        let remove_down = cur_pos.y == self.ending_point.y;

        possible_directions
            .filter(|d| {
                if *d == prev_direction.opposite() || (*counter == 2 && d == prev_direction) {
                    return false;
                }

                match *d {
                    Direction::Down => !remove_down,
                    Direction::Up => !remove_up,
                    Direction::Left => !remove_left,
                    Direction::Right => !remove_right,
                }
            })
            .collect()
    }

    fn check_next_pos(
        &mut self,
        prev_direction: Option<&Direction>,
        cur_pos: &Position,
        counter: &i8,
    ) {
        let next_moves = self.get_next_steps(&prev_direction, &cur_pos, &counter);
        let current_heat_acc = self.visited_vertices.get(cur_pos).unwrap().clone();

        write!(self.file, "Next moves: {:?}\n", next_moves).unwrap();
        for move_direction in next_moves.iter() {
            let next_pos = move_direction.get_next_pos(&cur_pos);
            let pos_value = self.get_value(&next_pos);
            // let heat_acc = self.visited_vertices.get(&next_pos).unwrap_or(&199999);
            // if *heat_acc < current_heat_acc + pos_value {
            //     continue;
            // }
            let new_heat = current_heat_acc.clone() + pos_value;
            self.visited_vertices.insert(next_pos, new_heat);
            write!(self.file, "Prev position: {:?}\n", cur_pos).unwrap();
            write!(self.file, "Inserted at {:?} heat: {new_heat}\n", next_pos).unwrap();

            if self.is_last_step(&next_pos) || *counter == 2 {
                continue;
            } else {
                let new_counter = match prev_direction {
                    Some(direction) => {
                        if move_direction == direction {
                            counter + 1
                        } else {
                            0
                        }
                    }
                    None => 0,
                };

                self.check_next_pos(Some(move_direction), &next_pos, &new_counter);
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut maze: Maze = Maze::new(input);
    let current_pos = Position { x: 0, y: 0 };
    let counter = 0;
    maze.check_next_pos(None, &current_pos, &counter);
    maze.result()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smaller_size() {
        let maze_str = "2413
3215
3255
3446";
        let result = part_one(&maze_str);
        assert_eq!(result, Some(21))
    }
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
