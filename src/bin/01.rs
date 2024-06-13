advent_of_code::solution!(1);
use std::char;

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;
    input.lines().for_each(|l| {
        if let Some(value) = find_first_and_last(l, &None, &None) {
            result += value;
        }
    });
    Some(result)
}

fn find_first_and_last(
    input: &str,
    digits: &Option<Vec<String>>,
    reversed_digits: &Option<Vec<String>>,
) -> Option<u32> {
    let first = find_digit(input, &digits)?;

    let reversed_input: String = input.chars().rev().collect();
    let last = find_digit(&reversed_input, &reversed_digits)?;

    let mut result = String::from(first);
    result.push(last);
    result.parse().ok()
}

fn find_digit(input: &str, digits: &Option<Vec<String>>) -> Option<char> {
    let mut smallest_index = match input.find(|c: char| c.is_numeric()) {
        Some(index) => index,
        None => input.len(),
    };

    let mut counter: u32 = 0;
    let mut result = input.chars().nth(smallest_index);

    if digits.is_some() {
        for digit in digits.as_ref().unwrap() {
            counter += 1;
            let index = match input.find(digit) {
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
    let digits = Some(DIGITS.iter().map(|&s| s.to_string()).collect());
    let reversed_digits = Some(DIGITS.iter().map(|&d| d.chars().rev().collect()).collect());

    input.lines().for_each(|l| {
        if let Some(value) = find_first_and_last(l, &digits, &reversed_digits) {
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

// First solution
// pt1: 16.7ms & pt2: 17.5ms
// Second solution: move creation of Vec<String> for pt2 out of loop.
// pt1: 4.8ms & pt2: 7.8ms
