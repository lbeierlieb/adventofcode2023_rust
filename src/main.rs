use std::fs::read_to_string;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

#[cfg(test)] 
mod tests;

fn main() {
    println!("1.1: {}", day1::task_one(read_to_string("inputs/day1").unwrap()));
    println!("1.2: {}", day1::task_two(read_to_string("inputs/day1").unwrap()));
    println!("2.1: {}", day2::task_one(read_to_string("inputs/day2").unwrap()));
    println!("2.2: {}", day2::task_two(read_to_string("inputs/day2").unwrap()));
    println!("3.1: {}", day3::task_one(read_to_string("inputs/day3").unwrap()));
    println!("3.2: {}", day3::task_two(read_to_string("inputs/day3").unwrap()));
    println!("4.1: {}", day4::task_one(read_to_string("inputs/day4").unwrap()));
    println!("4.2: {}", day4::task_two(read_to_string("inputs/day4").unwrap()));
    println!("5.1: {}", day5::task_one(read_to_string("inputs/day5").unwrap()));
    println!("5.2: skipped");//{}", day5::task_two(read_to_string("inputs/day5").unwrap()));
    println!("6.1: {}", day6::task_one(read_to_string("inputs/day6").unwrap()));
    println!("6.2: {}", day6::task_two(read_to_string("inputs/day6").unwrap()));
    println!("7.1: {}", day7::task_one(read_to_string("inputs/day7").unwrap()));
    println!("7.2: {}", day7::task_two(read_to_string("inputs/day7").unwrap()));
    println!("8.1: {}", day8::task_one(read_to_string("inputs/day8").unwrap()));
    println!("8.2: {}", day8::task_two(read_to_string("inputs/day8").unwrap()));
    println!("9.1: {}", day9::task_one(read_to_string("inputs/day9").unwrap()));
    println!("9.2: {}", day9::task_two(read_to_string("inputs/day9").unwrap()));
}
