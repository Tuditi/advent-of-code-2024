advent_of_code::solution!(5);

use advent_of_code::utils::parsers::*;
use rayon::prelude::*;

#[derive(Debug)]
struct MapLine {
    destination: i64,
    source: i64,
    range: i64,
}

type SeedMap = Vec<Vec<MapLine>>;

fn parse_input<'a>(
    input: &'a str,
) -> (impl rayon::iter::ParallelIterator<Item = i64> + 'a, SeedMap) {
    let mut maps: SeedMap = vec![vec![]];
    let mut map_index = 0;
    let (first_line, next_lines) = input.split_once('\n').unwrap();
    let iterator = next_lines.lines().filter(|l| !l.is_empty());

    let seeds = parsers::par_parse_line(&first_line);

    iterator.for_each(|l| {
        let next_line: Vec<i64> = parsers::par_parse_line(&l).collect();
        match &next_line.len() {
            0 => {
                map_index += 1;
                maps.push(vec![]);
            }
            3 => {
                let map_line = MapLine {
                    destination: next_line[0],
                    source: next_line[1],
                    range: next_line[2],
                };
                maps[map_index].push(map_line);
            }
            _ => panic!("Unrecognized line {:?}", next_line),
        }
    });
    maps.remove(0);
    (seeds, maps)
}

fn go_through_maps(maps: &SeedMap, mut input: i64) -> u32 {
    maps.iter().for_each(|current_map| {
        input = transform(current_map, &input);
    });
    input as u32
}

fn transform(map: &Vec<MapLine>, input: &i64) -> i64 {
    for line in map {
        let MapLine {
            source,
            range,
            destination,
        } = line;
        let diff = *input - *source as i64;
        if diff >= 0 && diff < *range as i64 {
            return (*destination as i64) + diff;
        }
    }
    *input
}

fn calculate_seeds_from_ranges(ranges: Vec<i64>, maps: SeedMap) -> u32 {
    let length = ranges.len();
    let mut min_location = u32::MAX;
    for i in 0..length / 2 {
        let start_seed = ranges[2 * i];
        let range = ranges[2 * i + 1];
        let seeds = (start_seed..start_seed + range).into_par_iter();

        let potential_location = calculate_closest_location(seeds, &maps).unwrap();
        if potential_location < min_location {
            min_location = potential_location
        }
    }
    min_location
}

fn calculate_closest_location(
    seeds: impl rayon::iter::ParallelIterator<Item = i64>,
    maps: &SeedMap,
) -> Option<u32> {
    let locations = seeds.map(|seed| go_through_maps(&maps, seed));
    let closest_location = locations.min();
    closest_location
}

pub fn part_one(input: &str) -> Option<u32> {
    let (seeds, maps) = parse_input(input);
    calculate_closest_location(seeds, &maps)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (seed_ranges, maps) = parse_input(input);
    Some(calculate_seeds_from_ranges(seed_ranges.collect(), maps))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}

// #1: 454.1µs
// #2  pt1: 354µs & pt2: 3618.3s
// #3: Use data parallelisation with Rayon: pt1 9.4 ms, pt2: 701s
