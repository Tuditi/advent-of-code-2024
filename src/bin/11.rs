advent_of_code::solution!(11);
use std::vec::IntoIter;

use advent_of_code::utils::position::position::*;
use itertools::{Combinations, Itertools};

type SpaceExpansion = (Vec<usize>, Vec<usize>);

fn create_space(input: &str) -> Vec<&str> {
    let mut space: Vec<&str> = vec![];
    input.lines().for_each(|l| {
        space.push(l);
    });
    space
}

fn get_expanded_rows_columns(space: &Vec<&str>) -> SpaceExpansion {
    let mut rows_to_be_expanded: Vec<usize> = vec![];
    'rows: for (i, l) in space.iter().enumerate() {
        if l.chars().any(|c| c == '#') {
            continue 'rows;
        };
        rows_to_be_expanded.push(i);
    }

    let mut columns_to_be_expanded: Vec<usize> = vec![];
    'cols: for i in 0..space[0].len() {
        for row in space {
            if row.as_bytes()[i] == b'#' {
                continue 'cols;
            }
        }
        columns_to_be_expanded.push(i);
    }

    (rows_to_be_expanded, columns_to_be_expanded)
}

fn create_pairs(
    space: Vec<&str>,
    expansion: &SpaceExpansion,
    expansion_factor: usize,
) -> Combinations<IntoIter<Position>> {
    let mut pairs: Vec<Position> = vec![];
    space.iter().enumerate().for_each(|(i, row)| {
        let mut galaxy_row = row
            .match_indices('#')
            .map(|(j, _)| {
                create_expanded_position(Position::new(j, i), expansion, expansion_factor)
            })
            .collect();
        pairs.append(&mut galaxy_row);
    });
    pairs.into_iter().combinations(2)
}

fn create_expanded_position(
    position: Position,
    expansion: &SpaceExpansion,
    expansion_factor: usize,
) -> Position {
    // println!("Old position: {:?}", position.get_position());
    let (x, y) = position.get_position();
    let mut x_expanded = x;
    let mut y_expanded = y;

    let (rows, columns) = expansion;

    for col_idx in columns {
        if (0..x).contains(col_idx) {
            x_expanded += expansion_factor - 1
        }
    }

    for row_idx in rows {
        if (0..y).contains(row_idx) {
            y_expanded += expansion_factor - 1;
        }
    }
    let position = Position::new(x_expanded, y_expanded);
    // println!("New position: {:?}", position.get_position());
    position
}

fn get_expanded_distance(pair: &Vec<Position>) -> usize {
    if pair.len() != 2 {
        panic!("Incorrect pair length!");
    }
    pair[0].get_distance(pair[1])
}

fn estimate_galaxy_size(input: &str, expansion_factor: usize) -> Option<u32> {
    let space = create_space(input);
    let expanded_space = get_expanded_rows_columns(&space);
    let pairs = create_pairs(space, &expanded_space, expansion_factor);
    let distance = pairs.fold(0, |acc: usize, el| acc + get_expanded_distance(&el));
    Some(distance as u32)
}
pub fn part_one(input: &str) -> Option<u32> {
    estimate_galaxy_size(input, 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    estimate_galaxy_size(input, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_expanded_distance() {
        let expansion_factor = 2;
        let expansion: (Vec<usize>, Vec<usize>) = (vec![3, 7], vec![2, 5, 8]);

        let point_5 = create_expanded_position(Position::new(1, 5), &expansion, expansion_factor);
        let point_9 = create_expanded_position(Position::new(4, 9), &expansion, expansion_factor);
        let result = get_expanded_distance(&vec![point_5, point_9]);
        assert_eq!(result, 9);

        let point_1 = create_expanded_position(Position::new(3, 0), &expansion, expansion_factor);
        let point_7 = create_expanded_position(Position::new(7, 8), &expansion, expansion_factor);
        let result = get_expanded_distance(&vec![point_1, point_7]);
        assert_eq!(result, 15);

        let point_3 = create_expanded_position(Position::new(0, 2), &expansion, expansion_factor);
        let point_6 = create_expanded_position(Position::new(9, 6), &expansion, expansion_factor);
        let result = get_expanded_distance(&vec![point_3, point_6]);
        assert_eq!(result, 17);

        let point_8 = create_expanded_position(Position::new(0, 9), &expansion, expansion_factor);
        let result = get_expanded_distance(&vec![point_8, point_9]);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_amount_of_pairs() {
        let binding = advent_of_code::template::read_file("examples", DAY);
        let space = create_space(&binding);
        let expansion_factor = 1;
        let expansion: (Vec<usize>, Vec<usize>) = (vec![3, 7], vec![2, 5, 8]);
        let pairs = create_pairs(space, &expansion, expansion_factor);
        let result = pairs.collect::<Vec<Vec<Position>>>().len();
        assert_eq!(result, 36)
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two_a() {
        let result =
            estimate_galaxy_size(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(1030));
    }

    #[test]
    fn test_part_two_b() {
        let result =
            estimate_galaxy_size(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(1030));
    }
}
// #1 Part 1: 9723824 (75.1ms) Part 2: 1099821032 (69.3ms)
