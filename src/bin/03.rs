advent_of_code::solution!(3);
use regex::Regex;

#[derive(Debug)]
struct Number {
    value: String,
    point: Point,
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Number {
    fn find_square(&self, max_point: Point) -> (Point, Point) {
        let Number { point, value } = &self;
        (point.top_left(), point.bottom_right(value.len(), max_point))
    }
}

impl Point {
    fn bottom_right(&self, value: usize, max_point: Point) -> Point {
        Point {
            x: Point::max(self.x + value, max_point.x),
            y: Point::max(self.y + 1, max_point.y),
        }
    }

    fn top_left(&self) -> Point {
        Point {
            x: Point::min(self.x),
            y: Point::min(self.y),
        }
    }

    fn max(value: usize, max_value: usize) -> usize {
        let mut result = value;
        if result == max_value {
            result = max_value - 1;
        }
        result
    }

    fn min(value: usize) -> usize {
        if value == 0 {
            return 0;
        }
        value - 1
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;
    let lines: Vec<&str> = input.lines().collect();
    for (y, &line) in lines.iter().enumerate() {
        let re = Regex::new(r"\d+").unwrap();
        for mat in re.find_iter(&line) {
            let number = Number {
                value: String::from(mat.as_str()),
                point: Point { x: mat.start(), y },
            };

            let (top_left, bottom_right) = number.find_square(Point {
                x: line.len(),
                y: lines.len(),
            });
            let mut neighbours: Vec<&str> = Vec::new();
            for y in top_left.y..=bottom_right.y {
                neighbours.push(&lines[y][top_left.x..=bottom_right.x]);
            }
            let re = Regex::new(r"[^0-9.]").unwrap();
            if re.is_match(&neighbours.concat()) {
                result += number.value.parse::<u32>().unwrap();
            }
            neighbours.concat();
        }
    }
    Some(result)
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

// First try: pt1: 3.4s
