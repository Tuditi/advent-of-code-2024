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
    let re = Regex::new(r"\d+").unwrap();

    let points_with_neigbors = find_points_with_neighbors_matching_regex(lines, re);
    let re = Regex::new(r"[^0-9.]").unwrap();
    points_with_neigbors.iter().for_each(|p| {
        let (number, neighbors) = p;
        if re.is_match(&neighbors.concat()) {
            result += number.value.parse::<u32>().unwrap();
        }
    });
    Some(result)
}

fn find_points_with_neighbors_matching_regex(
    lines: Vec<&str>,
    re_points: Regex,
) -> Vec<(Number, Vec<&str>)> {
    let mut result: Vec<(Number, Vec<&str>)> = vec![];
    for (y, &line) in lines.iter().enumerate() {
        for mat in re_points.find_iter(&line) {
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
                let neighbour = find_neighbour(&lines[y], top_left.x, bottom_right.x);
                neighbours.push(neighbour);
            }
            neighbours.concat();
            result.push((number, neighbours));
        }
    }
    result
}

fn find_neighbour(line: &str, start_index: usize, end_index: usize) -> &str {
    let characters = line.as_bytes();
    let mut start = start_index;
    let mut end = end_index;
    loop {
        let previous_character = characters[start] as char;
        if previous_character.is_numeric() {
            if start == 0 {
                break;
            } else {
                start -= 1;
            };
        } else {
            if start != start_index {
                start += 1;
            }
            break;
        }
    }
    loop {
        let next_char = characters[end] as char;
        if next_char.is_numeric() {
            if end == line.len() - 1 {
                break;
            } else {
                end += 1;
            }
        } else {
            if end != end_index {
                end -= 1;
            }
            break;
        }
    }
    &line[start..=end]
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;
    let lines: Vec<&str> = input.lines().collect();
    let re = Regex::new(r"[*]").unwrap();
    let re_digits = Regex::new(r"\d+").unwrap();
    let points_with_neigbors = find_points_with_neighbors_matching_regex(lines, re);
    points_with_neigbors
        .iter()
        .filter(|d| satisfies_re_twice(&d.1, &re_digits))
        .for_each(|(_, points)| {
            let mut square = 1;
            points.iter().for_each(|v| {
                v.split(|c: char| !c.is_numeric()).for_each(|c: &str| {
                    if let Ok(val) = c.parse::<u32>() {
                        square *= val;
                    }
                });
            });
            result += square;
        });
    Some(result)
}

fn satisfies_re_twice(input: &Vec<&str>, re: &Regex) -> bool {
    let mut counter = 0;
    for neighbor in input {
        let numbers: Vec<_> = re.find_iter(&neighbor).collect();
        let amount = numbers.len();
        counter += amount
    }
    counter == 2
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
        assert_eq!(result, Some(467835));
    }
}

// First try: pt1: 3.4s
// Second try: pt1: 28.2 ms & pt2: 12.5ms
