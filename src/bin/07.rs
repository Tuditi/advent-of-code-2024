use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(7);

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Hand {
    cards: String,
    bid: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_a = self.get_hand_type();
        let type_b = other.get_hand_type();
        println! {"A: {:?}, B: {:?}", type_a, type_b};
        if type_a > type_b {
            // println!("A greater than B!");
            return Ordering::Greater;
        } else if type_a < type_b {
            // println!("A smaller than B!");
            return Ordering::Less;
        } else {
            // println!("Else!!!");
            let mut chars_a = self.cards.chars();
            let mut chars_b = other.cards.chars();

            loop {
                let next_a = chars_a.next();
                let next_b = chars_b.next();

                if let Some(a) = next_a {
                    if next_b.is_none() {
                        panic!("Unequal hands!")
                    }
                    let b = convert_card_char_to_ascending_utf8_char(next_b.unwrap());
                    let a = convert_card_char_to_ascending_utf8_char(a);
                    if a > b {
                        return Ordering::Greater;
                    } else if a < b {
                        return Ordering::Less;
                    }
                } else {
                    return Ordering::Equal;
                }
            }
        }
    }
}

impl Hand {
    fn new(l: &str) -> Self {
        let mut iterator = l.split(' ');
        let hand_char = iterator.next().unwrap();
        let bid = iterator.next().unwrap().parse::<u32>();

        if let Ok(bid) = bid {
            Hand {
                cards: String::from(hand_char),
                bid,
            }
        } else {
            panic!("{:?}", bid)
        }
    }

    fn create_map(&self) -> HashMap<char, usize> {
        let mut cards: HashMap<char, usize> = HashMap::new();
        self.cards.chars().for_each(|card| {
            let current_count = cards.get(&card).unwrap_or(&0);
            cards.insert(card, current_count + 1);
        });
        cards
    }

    fn get_hand_type(&self) -> HandType {
        let mut cards = self.create_map();
        match cards.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                let (_, v) = cards.drain().next().unwrap();
                if v == 1 || v == 4 {
                    return HandType::FourOfAKind;
                } else {
                    return HandType::FullHouse;
                }
            }
            3 => {
                for (_, v) in cards.drain() {
                    if v == 3 {
                        return HandType::ThreeOfAKind;
                    }
                }
                HandType::TwoPair
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("Only 5 cards are allowed!"),
        }
    }
}

fn convert_card_char_to_ascending_utf8_char(input: char) -> char {
    match input {
        '1'..='9' => input,
        'T' => 'A',
        'J' => 'B',
        'Q' => 'C',
        'K' => 'D',
        'A' => 'E',
        _ => panic!("{input} is an Invalid Card!"),
    }
}

fn parse_input(input: &str) -> Vec<Hand> {
    let mut hands: Vec<Hand> = vec![];
    input.lines().for_each(|l| {
        hands.push(Hand::new(l));
    });
    hands.sort_by(|a, b| a.cmp(&b));
    println!("After: {:?}", hands);
    hands
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input);
    let mut result: u32 = 0;
    input.into_iter().enumerate().for_each(|(i, hand)| {
        result += (i as u32 + 1) * hand.bid;
    });
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comparison() {
        let hand1 = Hand {
            cards: String::from("KTJJT"),
            bid: 1,
        };
        let hand2 = Hand {
            cards: String::from("KK677"),
            bid: 1,
        };
        let result = hand1.cmp(&hand2);
        assert_eq!(result, Ordering::Less)
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
