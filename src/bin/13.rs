advent_of_code::solution!(13);
use std::cmp::min;

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    let mut mirrors = Vec::new();
    let mut mirror = Vec::new();
    input.lines().for_each(|l| {
        if l.len() == 0 {
            mirrors.push(mirror.clone());
            mirror = Vec::new();
        } else {
            mirror.push(l);
        }
    });
    mirrors.push(mirror);
    mirrors
}

fn find_horizontal_reflection(input: &Vec<&str>, smudges: usize) -> Option<usize> {
    let depth = input.len() - 1;
    'row: for i in 0..depth {
        let mut smudges_count = 0;
        let iterations = min(i + 1, depth - i);
        for j in 0..iterations {
            let top = input[i - j].as_bytes().iter();
            let bottom = input[i + 1 + j].as_bytes().iter();
            let comparison = top.zip(bottom);
            for (top_char, bottom_char) in comparison {
                if top_char != bottom_char {
                    smudges_count += 1;
                    if smudges_count > smudges {
                        continue 'row;
                    }
                }
            }
        }

        if smudges_count == smudges {
            return Some(i + 1);
        }
    }
    None
}

fn find_vertical_reflection(input: &Vec<&str>, smudges: usize) -> Option<usize> {
    let width = input[0].len() - 1;
    let depth = input.len();
    'col: for i in 0..width {
        let mut smudges_count = 0;
        let iterations = min(i + 1, width - i);
        for j in 0..depth {
            let row = input[j].as_bytes();
            for k in 0..iterations {
                if row[i - k] != row[i + 1 + k] {
                    smudges_count += 1;
                    if smudges_count > smudges {
                        continue 'col;
                    }
                }
            }
        }

        if smudges_count == smudges {
            return Some(i + 1);
        }
    }
    None
}

fn solve(input: &str, smudges: usize) -> Option<usize> {
    let mirrors = parse_input(input);
    let mut sum = 0;
    mirrors.iter().for_each(|m| {
        if let Some(rows_above) = find_horizontal_reflection(m, smudges) {
            sum += 100 * rows_above;
        } else if let Some(cols_left) = find_vertical_reflection(m, smudges) {
            sum += cols_left;
        } else {
            panic!("No pattern found!")
        }
    });
    Some(sum)
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 0)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_col() {
        let mirror = ".#...##..
..##.#.##
.#.###...
###..#.##
##.#.####
..#.#..##
.###...##
.#...#.##
#####.#..
...#..###
###.##.##
####...##
####..###
###.##.##
...#..###";
        let result = part_one(mirror);
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_vertical_part_two() {
        let mirror = "###.##.##
##..##..#
#.######.
#.######.
##..##..#
###.##.##
.###..###
#.#....#.
..#.##.##";
        let result = part_two(mirror);
        assert_eq!(result, Some(5))
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}

// #1: Part 1: 37113 (30.7ms)
// #2 Part 1: 37113 (1.9ms) && Part 2: 30449 (2.1ms)
