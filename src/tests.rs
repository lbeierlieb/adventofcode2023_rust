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
