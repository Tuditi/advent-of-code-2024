use std::collections::HashMap;

advent_of_code::solution!(8);

type Direction<'a> = (&'a str, &'a str);

fn parse_input<'a>(
    input: &'a str,
) -> (
    impl Iterator<Item = i8> + std::clone::Clone + 'a,
    HashMap<&str, Direction<'a>>,
) {
    let (first_line, other_lines) = input.split_once('\n').unwrap();
    let mut desert_map: HashMap<&str, Direction> = HashMap::new();
    other_lines.lines().filter(|l| !l.is_empty()).for_each(|l| {
        let (current_pos, next_pos) = l.split_once('=').unwrap();
        let directions = next_pos.split_once(',').unwrap();
        desert_map.insert(current_pos.trim(), directions);
    });
    let first_line = first_line.chars().map(|c| match c {
        'L' => 0,
        'R' => 1,
        _ => panic!("Invalid instruction"),
    });
    return (first_line, desert_map);
}

fn go_through_desert(
    instructions: impl Iterator<Item = i8> + std::clone::Clone,
    map: HashMap<&str, Direction>,
) -> Option<u32> {
    let mut count = 0;
    let mut current_pos = "AAA";
    for c in instructions.cycle() {
        // println!("Left/Right: {c}");
        let positions = *map.get(current_pos).unwrap();
        // println!("Test position: {:?}", positions);
        current_pos = match c {
            0 => positions.0,
            1 => positions.1,
            _ => unreachable!(),
        }
        .trim_matches(|c: char| !c.is_alphabetic());
        // println!("Cur position: {:?}", current_pos);
        count += 1;
        if current_pos == "ZZZ" {
            return Some(count);
        }
    }
    unreachable!()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (instructions, map) = parse_input(input);
    go_through_desert(instructions, map)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

// #1 (24.4ms)
