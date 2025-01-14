advent_of_code::solution!(17);

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::u32;

use advent_of_code::utils::map::*;
use strum::IntoEnumIterator;

#[derive(Debug)]
struct Maze {
    map: Vec<Vec<u8>>,
    ending_point: Position,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct ExtendedNode {
    heat: u32,
    position: Position,
    counter: u8,
    direction: Option<Direction>,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct VisitedKey {
    position: Position,
    counter: u8,
    direction: Option<Direction>,
}

impl Ord for ExtendedNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat.cmp(&self.heat).then_with(|| {
            let x = self.position.x + self.position.y;
            let y = other.position.x + other.position.y;
            x.cmp(&y)
        })
    }
}

impl PartialOrd for ExtendedNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Maze {
    fn new(input: &str) -> Self {
        let mut map = vec![];
        input.lines().for_each(|l| {
            let mut row: Vec<u8> = vec![];
            l.chars().for_each(|c| {
                row.push(c.to_digit(10).unwrap() as u8);
            });
            map.push(row);
        });

        let ending_point = Position {
            x: (map[0].len() - 1) as isize,
            y: (map.len() - 1) as isize,
        };

        Self { map, ending_point }
    }

    fn get_value(&self, pos: &Position) -> u32 {
        self.map[pos.y as usize][pos.x as usize] as u32
    }

    fn get_next_steps(
        &self,
        prev_direction: &Option<Direction>,
        cur_pos: &Position,
        counter: u8,
    ) -> Vec<Direction> {
        // Starting point
        if prev_direction.is_none() {
            return vec![Direction::Right, Direction::Down];
        }

        let prev_direction = prev_direction.unwrap();
        let possible_directions = Direction::iter();

        let remove_left = cur_pos.x == 0;
        let remove_up = cur_pos.y == 0;

        let remove_right = cur_pos.x == self.ending_point.x;
        let remove_down = cur_pos.y == self.ending_point.y;

        possible_directions
            .filter(|d| {
                if *d == prev_direction.opposite() || (counter == 2 && *d == prev_direction) {
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

    fn get_adjacent_nodes(&self, cur_node: ExtendedNode) -> Vec<ExtendedNode> {
        let directions =
            self.get_next_steps(&cur_node.direction, &cur_node.position, cur_node.counter);
        let mut result = vec![];
        for d in directions {
            let next_position = d.get_next_pos(&cur_node.position);

            let adj_node = ExtendedNode {
                heat: cur_node.heat + self.get_value(&next_position),
                position: next_position,
                counter: get_new_counter(&cur_node.direction, &d, cur_node.counter),
                direction: Some(d),
            };

            result.push(adj_node)
        }
        result
    }

    fn dijkstra_algorithm(&self) -> Option<u32> {
        let mut visited_vertices: HashMap<VisitedKey, u32> = HashMap::new();
        let mut unvisited_vertices: BinaryHeap<ExtendedNode> = BinaryHeap::new();

        let node = ExtendedNode {
            heat: 0,
            position: Position::new(0, 0),
            counter: 0,
            direction: None,
        };
        unvisited_vertices.push(node);
        visited_vertices.insert(
            VisitedKey {
                position: Position::new(0, 0),
                counter: 0,
                direction: None,
            },
            0,
        );

        while let Some(node) = unvisited_vertices.pop() {
            let key = VisitedKey {
                position: node.position,
                counter: node.counter,
                direction: node.direction,
            };
            if node.heat > *visited_vertices.get(&key).unwrap_or(&u32::MAX) {
                continue;
            }
            visited_vertices.insert(key, node.heat);

            let adjacent_nodes = self.get_adjacent_nodes(node.clone());
            for new_node in adjacent_nodes {
                unvisited_vertices.push(new_node.clone());
            }
        }

        Some(self.get_result(&visited_vertices))
    }

    fn get_result<'a>(&'a self, visited_vertices: &'a HashMap<VisitedKey, u32>) -> u32 {
        let mut results: BinaryHeap<Reverse<&u32>> = BinaryHeap::new();
        for i in 0..3 {
            for direction in [Direction::Right, Direction::Down] {
                let key = VisitedKey {
                    position: self.ending_point,
                    counter: i,
                    direction: Some(direction),
                };
                results.push(Reverse(visited_vertices.get(&key).unwrap_or(&u32::MAX)))
            }
        }
        println!("Results :{:?}", results);
        match results.pop() {
            Some(Reverse(result)) => *result,
            None => panic!("He"),
        }
    }
}

fn get_new_counter(
    prev_direction: &Option<Direction>,
    current_direction: &Direction,
    counter: u8,
) -> u8 {
    match *prev_direction {
        Some(direction) => {
            if *current_direction == direction {
                counter + 1
            } else {
                0
            }
        }
        None => 0,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let maze: Maze = Maze::new(input);
    maze.dijkstra_algorithm()
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

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
