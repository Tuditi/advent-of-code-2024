advent_of_code::solution!(17);

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::Hash;
use std::u32;

use advent_of_code::utils::map::*;
use strum::IntoEnumIterator;

type PositionX = (u32, u32);

#[derive(Debug)]
struct Maze {
    map: HashMap<PositionX, u8>,
    ending_point: PositionX,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct ExtendedNode {
    heat: u32,
    position: PositionX,
    counter: u8,
    direction: Option<Direction>,
}

type VisitedKey = (PositionX, u8, Option<Direction>);

impl Ord for ExtendedNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat.cmp(&self.heat).then_with(|| {
            let a = self.position.0 + self.position.1;
            let b = other.position.0 + other.position.1;
            a.cmp(&b)
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
        let mut map: HashMap<PositionX, u8> = HashMap::new();

        input.lines().enumerate().for_each(|(y, l)| {
            l.chars().enumerate().for_each(|(x, c)| {
                map.insert((x as u32, y as u32), c.to_digit(10).unwrap() as u8);
            });
        });

        let end_y = input.lines().count() as u32;
        let end_x = input.lines().next().unwrap().len() as u32;
        let ending_point: PositionX = (end_x - 1, end_y - 1);

        Self { map, ending_point }
    }

    fn get_value(&self, pos: &PositionX) -> u32 {
        *self.map.get(pos).unwrap() as u32
    }

    fn get_next_steps(
        &self,
        prev_direction: &Option<Direction>,
        cur_pos: &PositionX,
        counter: u8,
    ) -> Vec<Direction> {
        match prev_direction {
            // Starting Point
            None => vec![Direction::Right, Direction::Down],
            Some(prev_direction) => {
                let possible_directions = Direction::iter();

                possible_directions
                    .filter(|d| {
                        if *d == prev_direction.opposite() || (counter == 2 && d == prev_direction)
                        {
                            return false;
                        }

                        match *d {
                            Direction::Down => cur_pos.1 != self.ending_point.1,
                            Direction::Up => cur_pos.1 != 0,
                            Direction::Left => cur_pos.0 != 0,
                            Direction::Right => cur_pos.0 != self.ending_point.0,
                        }
                    })
                    .collect()
            }
        }
    }

    fn get_adjacent_nodes(&self, cur_node: ExtendedNode) -> Vec<ExtendedNode> {
        let directions =
            self.get_next_steps(&cur_node.direction, &cur_node.position, cur_node.counter);
        let mut result = vec![];
        for d in directions {
            let next_position = d.get_next_pos_x(&cur_node.position);

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
            position: (0, 0),
            counter: 0,
            direction: None,
        };
        unvisited_vertices.push(node);
        visited_vertices.insert(((0, 0), 0, None), 0);

        while let Some(node) = unvisited_vertices.pop() {
            let key: VisitedKey = (node.position, node.counter, node.direction);
            if node.heat > *visited_vertices.get(&key).unwrap_or(&u32::MAX) {
                continue;
            }

            if node.position == self.ending_point {
                return Some(node.heat);
            }
            visited_vertices.insert(key, node.heat);

            let adjacent_nodes = self.get_adjacent_nodes(node.clone());
            for new_node in adjacent_nodes {
                let new_key = (new_node.position, new_node.counter, new_node.direction);
                if !visited_vertices.contains_key(&new_key)
                    || new_node.heat < visited_vertices[&new_key]
                {
                    unvisited_vertices.push(new_node.clone());
                }
            }
        }

        Some(self.get_result(&visited_vertices))
    }

    fn get_result<'a>(&'a self, visited_vertices: &'a HashMap<VisitedKey, u32>) -> u32 {
        let mut results: BinaryHeap<Reverse<&u32>> = BinaryHeap::new();
        for i in 0..3 {
            for direction in [Direction::Right, Direction::Down] {
                let key = (self.ending_point, i, Some(direction));
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
