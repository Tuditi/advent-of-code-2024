use std::collections::HashMap;

advent_of_code::solution!(14);

type RockColumn = Vec<char>;
type Rocks = Vec<RockColumn>;

trait Platform {
    fn slide(self, normal: bool) -> Self;
    fn slide_north(self) -> Self;
    fn slide_east(self) -> Self;
    fn slide_south(self) -> Self;
    fn slide_west(self) -> Self;
    fn cycle(self) -> Self;
    fn transpose(self) -> Self;
    fn calculate_load(&self) -> usize;
}

impl Platform for Rocks {
    fn transpose(self) -> Self {
        let mut transposed: Rocks = vec![Vec::new(); self[0].len()];
        self.into_iter().for_each(|rock_row| {
            rock_row.into_iter().enumerate().for_each(|(i, c)| {
                transposed[i].push(c);
            })
        });
        transposed
    }

    fn slide(self, normal: bool) -> Self {
        let mut result = vec![];
        self.into_iter().for_each(|row| {
            let mut sorted_row: Vec<char> = Vec::with_capacity(row.len());
            let free_cols = row.split(|c| *c == '#');
            free_cols.into_iter().for_each(|cols| {
                let mut col_vec = cols.to_vec();
                col_vec.sort_by(|a, b| if normal { a.cmp(b) } else { b.cmp(a) });
                sorted_row.extend(col_vec);
                sorted_row.push('#');
            });
            sorted_row.pop();
            result.push(sorted_row)
        });
        result
    }

    fn slide_north(self) -> Self {
        self.transpose().slide(false).transpose()
    }

    fn slide_east(self) -> Self {
        self.slide(true)
    }

    fn slide_south(self) -> Self {
        self.transpose().slide(true).transpose()
    }

    fn slide_west(self) -> Self {
        self.slide(false)
    }

    fn cycle(self) -> Self {
        self.slide_north().slide_west().slide_south().slide_east()
    }

    fn calculate_load(&self) -> usize {
        let mut load = 0;
        let length = self.len();
        self.iter().enumerate().for_each(|(i, rock_row)| {
            rock_row.iter().for_each(|rock| {
                if *rock == 'O' {
                    load += length - i;
                }
            });
        });
        load
    }
}

fn parse_input(input: &str) -> Vec<RockColumn> {
    let mut rocks: Vec<RockColumn> = Vec::new();
    input.lines().for_each(|l| {
        let row = l.chars().collect();
        rocks.push(row);
    });
    rocks
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut rocks = parse_input(input);
    rocks = rocks.slide_north();
    Some(rocks.calculate_load())
}

fn calculate_remaining_cycles(mut rocks: Rocks, remaining_iterations: i32) -> usize {
    for _ in 0..remaining_iterations {
        rocks = rocks.cycle();
    }
    rocks.calculate_load()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut rocks = parse_input(input);
    let mut hash_map: HashMap<Rocks, i32> = HashMap::new();
    let iterations = 1_000_000_000;
    for i in 1..=iterations {
        hash_map.insert(rocks.clone(), i);
        rocks = rocks.cycle();
        if let Some(counter) = hash_map.get(&rocks) {
            let remaining_iterations = (iterations - i) % (i - counter + 1);
            return Some(calculate_remaining_cycles(rocks, remaining_iterations));
        }
    }
    panic!("Shouldn't reach end of code!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let rocks: Rocks = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ];
        let expected: Rocks = vec![
            vec!['a', 'd', 'g'],
            vec!['b', 'e', 'h'],
            vec!['c', 'f', 'i'],
        ];
        let result = rocks.clone().transpose();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_transpose_twice() {
        let rocks: Rocks = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ];
        let result = rocks.clone().transpose().transpose();
        assert_eq!(result, rocks);
    }

    #[test]
    fn test_sorting() {
        let rocks: Rocks = vec![vec!['O', 'O', '.', 'O', '.', 'O', '.', '.', '#', '#']];
        let result = rocks.slide(false);
        assert_eq!(
            result,
            vec![vec!['O', 'O', 'O', 'O', '.', '.', '.', '.', '#', '#']]
        )
    }

    #[test]
    fn test_east_slide() {
        let rocks: Rocks = vec![vec!['O', 'O', '.', 'O', '.', 'O', '.', '.', '#', '#']];
        let result = rocks.slide_east();
        assert_eq!(
            result,
            vec![vec!['.', '.', '.', '.', 'O', 'O', 'O', 'O', '#', '#']]
        )
    }

    #[test]
    fn test_west_slide() {
        let rocks: Rocks = vec![vec!['O', 'O', '.', 'O', '.', 'O', '.', '.', '#', '#']];
        let result = rocks.slide_west();
        assert_eq!(
            result,
            vec![vec!['O', 'O', 'O', 'O', '.', '.', '.', '.', '#', '#']]
        )
    }

    #[test]
    fn test_load() {
        let rocks: Rocks = vec![vec!['O', 'O', 'O', 'O', '.', '.', '.', '.', '#', '#']];
        let result = rocks.calculate_load();
        assert_eq!(result, 4)
    }

    #[test]
    fn test_one_cycle() {
        let rocks: Rocks = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let result = rocks.cycle();
        let expected_result = vec![
            vec!['.', '.', '.', '.', '.', '#', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '#', '.', '.', '.', 'O', '#'],
            vec!['.', '.', '.', 'O', 'O', '#', '#', '.', '.', '.'],
            vec!['.', 'O', 'O', '#', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', 'O', 'O', 'O', '#', '.'],
            vec!['.', 'O', '#', '.', '.', '.', 'O', '#', '.', '#'],
            vec!['.', '.', '.', '.', 'O', '#', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', 'O', 'O', 'O', 'O'],
            vec!['#', '.', '.', '.', 'O', '#', '#', '#', '.', '.'],
            vec!['#', '.', '.', 'O', 'O', '#', '.', '.', '.', '.'],
        ];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_three_cycles() {
        let rocks: Rocks = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let result = rocks.cycle().cycle().cycle();
        let expected_result = parse_input(
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O",
        );
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}

// Part 1: 108144 (9.3ms) && Part 2: 108404 (2.8s)
