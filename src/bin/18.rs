advent_of_code::solution!(18);

use advent_of_code::utils::map::Direction;

type Position = (isize, isize);
type DigStep = (Direction, isize);

fn parse_map<F>(input: &str, parse_line: F) -> Vec<Position>
where
    F: Fn(&str) -> DigStep,
{
    let mut current_point: Position = (0, 0);
    let mut dig_plan: Vec<Position> = Vec::with_capacity(input.lines().count());

    for l in input.lines() {
        let (direction, steps) = parse_line(l);
        match direction {
            Direction::Down => {
                current_point.1 = current_point.1 - steps;
            }
            Direction::Right => {
                current_point.0 = current_point.0 + steps;
            }
            Direction::Left => {
                current_point.0 = current_point.0 - steps;
            }
            Direction::Up => {
                current_point.1 = current_point.1 + steps;
            }
        }
        dig_plan.push(current_point);
    }
    dig_plan
}

fn parse_line(line: &str) -> DigStep {
    let mut splitted_line = line.split(|c| c == ' ');
    let direction = match splitted_line.next().unwrap() {
        "D" => Direction::Down,
        "R" => Direction::Right,
        "L" => Direction::Left,
        "U" => Direction::Up,
        _ => panic!("At least one value is required!"),
    };
    let steps = splitted_line.next().unwrap().parse::<isize>().unwrap();
    (direction, steps)
}

fn parse_hex(line: &str) -> DigStep {
    let mut it = line.split_whitespace();
    let _dir_txt = it.next().unwrap();
    let _steps_txt = it.next().unwrap();
    let raw = it.next().unwrap();
    let inner = raw.strip_prefix('(').and_then(|s| s.strip_suffix(')'));
    let inner = inner.and_then(|s| s.strip_prefix('#')).unwrap();
    let (meters, direction_str) = inner.split_at(5);

    let direction = match direction_str {
        "0" => Direction::Right,
        "1" => Direction::Down,
        "2" => Direction::Left,
        "3" => Direction::Up,
        _ => panic!("At least one value is required!"),
    };

    let steps = isize::from_str_radix(meters, 16).unwrap();
    (direction, steps)
}

fn area_with_boundaries(map: &[Position]) -> u64 {
    let mut double_area = 0;
    let mut perimeter = 0;
    let n = map.len();
    for i in 0..n {
        let (x1, y1) = map[i];
        let (x2, y2) = map[(i + 1) % n];
        let area = (x1 * y2) - (x2 * y1);
        perimeter += (x2 - x1).abs() + (y2 - y1).abs();
        double_area += area;
    }
    let area = (num::abs(double_area) / 2) + perimeter / 2 + 1;
    area.try_into().unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = parse_map(input, parse_line);
    let res = area_with_boundaries(&map);
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = parse_map(input, parse_hex);
    let res = area_with_boundaries(&map);
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}

// On Macbook Pro M4
// Part 1: 48503 (507.5µs)
// Part 2: 148442153147147 (860.5µs)
