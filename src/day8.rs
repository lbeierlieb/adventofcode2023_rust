use num::integer::lcm;
use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!(),
        }
    }

    fn select_destination<'a>(&self, destinations: &'a (String, String)) -> &'a String {
        match &self {
            Direction::Left => &destinations.0,
            Direction::Right => &destinations.1,
        }
    }
}

#[derive(Debug)]
struct Input {
    directions: Vec<Direction>,
    graph: HashMap<String, (String, String)>,
}

impl Input {
    fn parse_graph_line(line: &str) -> (String, String, String) {
        let re_line = Regex::new(r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)").unwrap();
        let captures = re_line.captures(line).unwrap();
        (
            captures[1].to_string(),
            captures[2].to_string(),
            captures[3].to_string(),
        )
    }

    fn parse(input: &str) -> Input {
        let mut lines = input.lines();
        let directions_str = lines.next().unwrap();
        let directions = directions_str.chars().map(Direction::from_char).collect();

        lines.next(); // skip blank line

        let mut graph = HashMap::new();
        for (src, left, right) in lines.map(Input::parse_graph_line) {
            graph.insert(src, (left, right));
        }

        Input { directions, graph }
    }

    fn get_steps_AAA_to_ZZZ(&self) -> u64 {
        let mut counter = 0;
        let mut pos = "AAA";
        for dir in self.directions.iter().cycle() {
            let destinations = self.graph.get(pos).unwrap();
            let dest = dir.select_destination(destinations);
            counter += 1;
            if dest == "ZZZ" {
                break;
            }
            pos = dest;
        }
        counter
    }

    fn position_ends_on(name: &str, end: char) -> bool {
        name.chars().nth(2).unwrap() == end
    }

    fn get_steps_xxA_to_xxZ(&self, start: &str) -> u64 {
        let mut counter = 0;
        let mut pos = start;
        for dir in self.directions.iter().cycle() {
            let destinations = self.graph.get(pos).unwrap();
            let dest = dir.select_destination(destinations);
            counter += 1;
            if Input::position_ends_on(dest, 'Z') {
                break;
            }
            pos = dest;
        }
        counter
    }

    fn get_steps_to_target_multi_bruteforce(&self) -> u64 {
        let mut positions: Vec<_> = self
            .graph
            .keys()
            .filter(|pos| Input::position_ends_on(pos, 'A'))
            .collect();
        let mut counter = 0;
        for dir in self.directions.iter().cycle() {
            for pos in &mut positions {
                let destinations = self.graph.get(*pos).unwrap();
                let dest = dir.select_destination(destinations);
                *pos = dest;
            }
            counter += 1;
            if positions
                .iter()
                .all(|pos| Input::position_ends_on(pos, 'Z'))
            {
                break;
            }
        }
        counter
    }

    fn get_steps_to_target_multi(&self) -> u64 {
        self.graph
            .keys()
            .filter(|pos| Input::position_ends_on(pos, 'A'))
            .map(|pos| self.get_steps_xxA_to_xxZ(pos))
            .fold(1, |acc, n| lcm(acc, n))
    }
}

pub fn task_one(input: String) -> u64 {
    Input::parse(&input).get_steps_AAA_to_ZZZ()
}

pub fn task_two(input: String) -> u64 {
    Input::parse(&input).get_steps_to_target_multi()
}
