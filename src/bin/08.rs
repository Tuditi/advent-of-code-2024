use num::integer::lcm;
use rayon::prelude::*;
use std::collections::HashMap;
use std::time::Instant;
advent_of_code::solution!(8);

type Direction<'a> = Vec<&'a str>;

fn parse_input<'a>(
    input: &'a str,
) -> (
    impl Iterator<Item = char> + std::clone::Clone + 'a,
    HashMap<&str, Direction<'a>>,
) {
    let (first_line, other_lines) = input.split_once('\n').unwrap();
    let desert_map = create_desert_map(other_lines);
    return (first_line.chars(), desert_map);
}

fn create_desert_map(input: &str) -> HashMap<&str, Direction> {
    let mut desert_map: HashMap<&str, Direction> = HashMap::new();
    input.lines().filter(|l| !l.is_empty()).for_each(|l| {
        let (current_pos, next_pos) = l.split_once('=').unwrap();
        let directions = next_pos.split_once(',').unwrap();
        let directions = [directions.0, directions.1]
            .iter_mut()
            .map(|direction| direction.trim_matches(|c: char| !c.is_alphanumeric()))
            .collect();
        desert_map.insert(current_pos.trim(), directions);
    });
    desert_map
}

fn get_next_pos<'a>(input: char, directions: &Direction<'a>) -> &'a str {
    match input {
        'L' => directions[0],
        'R' => directions[1],
        _ => panic!("Invalid instruction"),
    }
}

fn go_through_desert(
    start_position: &str,
    instructions: impl Iterator<Item = char> + std::clone::Clone,
    map: HashMap<&str, Direction>,
) -> Option<u32> {
    let mut count = 0;
    let mut current_pos = start_position;
    for c in instructions.cycle() {
        count += 1;
        let directions = map.get(current_pos).unwrap();
        current_pos = get_next_pos(c, directions);
        if current_pos == "ZZZ" {
            return Some(count);
        }
    }
    unreachable!()
}

fn ghost_through_desert(
    instructions: impl Iterator<Item = char> + std::clone::Clone,
    map: HashMap<&str, Direction>,
) -> Option<u32> {
    let mut start_positions: Vec<&str> = map
        .keys()
        .filter(|pos| (pos).ends_with('A'))
        .cloned()
        .collect();
    let mut count = 0;
    // let mut iteration_count: Vec<(&str, u64)> = vec![];
    // let mut prev_iteration_count: Vec<(&str, u64)> = vec![];
    'instruction: for c in instructions.cycle() {
        count += 1;
        start_positions.iter_mut().for_each(|pos| {
            let directions = map.get(pos).unwrap();
            *pos = get_next_pos(c, directions);
        });

        if start_positions.iter().any(|el| !el.ends_with('Z')) {
            continue 'instruction;
        } else {
            return Some(count);
        }
    }
    unreachable!()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (instructions, map) = parse_input(input);
    go_through_desert("AAA", instructions, map)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (instructions, map) = parse_input(input);
    ghost_through_desert(instructions, map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    // #[bench]
    // fn bench_add_two(b: &mut Bencher) {
    //     part_two(&advent_of_code::template::read_file("examples", DAY));
    // }
}

// #1 (24.4ms)
