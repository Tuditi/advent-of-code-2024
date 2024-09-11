advent_of_code::solution!(12);

#[derive(Debug, PartialEq, Eq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl From<u8> for Condition {
    fn from(c: u8) -> Self {
        match c {
            b'#' => Condition::Damaged,
            b'.' => Condition::Operational,
            b'?' => Condition::Unknown,
            _ => panic!("Unknown Condition!"),
        }
    }
}

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct HotSpring {
    condition_records: Vec<u8>,
    damaged_groups: Vec<usize>,
}

impl HotSpring {
    fn get_different_arrangements(&self) -> usize {
        let mut cache = HashMap::new();
        self.count_arrangements(0, 0, 0, &mut cache)
    }

    fn count_arrangements(
        &self,
        pos: usize,
        group: usize,
        current_length: usize,
        cache: &mut HashMap<(usize, usize, usize), usize>,
    ) -> usize {
        // Check if this state has been calculated before
        if let Some(&count) = cache.get(&(pos, group, current_length)) {
            return count;
        }

        // Base case: reached the end of the condition records
        if pos == self.condition_records.len() {
            return if group == self.damaged_groups.len() && current_length == 0 {
                // All groups have been processed and we're not in the middle of a group
                1
            } else if group == self.damaged_groups.len() - 1
                && current_length == self.damaged_groups[group]
            {
                // We've just finished the last group
                1
            } else {
                // Invalid arrangement
                0
            };
        }

        let mut count = 0;
        // Try both operational (.) and damaged (#) states
        for &c in [b'.', b'#'].iter() {
            // Check if the current position allows this state
            if self.condition_records[pos] == c || self.condition_records[pos] == b'?' {
                if c == b'.' && current_length == 0 {
                    // Continue with next position, same group, reset length
                    count += self.count_arrangements(pos + 1, group, 0, cache);
                } else if c == b'.'
                    && current_length > 0
                    && group < self.damaged_groups.len()
                    && current_length == self.damaged_groups[group]
                {
                    // Finished a group, move to next group
                    count += self.count_arrangements(pos + 1, group + 1, 0, cache);
                } else if c == b'#' {
                    // Continue damaged group
                    count += self.count_arrangements(pos + 1, group, current_length + 1, cache);
                }
            }
        }

        // Cache the result for this state
        cache.insert((pos, group, current_length), count);
        count
    }

    fn unfold(&self) -> Self {
        let mut unfolded_hot_spring = self.clone();
        let mut condition_records: Vec<u8> = self.condition_records.clone();
        condition_records.insert(0, b'?');
        let damaged_groups = self.damaged_groups.clone();
        for _ in 0..4 {
            unfolded_hot_spring
                .condition_records
                .extend(&condition_records);
            unfolded_hot_spring.damaged_groups.extend(&damaged_groups);
        }
        unfolded_hot_spring
    }
}

fn parse_input(input: &str) -> Vec<HotSpring> {
    let mut hot_springs: Vec<HotSpring> = vec![];
    input.lines().for_each(|l| hot_springs.push(parse_line(l)));
    hot_springs
}

fn parse_line(l: &str) -> HotSpring {
    let mut group = l.split_whitespace();
    let condition_records = group.next().unwrap().as_bytes().to_vec();
    let damaged_groups = group
        .next()
        .unwrap()
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect();
    HotSpring {
        condition_records,
        damaged_groups,
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut sum = 0;
    let hotsprings = parse_input(input);
    hotsprings
        .into_iter()
        .for_each(|s| sum += s.get_different_arrangements());
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut sum = 0;
    let hotsprings = parse_input(input);

    hotsprings.into_iter().for_each(|s| {
        let unfolded_hot_spring = s.unfold();
        sum += unfolded_hot_spring.get_different_arrangements()
    });
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_case() {
        let hotspring1 = parse_line("# 1");
        assert_eq!(hotspring1.get_different_arrangements(), 1);

        let hotspring1 = parse_line("? 1");
        assert_eq!(hotspring1.get_different_arrangements(), 1);

        let hotspring1 = parse_line(". 1");
        assert_eq!(hotspring1.get_different_arrangements(), 0);

        let hotspring2 = parse_line("??? 1,1");
        assert_eq!(hotspring2.get_different_arrangements(), 1);
    }

    #[test]
    fn test_two_iterations() {
        let hotspring1 = parse_line("??.### 1,3");
        assert_eq!(hotspring1.get_different_arrangements(), 2);
    }

    #[test]
    fn test_different_arrangements() {
        let hotspring1 = parse_line("???.### 1,1,3");
        assert_eq!(hotspring1.get_different_arrangements(), 1);

        let hotspring2 = parse_line(".??..??...?##. 1,1,3");
        assert_eq!(hotspring2.get_different_arrangements(), 4);

        let hotspring3 = parse_line("?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(hotspring3.get_different_arrangements(), 1);

        let hotspring4 = parse_line("????.#...#... 4,1,1");
        assert_eq!(hotspring4.get_different_arrangements(), 1);

        let hotspring5 = parse_line("????.######..#####. 1,6,5");
        assert_eq!(hotspring5.get_different_arrangements(), 4);

        let hotspring6 = parse_line("?###???????? 3,2,1");
        assert_eq!(hotspring6.get_different_arrangements(), 10);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}

// Part 1: 7195 (261.6ms) &&Part 2: 33992866292225 (6.9s)
