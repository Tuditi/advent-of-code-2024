advent_of_code::solution!(1);
use std::char;

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;
    input.lines().for_each(|l| {
        if let Some(value) = find_first_and_last(l, false) {
            result += value;
        }
    });
    Some(result)
}

fn find_first_and_last(input: &str, pt_2: bool) -> Option<u32> {
    let digits = DIGITS.iter().map(|&s| s.to_string()).collect();
    let first = find_digit(input, digits, pt_2)?;

    let reversed_input: String = input.chars().rev().collect();
    let reversed_digits = DIGITS.iter().map(|&d| d.chars().rev().collect()).collect();
    let last = find_digit(&reversed_input, reversed_digits, pt_2)?;

    let mut result = String::from(first);
    result.push(last);
    result.parse().ok()
}

fn find_digit(input: &str, digits: Vec<String>, pt_2: bool) -> Option<char> {
    let mut smallest_index = match input.find(|c: char| c.is_numeric()) {
        Some(index) => index,
        None => input.len(),
    };

    let mut counter: u32 = 0;
    let mut result = input.chars().nth(smallest_index);

    if pt_2 {
        for digit in digits {
            counter += 1;
            let index = match input.find(&digit) {
                Some(val) => val,
                None => continue,
            };
            if index < smallest_index {
                smallest_index = index;
                result = char::from_digit(counter, 10);
            }
        }
    }
    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;
    input.lines().for_each(|l| {
        if let Some(value) = find_first_and_last(l, true) {
            result += value
        };
    });
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        let result = part_one(
            "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet",
        );
        assert_eq!(result, Some(142));
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}

// First solution pt1: 16.7ms & pt2: 17.5ms
