fn find_first_character(line: &str) -> u32 {
    let c = line.chars().find(|c| c.is_numeric()).unwrap();
    c.to_digit(10).unwrap()
}

fn find_last_character(line: &str) -> u32 {
    let c = line.chars().rev().find(|c| c.is_numeric()).unwrap();
    c.to_digit(10).unwrap()
}

pub fn task_one(input: String) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let sum: u32 = lines.iter().map(|line| find_first_character(line)*10 + find_last_character(line)).sum(); 
    sum as u64
}

fn replace_numberstring(line: &str) -> String {
    let words: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let digits: [&str; 9] = ["o1e", "t2o", "t3e", "f4r", "f5e", "s6x", "s7n", "e8t", "n9e"]; // hacky workaround from https://www.reddit.com/r/adventofcode/comments/1884fpl/comment/kbiywz6/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

    let mut s = line.to_string();
    
    loop {
        let res = (0..words.len()).map(|i| (i, s.find(words[i]))).filter(|(_i, opt)| opt.is_some()).min_by(|(_i1, opt1), (_i2, opt2)| opt1.unwrap().cmp(&opt2.unwrap()));
        match res {
            None => return s,
            Some((i, _)) => {s = s.replacen(words[i], digits[i], 1)},
        }
    }
}

pub fn task_two(input: String) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let replaced_lines: Vec<_> = lines.iter().map(|line| replace_numberstring(line)).collect();
    let sum: u32 = replaced_lines.iter().map(|line| find_first_character(line)*10 + find_last_character(line)).sum(); 
    sum as u64
}
