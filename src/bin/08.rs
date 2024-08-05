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
) -> u64 {
    let mut current_positions: Vec<&str> = map
        .keys()
        .filter(|pos| (pos).ends_with('A'))
        .cloned()
        .collect();
    let mut count = 0;
    let mut iteration_count: Vec<(&str, u64)> = vec![];
    let mut prev_iteration_count: Vec<(&str, u64)> = vec![];
    for c in instructions.cycle() {
        if current_positions.is_empty() {
            break;
        }
        count += 1;

        // println!("{count}");
        println!("Before: {:?}", current_positions);
        current_positions.iter_mut().for_each(|ghost_pos| {
            let directions = map.get(ghost_pos).unwrap();
            *ghost_pos = get_next_pos(c, directions);

            if ghost_pos.ends_with('Z') {
                iteration_count.push((ghost_pos, count));
            }
        });
        println!("After: {:?}", current_positions);

        if prev_iteration_count != iteration_count {
            println!("{:?}", iteration_count);
            iteration_count.iter().for_each(|i| {
                if let Ok(el) = current_positions.binary_search(&i.0) {
                    current_positions.remove(el);
                }
            });
            prev_iteration_count = iteration_count.clone();
        }
        // panic!("Oei");
        // println!("Test position: {:?}", positions);

        // println!("Cur position: {:?}", current_pos);
    }
    iteration_count
        .into_iter()
        .reduce(|acc, el| (acc.0, lcm(acc.1, el.1)))
        .unwrap()
        .1
}

pub fn part_one(input: &str) -> Option<u32> {
    let (instructions, map) = parse_input(input);
    go_through_desert("AAA", instructions, map)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (instructions, map) = parse_input(input);
    Some(ghost_through_desert(instructions, map))
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
