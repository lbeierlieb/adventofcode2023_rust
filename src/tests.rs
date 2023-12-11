use std::fs::read_to_string;


#[test]
fn day1_task1() {
    let input = read_to_string("inputs/day1_task1_test").unwrap();
    let result = 142;
    assert_eq!(result, crate::day1::task_one(input));
}

#[test]
fn day1_task2() {
    let input = read_to_string("inputs/day1_task2_test").unwrap();
    let result = 281;
    assert_eq!(result, crate::day1::task_two(input));
}

#[test]
fn day2_task1() {
    let input = read_to_string("inputs/day2_task1_test").unwrap();
    let result = 8;
    assert_eq!(result, crate::day2::task_one(input));
}

#[test]
fn day2_task2() {
    let input = read_to_string("inputs/day2_task2_test").unwrap();
    let result = 2286;
    assert_eq!(result, crate::day2::task_two(input));
}

#[test]
fn day3_task1() {
    let input = read_to_string("inputs/day3_task1_test").unwrap();
    let result = 4361;
    assert_eq!(result, crate::day3::task_one(input));
}

#[test]
fn day3_task2() {
    let input = read_to_string("inputs/day3_task2_test").unwrap();
    let result = 467835;
    assert_eq!(result, crate::day3::task_two(input));
}

#[test]
fn day4_task1() {
    let input = read_to_string("inputs/day4_task1_test").unwrap();
    let result = 13;
    assert_eq!(result, crate::day4::task_one(input));
}

#[test]
fn day4_task2() {
    let input = read_to_string("inputs/day4_task2_test").unwrap();
    let result = 30;
    assert_eq!(result, crate::day4::task_two(input));
}

#[test]
fn day5_task1() {
    let input = read_to_string("inputs/day5_task1_test").unwrap();
    let result = 35;
    assert_eq!(result, crate::day5::task_one(input));
}

#[test]
fn day5_task2() {
    let input = read_to_string("inputs/day5_task2_test").unwrap();
    let result = 46;
    assert_eq!(result, crate::day5::task_two(input));
}

#[test]
fn day6_task1() {
    let input = read_to_string("inputs/day6_task1_test").unwrap();
    let result = 288;
    assert_eq!(result, crate::day6::task_one(input));
}

#[test]
fn day6_task2() {
    let input = read_to_string("inputs/day6_task2_test").unwrap();
    let result = 71503;
    assert_eq!(result, crate::day6::task_two(input));
}

#[test]
fn day7_task1() {
    let input = read_to_string("inputs/day7_task1_test").unwrap();
    let result = 6440;
    assert_eq!(result, crate::day7::task_one(input));
}

#[test]
fn day7_task2() {
    let input = read_to_string("inputs/day7_task2_test").unwrap();
    let result = 5905;
    assert_eq!(result, crate::day7::task_two(input));
}

#[test]
fn day8_task1() {
    let input = read_to_string("inputs/day8_task1_test").unwrap();
    let result = 2;
    assert_eq!(result, crate::day8::task_one(input));
    let input = read_to_string("inputs/day8_task1_test2").unwrap();
    let result = 6;
    assert_eq!(result, crate::day8::task_one(input));
}

#[test]
fn day8_task2() {
    let input = read_to_string("inputs/day8_task2_test").unwrap();
    let result = 6;
    assert_eq!(result, crate::day8::task_two(input));
}
