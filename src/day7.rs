use std::{cmp::Ordering, collections::HashMap};

use enum_iterator::{all, Sequence};
use regex::Regex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Sequence, Clone)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    L9,
    L8,
    L7,
    L6,
    L5,
    L4,
    L3,
    L2,
}

impl Card {
    fn from_char(c: char) -> Card {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::L9,
            '8' => Card::L8,
            '7' => Card::L7,
            '6' => Card::L6,
            '5' => Card::L5,
            '4' => Card::L4,
            '3' => Card::L3,
            '2' => Card::L2,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Sequence)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn has_type(hand: &Hand, handtype: &HandType) -> bool {
        let pattern = match handtype {
            HandType::FiveOfAKind => vec![5],
            HandType::FourOfAKind => vec![4, 1],
            HandType::FullHouse => vec![3, 2],
            HandType::ThreeOfAKind => vec![3, 1, 1],
            HandType::TwoPair => vec![2, 2, 1],
            HandType::OnePair => vec![2, 1, 1, 1],
            HandType::HighCard => vec![1, 1, 1, 1, 1],
        };
        let mut countmap = HashMap::new();
        for card in hand.cards.iter() {
            countmap.insert(card, *countmap.get(&card).unwrap_or(&0) + 1);
        }
        let mut observered_pattern: Vec<_> = countmap.values().map(|r| *r).collect();
        observered_pattern.sort();
        observered_pattern = observered_pattern.into_iter().rev().collect();

        pattern == observered_pattern
    }

    fn has_type_joker(hand: &Hand, handtype: &HandType) -> bool {
        let pattern = match handtype {
            HandType::FiveOfAKind => vec![5],
            HandType::FourOfAKind => vec![4, 1],
            HandType::FullHouse => vec![3, 2],
            HandType::ThreeOfAKind => vec![3, 1, 1],
            HandType::TwoPair => vec![2, 2, 1],
            HandType::OnePair => vec![2, 1, 1, 1],
            HandType::HighCard => vec![1, 1, 1, 1, 1],
        };
        let mut countmap = HashMap::new();
        for card in hand.cards.iter() {
            countmap.insert(card, *countmap.get(&card).unwrap_or(&0) + 1);
        }
        let joker_count = *countmap.get(&Card::J).unwrap_or(&0);
        countmap.remove(&Card::J);
        let mut observered_pattern: Vec<_> = countmap.values().map(|r| *r).collect();
        if observered_pattern.is_empty() {
            observered_pattern.push(0);
        }
        observered_pattern.sort();
        observered_pattern = observered_pattern.into_iter().rev().collect();
        observered_pattern[0] += joker_count;

        pattern == observered_pattern
    }
}

#[derive(Debug, Clone)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn parse(string: &str) -> Hand {
        let cards = string
            .chars()
            .map(|c| Card::from_char(c))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Hand { cards }
    }

    fn get_type(&self) -> HandType {
        all::<HandType>()
            .filter(|handtype| HandType::has_type(self, handtype))
            .next()
            .unwrap()
    }

    fn get_type_joker(&self) -> HandType {
        all::<HandType>()
            .filter(|handtype| HandType::has_type_joker(self, handtype))
            .next()
            .unwrap()
    }

    fn cmp(hand1: &Hand, hand2: &Hand) -> Ordering {
        let type1 = hand1.get_type();
        let type2 = hand2.get_type();
        if type1 != type2 {
            return type1.cmp(&type2);
        }
        for (card1, card2) in hand1.cards.iter().zip(&hand2.cards) {
            if card1 != card2 {
                return card1.cmp(&card2);
            }
        }
        Ordering::Equal
    }

    fn cmp_joker(hand1: &Hand, hand2: &Hand) -> Ordering {
        let type1 = hand1.get_type_joker();
        let type2 = hand2.get_type_joker();
        if type1 != type2 {
            return type1.cmp(&type2);
        }
        for (card1, card2) in hand1.cards.iter().zip(&hand2.cards) {
            if card1 == card2 {
                continue;
            } else if *card1 == Card::J {
                return Ordering::Greater;
            } else if *card2 == Card::J {
                return Ordering::Less;
            } else {
                return card1.cmp(&card2);
            }
        }
        Ordering::Equal
    }
}

#[derive(Debug)]
struct Input {
    hands_with_bids: Vec<(Hand, u64)>,
}

impl Input {
    fn parse(input: &str) -> Input {
        let mut hands_with_bids = vec![];
        for line in input.lines() {
            let re_line = Regex::new(r"([2-9AKQJT]{5}) ([0-9]+)").unwrap();
            let captures = re_line.captures(line).unwrap();
            let hand = Hand::parse(&captures[1]);
            let bid: u64 = captures[2].parse().unwrap();

            hands_with_bids.push((hand, bid));
        }

        Input { hands_with_bids }
    }

    fn get_hands_and_bids_ranked(&self) -> Vec<(Hand, u64)> {
        let mut copy = self.hands_with_bids.clone();
        copy.sort_by(|(hand1, _), (hand2, _)| Hand::cmp(hand1, hand2));
        copy.into_iter().rev().collect()
    }

    fn calculate_total_bids(&self) -> u64 {
        self.get_hands_and_bids_ranked()
            .iter()
            .map(|(_, bid)| bid)
            .enumerate()
            .map(|(rank, bid)| (rank as u64 + 1) * bid)
            .sum()
    }

    fn get_hands_and_bids_ranked_joker(&self) -> Vec<(Hand, u64)> {
        let mut copy = self.hands_with_bids.clone();
        copy.sort_by(|(hand1, _), (hand2, _)| Hand::cmp_joker(hand1, hand2));
        copy.into_iter().rev().collect()
    }

    fn calculate_total_bids_joker(&self) -> u64 {
        self.get_hands_and_bids_ranked_joker()
            .iter()
            .map(|(_, bid)| bid)
            .enumerate()
            .map(|(rank, bid)| (rank as u64 + 1) * bid)
            .sum()
    }
}

pub fn task_one(input: String) -> u64 {
    let input = Input::parse(&input);
    input.calculate_total_bids()
}

pub fn task_two(input: String) -> u64 {
    let input = Input::parse(&input);
    input.calculate_total_bids_joker()
}
