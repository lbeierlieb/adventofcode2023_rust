use crate::day1::{task_one, task_two, self};
use std::fs::read_to_string;


#[test]
fn day1_task1() {
    let input = read_to_string("inputs/day1_task1_test").unwrap();
    let result = 142;
    assert_eq!(result, day1::task_one(input));
}

#[test]
fn day1_task2() {
    let input = read_to_string("inputs/day1_task2_test").unwrap();
    let result = 281;
    assert_eq!(result, day1::task_two(input));
}
