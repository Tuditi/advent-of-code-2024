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
    cards: [u8; 5],
    bid: u32,
    with_joker: bool,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_type = self.get_hand_type();
        let other_type = other.get_hand_type();
        if self_type > other_type {
            return Ordering::Greater;
        } else if self_type < other_type {
            return Ordering::Less;
        } else {
            let mut self_chars = self.cards.iter();
            let mut other_chars = other.cards.iter();

            for _ in 0..5 {
                let self_next = self_chars.next();
                let other_next = other_chars.next();

                if let Some(self_char) = self_next {
                    if other_next.is_none() {
                        panic!("Unequal hands!")
                    }
                    let self_char = map_to_ascending_utf8_char(*self_char, self.with_joker);
                    let next_char =
                        map_to_ascending_utf8_char(*other_next.unwrap(), self.with_joker);
                    if self_char > next_char {
                        return Ordering::Greater;
                    } else if self_char < next_char {
                        return Ordering::Less;
                    }
                }
            }
            unreachable!()
        }
    }
}

impl Hand {
    fn new(l: &str, with_joker: bool) -> Self {
        let mut iterator = l.split(' ');
        let hand_char = iterator.next().unwrap();
        let bid = iterator.next().unwrap().parse::<u32>();
        let cards = hand_char
            .chars()
            .map(|c| c as u8)
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        if let Ok(bid) = bid {
            Hand {
                cards,
                bid,
                with_joker,
            }
        } else {
            panic!("{:?}", bid)
        }
    }

    fn create_map(&self) -> HashMap<u8, usize> {
        let mut cards: HashMap<u8, usize> = HashMap::new();
        self.cards.iter().for_each(|card| {
            // TODO: Optimize using and_modify()/or_insert()
            let current_count = cards.get(&card).unwrap_or(&0);
            cards.insert(*card, current_count + 1);
        });
        cards
    }

    fn get_hand_type(&self) -> HandType {
        let mut cards = self.create_map();
        let mut card_length = cards.len();

        let mut jokers: usize = 0;
        if self.with_joker {
            jokers = *cards.get(&b'J').unwrap_or(&0);
            if jokers > 0 {
                card_length -= 1;
            }
        }

        let (_, v) = cards.drain().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
        match card_length {
            0 | 1 => HandType::FiveOfAKind,
            2 => {
                if (v + jokers) == 4 || jokers == 3 {
                    return HandType::FourOfAKind;
                } else {
                    return HandType::FullHouse;
                }
            }
            3 => {
                if (v + jokers) == 3 || jokers == 2 {
                    return HandType::ThreeOfAKind;
                } else {
                    return HandType::TwoPair;
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => unimplemented!(),
        }
    }
}

fn map_to_ascending_utf8_char(input: u8, with_joker: bool) -> u8 {
    match input {
        b'2'..=b'9' => input,
        b'T' => b'A',
        b'J' => set_to_zero_if_enabled(with_joker),
        b'Q' => b'C',
        b'K' => b'D',
        b'A' => b'E',
        _ => panic!("{input} is an Invalid Card!"),
    }
}

fn set_to_zero_if_enabled(enabled: bool) -> u8 {
    match enabled {
        true => b'0',
        false => b'B',
    }
}

fn parse_input(input: &str, with_joker: bool) -> Vec<Hand> {
    let mut hands: Vec<Hand> = vec![];
    input.lines().for_each(|l| {
        hands.push(Hand::new(l, with_joker));
    });
    hands.sort_by(|a, b| a.cmp(&b));
    hands
}

fn calculate_result(hands: Vec<Hand>) -> Option<u32> {
    let mut result: u32 = 0;
    hands.into_iter().enumerate().for_each(|(i, hand)| {
        result += (i as u32 + 1) * hand.bid;
    });
    Some(result)
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input, false);
    calculate_result(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse_input(input, true);
    calculate_result(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }

    #[test]
    fn test_comparison_pt1() {
        let hand1 = Hand {
            cards: *b"KTJJT",
            bid: 1,
            with_joker: false,
        };
        let hand2 = Hand {
            cards: *b"KK677",
            bid: 1,
            with_joker: false,
        };
        let result = hand1.cmp(&hand2);
        assert_eq!(result, Ordering::Less)
    }

    #[test]
    fn test_comparison_pt2() {
        let hand1 = Hand {
            cards: *b"AK653",
            bid: 1,
            with_joker: true,
        };
        let hand2 = Hand {
            cards: *b"8KA9J",
            bid: 1,
            with_joker: true,
        };
        let result = hand1.cmp(&hand2);
        assert_eq!(result, Ordering::Less)
    }

    #[test]
    fn test_comparison_pt2_jokers() {
        let hand1 = Hand {
            cards: *b"227K7",
            bid: 1,
            with_joker: true,
        };
        let hand2 = Hand {
            cards: *b"JJ6K4",
            bid: 1,
            with_joker: true,
        };
        let result = hand1.cmp(&hand2);
        assert_eq!(result, Ordering::Less)
    }

    #[test]
    fn test_comparison_high_card() {
        let hand1 = Hand {
            cards: *b"247QA",
            bid: 1,
            with_joker: true,
        };
        let hand2 = Hand {
            cards: *b"25794",
            bid: 1,
            with_joker: true,
        };
        let result = hand1.cmp(&hand2);
        assert_eq!(result, Ordering::Less)
    }

    #[test]
    fn test_hand_type_pt2() {
        assert_eq!(
            Hand {
                cards: *b"K3K7J",
                bid: 1,
                with_joker: true,
            }
            .get_hand_type(),
            HandType::ThreeOfAKind
        );
        assert_eq!(
            Hand {
                cards: *b"QJJQ2",
                bid: 1,
                with_joker: true
            }
            .get_hand_type(),
            HandType::FourOfAKind
        );
        assert_eq!(
            Hand {
                cards: *b"JJJQ2",
                bid: 1,
                with_joker: true
            }
            .get_hand_type(),
            HandType::FourOfAKind
        );
        assert_eq!(
            Hand {
                cards: *b"32T3K",
                bid: 1,
                with_joker: true
            }
            .get_hand_type(),
            HandType::OnePair
        );
        assert_eq!(
            Hand {
                cards: *b"TJ6J5",
                bid: 1,
                with_joker: true
            }
            .get_hand_type(),
            HandType::ThreeOfAKind
        );
        assert_eq!(
            Hand {
                cards: *b"KK677",
                bid: 1,
                with_joker: true
            }
            .get_hand_type(),
            HandType::TwoPair
        );
        assert_eq!(
            Hand {
                cards: *b"KTJJT",
                bid: 1,
                with_joker: true
            }
            .get_hand_type(),
            HandType::FourOfAKind
        );
        assert_eq!(
            Hand {
                cards: *b"QQAJA",
                bid: 1,
                with_joker: true
            }
            .get_hand_type(),
            HandType::FullHouse
        );
        assert_eq!(
            Hand {
                cards: *b"JJJJJ",
                bid: 1,
                with_joker: true
            }
            .get_hand_type(),
            HandType::FiveOfAKind
        );
    }
}

// #1: pt1 170ms
