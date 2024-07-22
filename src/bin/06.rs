advent_of_code::solution!(6);

const SPEED_INCREASE: u32 = 1;
// mm/ms

fn parse_input(input: &str) -> Vec<(u32, u32)> {
    let lines = input.lines();
    let time = lines.next().unwrap();
    let distances = lines.next().unwrap();
}
pub fn part_one(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
