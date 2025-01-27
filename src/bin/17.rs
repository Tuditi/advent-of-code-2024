advent_of_code::solution!(17);

use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;
use std::collections::{BinaryHeap, HashSet};
use std::u32;

use advent_of_code::utils::map::*;
use strum::IntoEnumIterator;

#[derive(Debug)]
struct Maze {
    map: HashMap<Position, u8>,
    ending_point: Position,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct ExtendedNode {
    heat: u32,
    position: Position,
    counter: u8,
    direction: Option<Direction>,
}

type VisitedKey = (Position, u8, Option<Direction>);
type ConstraintFunction = dyn Fn(&ExtendedNode, &Direction) -> bool;

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
        let mut map: HashMap<Position, u8> = HashMap::new();

        input.lines().enumerate().for_each(|(y, l)| {
            l.chars().enumerate().for_each(|(x, c)| {
                map.insert((x, y), c.to_digit(10).unwrap() as u8);
            });
        });

        let end_y = input.lines().count();
        let end_x = input.lines().next().unwrap().len();
        let ending_point: Position = (end_x - 1, end_y - 1);

        Self { map, ending_point }
    }

    pub fn get_value(&self, pos: &Position) -> u32 {
        *self.map.get(pos).unwrap() as u32
    }

    fn dijkstra_algorithm(&self, constraint: &ConstraintFunction) -> Option<u32> {
        let mut visited_vertices: HashMap<VisitedKey, u32> = HashMap::new();
        let mut unvisited_vertices: BinaryHeap<ExtendedNode> = BinaryHeap::new();
        let mut unvisited_keys: HashSet<VisitedKey> = HashSet::new();

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

            let adjacent_nodes = self.get_adjacent_nodes(node.clone(), constraint);
            for new_node in adjacent_nodes {
                let new_key = (new_node.position, new_node.counter, new_node.direction);
                if unvisited_keys.contains(&new_key) {
                    continue;
                }

                if !visited_vertices.contains_key(&new_key)
                    || new_node.heat < visited_vertices[&new_key]
                {
                    unvisited_vertices.push(new_node.clone());
                    unvisited_keys.insert(new_key);
                }
            }
        }

        Some(self.get_result(&visited_vertices))
    }

    fn get_adjacent_nodes(
        &self,
        cur_node: ExtendedNode,
        constraint: &ConstraintFunction,
    ) -> Vec<ExtendedNode> {
        let directions = self.get_next_steps(&cur_node, constraint);
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

    fn get_next_steps(
        &self,
        cur_node: &ExtendedNode,
        constraint: &ConstraintFunction,
    ) -> Vec<Direction> {
        let prev_direction = cur_node.direction;
        let cur_pos = cur_node.position;
        match prev_direction {
            // Starting Point
            None => vec![Direction::Down, Direction::Right],
            Some(prev_direction) => {
                let possible_directions = Direction::iter();

                let res = possible_directions
                    .filter(|d| {
                        if *d == prev_direction.opposite() || constraint(cur_node, d) {
                            return false;
                        }

                        match *d {
                            Direction::Down => cur_pos.1 != self.ending_point.1,
                            Direction::Up => cur_pos.1 != 0,
                            Direction::Left => cur_pos.0 != 0,
                            Direction::Right => cur_pos.0 != self.ending_point.0,
                        }
                    })
                    .collect();
                res
            }
        }
    }

    fn get_result<'a>(&'a self, visited_vertices: &'a HashMap<VisitedKey, u32>) -> u32 {
        let mut results: BinaryHeap<Reverse<&u32>> = BinaryHeap::new();
        for i in 0..3 {
            for direction in [Direction::Right, Direction::Down] {
                let key = (self.ending_point, i, Some(direction));
                results.push(Reverse(visited_vertices.get(&key).unwrap_or(&u32::MAX)))
            }
        }
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
    let constraint = |node: &ExtendedNode, next_direction: &Direction| {
        let counter = node.counter;
        let prev_direction = node.direction.unwrap();
        counter == 2 && *next_direction == prev_direction
    };
    maze.dijkstra_algorithm(&constraint)
}

pub fn part_two(input: &str) -> Option<u32> {
    let maze: Maze = Maze::new(input);
    let ending_point = maze.ending_point;
    let constraint = move |node: &ExtendedNode, next_direction: &Direction| {
        let counter = node.counter;
        let prev_direction = node.direction.unwrap();

        if next_direction.get_next_pos(&node.position) == ending_point {
            // <=, because we will do another move to land in the good spot
            return counter <= 3 || *next_direction != prev_direction;
        }

        let not_enough_moves = counter < 3 && *next_direction != prev_direction;
        let reached_maximum_moves = counter == 9 && *next_direction == prev_direction;

        not_enough_moves || reached_maximum_moves
    };
    maze.dijkstra_algorithm(&constraint)
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
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two_b() {
        let maze_str = "111111111111
999999999991
999999999991
999999999991
999999999991";
        let result = part_two(&maze_str);
        assert_eq!(result, Some(71));
    }
}

// Part 1: 1001 (878.9ms)  Part 2: 1197 (2.5s)
