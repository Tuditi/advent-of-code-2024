advent_of_code::solution!(2);
use std::collections::HashMap;

const COLORS: [&str; 3] = ["green", "red", "blue"];

pub fn part_one(input: &str) -> Option<u32> {
    let mut constraints: HashMap<&str, u32> = HashMap::new();

    constraints.insert(COLORS[0], 12);
    constraints.insert(COLORS[1], 13);
    constraints.insert(COLORS[2], 14);

    let mut result = 0;
    let mut counter = 0;
    'game: for round in input.lines() {
        counter += 1;
        let parsed_round = parse_round(&round);
        for color in COLORS {
            if parsed_round.get(color) > constraints.get(color) {
                continue 'game;
            }
        }

        result += counter;
    }
    Some(result)
}

fn parse_round(round: &str) -> HashMap<&str, u32> {
    let start = round.find(':').unwrap();
    let round = &round[start + 1..];
    let mut parsed_map: HashMap<&str, u32> = HashMap::new();
    for turn in round.split(';') {
        for colors in turn.split(',') {
            let mut values = colors.split_whitespace();
            let amount: u32 = values.next().unwrap().parse().unwrap();
            let color = values.next().unwrap();

            let current_value = parsed_map.get(color).unwrap_or(&0);
            if &amount > current_value {
                parsed_map.insert(color, amount);
            }
        }
    }
    parsed_map
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;

    for round in input.lines() {
        let parsed_round = parse_round(round);

        let mut power = 1;
        COLORS.iter().for_each(|color| {
            let amount = parsed_round.get(color).unwrap_or(&0);
            power *= amount;
        });
        result += power;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}

// First try: 2.5ms
// Second try: pt1: 3.7ms pt2: 7.7ms
// Third try: pt1: 2.4ms pt2: 2.3ms
