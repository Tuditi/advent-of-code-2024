advent_of_code::solution!(5);

#[derive(Debug)]
struct MapLine {
    destination: i64,
    source: i64,
    range: i64,
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split(' ')
        .map(|x| x.parse::<i64>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect()
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<Vec<MapLine>>) {
    let mut seeds: Vec<i64> = vec![];
    let mut maps: Vec<Vec<MapLine>> = vec![vec![]];
    let mut map_index = 0;
    input
        .lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .for_each(|(i, l)| {
            // println!("Next line {l}");
            if i == 0 {
                seeds = parse_line(&l)
            } else {
                let next_line = parse_line(&l);

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
            }
        });
    maps.remove(0);
    (seeds, maps)
}

fn go_through_maps(maps: &Vec<Vec<MapLine>>, mut input: i64) -> u32 {
    println!("NEW INPUT {input}");
    // println!("");
    maps.iter().for_each(|current_map| {
        // println!("Input: {input}, Current Map: {:?}", current_map);
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

fn calculate_seeds_from_ranges(ranges: Vec<i64>) -> Vec<i64> {
    let length = ranges.len();
    let total_size = ranges.iter().step_by(2).map(|&range| range as usize).sum();
    let mut seeds = Vec::with_capacity(total_size);
    println!("start {length}");
    for i in 0..length / 2 {
        println!("{i}");
        let start_seed = ranges[2 * i];
        println!("a");

        let range = ranges[2 * i + 1];
        // let next_seeds = Vec::with_capacity(range);
        println!("b");
        seeds.extend(start_seed..start_seed + range);
        println!("c");
    }
    seeds
}

fn calculate_closest_location(seeds: Vec<i64>, maps: Vec<Vec<MapLine>>) -> Option<u32> {
    let locations = seeds.iter().map(|seed| go_through_maps(&maps, *seed));
    let closest_location = locations.min();
    closest_location
}

pub fn part_one(input: &str) -> Option<u32> {
    let (seeds, maps) = parse_input(input);
    calculate_closest_location(seeds, maps)
}

pub fn part_two(input: &str) -> Option<u32> {
    println!("Parse input");
    let (seed_ranges, maps) = parse_input(input);
    println!("Seed_ranges");
    let seeds = calculate_seeds_from_ranges(seed_ranges);
    println!("Seeds: {:?}", seeds);
    calculate_closest_location(seeds, maps)
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

// Part 1: 454.1µs
