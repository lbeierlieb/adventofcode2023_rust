use rayon::prelude::*;
use std::io::BufRead;
use std::str::Lines;

use regex::Captures;
use regex::Regex;

#[derive(Debug)]
struct MappingRule {
    dest_start: u64,
    src_start: u64,
    len: u64,
}

impl MappingRule {
    fn map(&self, num: u64) -> Option<u64> {
        if num < self.src_start || num >= self.src_start + self.len {
            None
        } else {
            Some(num - self.src_start + self.dest_start)
        }
    }

    fn parse(line: &str) -> MappingRule {
        let nums = numstring_to_numbers(line);
        let dest_start = nums[0];
        let src_start = nums[1];
        let len = nums[2];
        MappingRule {
            dest_start,
            src_start,
            len,
        }
    }
}

#[derive(Debug)]
struct Map {
    rules: Vec<MappingRule>,
}

impl Map {
    fn map(&self, num: u64) -> u64 {
        self.rules
            .iter()
            .find_map(|rule| rule.map(num))
            .unwrap_or(num)
    }

    fn parse(lines: &[&str]) -> Map {
        let rules = lines.iter().map(|line| MappingRule::parse(line)).collect();
        Map { rules }
    }
}

#[derive(Debug)]
struct MapChain {
    maps: Vec<Map>,
}

impl MapChain {
    fn map(&self, num: u64) -> u64 {
        self.maps.iter().fold(num, |acc, m| m.map(acc))
    }

    fn parse(mut lines: Lines) -> MapChain {
        let mut line_buffer = Vec::new();
        let mut maps = Vec::new();

        loop {
            match lines.next() {
                None => break,
                Some("") => {
                    maps.push(Map::parse(&line_buffer[1..]));
                    line_buffer.clear();
                }
                Some(non_empty_line) => line_buffer.push(non_empty_line),
            }
        }
        maps.push(Map::parse(&line_buffer[1..]));

        MapChain { maps }
    }
}

#[derive(Debug)]
struct Input {
    seeds: Vec<u64>,
    map_chain: MapChain,
}

impl Input {
    fn parse(input: &str) -> Input {
        let mut line_iter = input.lines();
        let seedline = line_iter.next().unwrap();
        let seeds = numstring_to_numbers(seedline);

        line_iter.next(); // just empty line
        let map_chain = MapChain::parse(line_iter);

        Input { seeds, map_chain }
    }

    fn get_lowest_transformed_seed(&self) -> u64 {
        self.seeds
            .iter()
            .map(|num| self.map_chain.map(*num))
            .min()
            .unwrap()
    }

    fn get_lowest_transformed_seed_range(&self) -> u64 {
        let start1 = self.seeds[0];
        let len1 = self.seeds[1];
        let iter1 = (start1..(start1 + len1)).into_par_iter();
        let start2 = self.seeds[0];
        let len2 = self.seeds[1];
        let iter2 = start2..(start2 + len2);
        iter1
            .chain(iter2)
            .map(|num| self.map_chain.map(num))
            .min()
            .unwrap()
    }
}

fn numstring_to_numbers(input: &str) -> Vec<u64> {
    let re_num = Regex::new(r"[0-9]+").unwrap();
    re_num
        .captures_iter(input)
        .filter_map(|cap| cap[0].parse::<u64>().ok())
        .collect()
}

pub fn task_one(input: String) -> u64 {
    Input::parse(&input).get_lowest_transformed_seed()
}

pub fn task_two(input: String) -> u64 {
    Input::parse(&input).get_lowest_transformed_seed_range()
}
