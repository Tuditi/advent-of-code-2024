use num::integer::lcm;
use std::collections::HashMap;
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
) -> Option<u64> {
    let mut current_positions: Vec<&str> = map
        .keys()
        .filter(|pos| (pos).ends_with('A'))
        .cloned()
        .collect();
    let mut count = 0;
    let mut found_pos: Vec<&str> = vec![];
    let mut iteration_count: Vec<u64> = vec![];
    for c in instructions.cycle() {
        if current_positions.is_empty() {
            break;
        }
        count += 1;

        current_positions.iter_mut().for_each(|ghost_pos| {
            let directions = map.get(ghost_pos).unwrap();
            *ghost_pos = get_next_pos(c, directions);

            if ghost_pos.ends_with('Z') {
                iteration_count.push(count);
                found_pos.push(ghost_pos);
            }
        });

        remove_found_pos(&mut found_pos, &mut current_positions)
    }
    iteration_count.into_iter().reduce(|acc, el| lcm(acc, el))
}

fn remove_found_pos(found_pos: &mut Vec<&str>, current_positions: &mut Vec<&str>) {
    if !found_pos.is_empty() {
        found_pos.into_iter().for_each(|pos| {
            if let Some(index) = current_positions.iter().position(|p| p == pos) {
                current_positions.remove(index);
            }
        });
        *found_pos = vec![];
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (instructions, map) = parse_input(input);
    go_through_desert("AAA", instructions, map)
}

pub fn part_two(input: &str) -> Option<u64> {
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
// #2 Part 1: 14681 (16.0ms) Part 2: 14321394058031 (103.1ms)
