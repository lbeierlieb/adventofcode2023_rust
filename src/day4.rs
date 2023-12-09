use std::collections::HashMap;

use regex::Captures;
use regex::Regex;

#[derive(Debug)]
struct Card {
    id: u64,
    winning: Vec<u64>,
    nums: Vec<u64>,
}

fn numstring_to_numbers(input: &str) -> Vec<u64> {
    let re_num = Regex::new(r"[0-9]+").unwrap();
    re_num
        .captures_iter(input)
        .filter_map(|cap| cap[0].parse::<u64>().ok())
        .collect()
}

impl Card {
    fn parse_from_line(line: &str) -> Card {
        let re_line =
            Regex::new(r"Card *([0-9]+):((?: [ 0-9][0-9])+) \|((?: [ 0-9][0-9])+)").unwrap();
        let captures = re_line.captures(line).unwrap();
        let id = captures[1].parse().unwrap();
        let winning = numstring_to_numbers(&captures[2]);
        let nums = numstring_to_numbers(&captures[3]);

        Card { id, winning, nums }
    }

    fn get_score(&self) -> u64 {
        match self.get_num_winning() {
            0 => 0,
            count => (2 as u64).pow(count as u32 - 1),
        }
    }

    fn get_num_winning(&self) -> usize {
        self.winning
            .iter()
            .filter(|win| self.nums.iter().any(|num| *win == num))
            .count()
    }
}

pub fn task_one(input: String) -> u64 {
    input
        .lines()
        .map(Card::parse_from_line)
        .map(|card| card.get_score())
        .sum()
}

pub fn task_two(input: String) -> u64 {
    let cards: Vec<_> = input.lines().map(Card::parse_from_line).collect();
    let mut card_counts = HashMap::new();
    for card in cards.iter() {
        card_counts.insert(card.id, 1);
    }
    for card in cards.iter() {
        let id = card.id;
        let count = *card_counts.get(&id).unwrap();
        let winning_count = card.get_num_winning();
        for i in 1..=winning_count {
            let next_id = id + i as u64;
            if let Some(card_count) = card_counts.get(&next_id) {
                card_counts.insert(next_id, card_count + count);
            }
        }
    }

    card_counts.values().sum()
}
