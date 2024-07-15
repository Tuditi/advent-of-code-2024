advent_of_code::solution!(4);
use std::cmp;
use std::collections::HashMap;

#[derive(Debug)]
struct Card {
    winning: Vec<u32>,
    current: Vec<u32>,
}

impl Card {
    fn new(input: &str) -> Self {
        let numbers = input.split(':').collect::<Vec<&str>>()[1];
        let winning_and_current = numbers.split('|').collect::<Vec<&str>>();
        let winning = Card::parse_values(winning_and_current[0]);
        let current = Card::parse_values(winning_and_current[1]);

        Card { winning, current }
    }

    fn parse_values(input: &str) -> Vec<u32> {
        input
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u32>().unwrap())
            .collect()
    }

    fn get_match(&self) -> u32 {
        let mut result = 0;
        for item in self.current.iter() {
            if self.winning.contains(item) {
                result += 1;
            }
        }
        result
    }

    fn get_points(&self) -> u32 {
        let result = self.get_match();
        if result == 0 {
            return 0;
        }
        let points = 2_u32.pow(result - 1);
        points
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    input.lines().for_each(|l| {
        let card = Card::new(&l);
        total += card.get_points();
    });
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards: HashMap<usize, Card> = HashMap::new();
    input.lines().enumerate().for_each(|(i, l)| {
        let card = Card::new(&l);
        cards.insert(i, card);
    });
    let cards_length = cards.len();

    let mut indices_to_check: Vec<u32> = vec![1; cards_length];
    for i in 0..cards_length {
        let current_card = cards.get(&i).unwrap();
        let match_count = current_card.get_match();
        let start = i + 1;
        let end = cmp::min(cards_length - 1, start + (match_count as usize));
        for j in start..end {
            indices_to_check[j] += indices_to_check[i];
        }
    }
    indices_to_check.into_iter().reduce(|acc, e| acc + e)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}

// pt1 10ms
// pt1 5.2ms & pt2 5.6ms
