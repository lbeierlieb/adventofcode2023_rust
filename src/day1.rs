fn map_only_digits(line: &str) -> Option<u32> {
    line.chars().next().unwrap().to_digit(10)
}

fn map_digits_and_words(line: &str) -> Option<u32> {
    if let Some(digit) = map_only_digits(line) {
        return Some(digit);
    }

    let words: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for i in 0..words.len() {
        if line.starts_with(words[i]) {
            return Some((i + 1) as u32);
        }
    }
    None
}

fn extract_numbers(line: &str, mapping_function: fn(&str) -> Option<u32>) -> Vec<u32> {
    (0..line.len()).filter_map(|i| mapping_function(&line.chars().skip(i).take(5).collect::<String>())).collect()
}

 fn process(input: String, mapping_function: fn(&str) -> Option<u32>) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let numbers = lines.iter().map(|line| extract_numbers(line, mapping_function));
    let first_and_last = numbers.map(|nums| nums[0] * 10 + nums[nums.len()-1]);
    let sum: u32 = first_and_last.sum();
    sum as u64
 }

pub fn task_one(input: String) -> u64 {
    process(input, map_only_digits)
}

pub fn task_two(input: String) -> u64 {
    process(input, map_digits_and_words)
}
