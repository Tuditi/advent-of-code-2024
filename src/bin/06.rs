advent_of_code::solution!(6);

use std::iter::Zip;

use advent_of_code::utils::parsers::*;

fn parse_input(input: &str) -> Zip<impl Iterator<Item = u64> + '_, impl Iterator<Item = u64> + '_> {
    let mut lines = input.lines();
    let time = parsers::parse_line::<u64>(lines.next().unwrap());
    let distances = parsers::parse_line::<u64>(lines.next().unwrap());
    time.zip(distances)
}

fn calculate_distances(time: &u64) -> Vec<u64> {
    let mut possible_distances = vec![];
    for i in 1..*time {
        let speed = i;
        possible_distances.push((time - i) * speed);
    }
    possible_distances
}

fn ways_to_win(distances: Vec<u64>, record: &u64) -> usize {
    distances.iter().filter(|&dist| dist > record).count()
}

fn calculate_result(input: Vec<(u64, u64)>) -> u64 {
    let mut result = 1;
    input.iter().for_each(|(time, record)| {
        let distances = calculate_distances(time);
        result *= ways_to_win(distances, record);
    });
    result as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input).collect();
    Some(calculate_result(input))
}

fn concat_f(a: u64, b: u64) -> u64 {
    format!("{a}{b}").parse().unwrap()
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let input =
        input.reduce(|(acc_d, acc_t), (e_d, e_t)| (concat_f(acc_d, e_d), concat_f(acc_t, e_t)));
    Some(calculate_result(vec![input.unwrap()]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}

// #1: (111.6µs)
// #2; Part 1: 32076 (80.0µs) Part 2: 34278221 (3.0s)
