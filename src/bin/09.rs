advent_of_code::solution!(9);

use advent_of_code::utils::parsers::*;
use parsers::parse_line;

#[derive(Debug)]
struct History {
    values: Vec<Vec<i32>>,
}

impl History {
    fn new(line: &str) -> Self {
        let mut first_row: Vec<i32> = vec![];
        parse_line::<i32>(line).for_each(|v| first_row.push(v));
        let mut values = vec![first_row];
        History::_populate_differences_recursively(&mut values);
        History { values }
    }

    fn _populate_differences_recursively(result: &mut Vec<Vec<i32>>) {
        let last = result.last().unwrap();
        let first_element = last.first().unwrap();
        if last.iter().all(|el| el == first_element) {
            return;
        } else {
            let differences = History::_find_differences(last);
            result.push(differences);
            History::_populate_differences_recursively(result);
        }
    }

    fn _find_differences(values: &Vec<i32>) -> Vec<i32> {
        let length = values.len();
        let mut differences: Vec<i32> = Vec::with_capacity(values.len());
        for i in 1..length {
            let diff = values[i] - values[i - 1];
            differences.push(diff);
        }
        differences
    }

    fn extrapolate_next_value(&self) -> i32 {
        let iterator = self.values.iter().rev();
        let mut next_value = 0;
        for row in iterator {
            let last = row.last().unwrap();
            next_value = last + next_value;
        }
        next_value
    }

    fn extrapolate_backwards(&self) -> i32 {
        let iterator = self.values.iter().rev();
        let mut previous_value = 0;
        for row in iterator {
            let first = row.first().unwrap();
            previous_value = first - previous_value;
        }
        previous_value
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut result = 0;
    input.lines().for_each(|l| {
        let history = History::new(l);
        let next_value = history.extrapolate_next_value();
        result += next_value;
    });
    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut result = 0;
    input.lines().for_each(|l| {
        let history = History::new(l);
        let previous_value = history.extrapolate_backwards();
        result += previous_value;
    });
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}

// #1 Part 1: 2098530125 (6.8ms) && Part 2: 1016 (6.7ms)
