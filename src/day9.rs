use regex::Regex;

#[derive(Debug, Clone)]
struct TimeSeries {
    values: Vec<i64>,
}

impl TimeSeries {
    fn parse(input: &str) -> TimeSeries {
        let re_num = Regex::new(r"-?[0-9]+").unwrap();
        let values = re_num
            .captures_iter(input)
            .filter_map(|cap| cap[0].parse::<i64>().ok())
            .collect();
        TimeSeries { values }
    }

    fn get_differences(&self) -> TimeSeries {
        let values = (0..(self.values.len() - 1))
            .map(|i| self.values[i + 1] - self.values[i])
            .collect();
        TimeSeries { values }
    }

    fn get_with_all_differences(&self) -> Vec<TimeSeries> {
        let mut result = vec![self.clone()];

        let mut diff = self.get_differences();
        while !diff.is_all_zeros() {
            result.push(diff.clone());
            diff = diff.get_differences();
        }

        result
    }

    fn is_all_zeros(&self) -> bool {
        self.values.iter().all(|i| *i == 0)
    }

    fn predict_next_value(&self) -> i64 {
        self.get_with_all_differences()
            .iter()
            .map(|ts| ts.values.last().unwrap())
            .sum()
    }

    fn predict_previous_value(&self) -> i64 {
        let mut acc = 0;
        for ts in self.get_with_all_differences().iter().rev() {
            acc = ts.values.first().unwrap() - acc;
        }
        acc
    }
}

fn process(input: String, prediction: fn(&TimeSeries) -> i64) -> u64 {
    input
        .lines()
        .map(|line| TimeSeries::parse(line))
        .map(|ts| prediction(&ts))
        .sum::<i64>() as u64
}

pub fn task_one(input: String) -> u64 {
    process(input, TimeSeries::predict_next_value)
}

pub fn task_two(input: String) -> u64 {
    process(input, TimeSeries::predict_previous_value)
}
