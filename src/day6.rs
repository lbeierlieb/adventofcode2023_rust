use regex::Regex;

#[derive(Debug)]
struct Input {
    times: Vec<u64>,
    distances: Vec<u64>,
}

impl Input {
    fn parse(input: &str) -> Input {
        let mut lines = input.lines();
        let times_str = lines.next().unwrap();
        let times = numstring_to_numbers(times_str);
        let distance_str = lines.next().unwrap();
        let distances = numstring_to_numbers(distance_str);

        Input { times, distances }
    }

    fn merge_times_and_distances(&self) -> Input {
        let merged_time = concat_numbers(&self.times);
        let merged_distance = concat_numbers(&self.distances);

        Input {
            times: vec![merged_time],
            distances: vec![merged_distance],
        }
    }

    fn calculate_all_possibilities(&self) -> u64 {
        self.times
            .iter()
            .zip(&self.distances)
            .map(|(time, dist)| calculate_boat_win_possibilities(*dist, *time))
            .product()
    }
}

fn concat_numbers(nums: &[u64]) -> u64 {
    let concat: String = nums.iter().map(|num| num.to_string()).collect();
    concat.parse().unwrap()
}

fn calculate_boat_distance(total_time: u64, charge_time: u64) -> u64 {
    let move_time = total_time - charge_time;
    let speed = charge_time;

    move_time * speed
}

fn calculate_boat_win(distance_to_beat: u64, total_time: u64, charge_time: u64) -> bool {
    let distance = calculate_boat_distance(total_time, charge_time);
    distance > distance_to_beat
}

fn calculate_boat_win_possibilities(distance_to_beat: u64, total_time: u64) -> u64 {
    (0..total_time)
        .filter(|charge_time| calculate_boat_win(distance_to_beat, total_time, *charge_time))
        .count() as u64
}

fn numstring_to_numbers(input: &str) -> Vec<u64> {
    let re_num = Regex::new(r"[0-9]+").unwrap();
    re_num
        .captures_iter(input)
        .filter_map(|cap| cap[0].parse::<u64>().ok())
        .collect()
}

pub fn task_one(input: String) -> u64 {
    Input::parse(&input).calculate_all_possibilities()
}

pub fn task_two(input: String) -> u64 {
    Input::parse(&input)
        .merge_times_and_distances()
        .calculate_all_possibilities()
}
