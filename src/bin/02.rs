advent_of_code::solution!(2);
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut constraints: HashMap<String, u32> = HashMap::new();

    constraints.insert(String::from("red"), 12);
    constraints.insert(String::from("green"), 13);
    constraints.insert(String::from("blue"), 14);

    let mut result = 0;
    let mut counter = 0;
    'game: for l in input.lines() {
        counter += 1;

        let start = l.find(':').unwrap();
        let round = &l[start + 1..];

        for turn in round.split(';') {
            for colors in turn.split(',') {
                let mut values = colors.split_whitespace();
                let amount: u32 = values.next().unwrap().parse().unwrap();
                let color = values.next().unwrap();

                if &amount > constraints.get(color).unwrap() {
                    continue 'game;
                }
            }
        }
        result += counter;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;

    for l in input.lines() {
        let mut fewest_numbers: HashMap<&str, u32> = HashMap::new();
        let start = l.find(':').unwrap();
        let round = &l[start + 1..];

        for turn in round.split(';') {
            for colors in turn.split(',') {
                let mut values = colors.split_whitespace();
                let amount: u32 = values.next().unwrap().parse().unwrap();
                let color = values.next().unwrap();

                let minimal_number = fewest_numbers.get(color).unwrap_or(&0);
                if &amount > minimal_number {
                    fewest_numbers.insert(&color, amount);
                }
            }
        }
        let mut power = 1;
        ["green", "red", "blue"].iter().for_each(|color| {
            let amount = fewest_numbers.get(color).unwrap_or(&0);
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
// Second try: 3.7ms
