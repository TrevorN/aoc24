use std::fs;

fn test_line(line: &str) -> i32 {
    let mut nums = line
        .split_ascii_whitespace()
        .map(|x| x.parse::<i32>().unwrap());

    if !(nums.clone().is_sorted() || nums.clone().rev().is_sorted()) {
        return 0;
    }

    let mut last_value = nums.next().unwrap();

    for num in nums {
        match (num - last_value).abs() {
            0 => {
                return 0;
            }
            4.. => {
                return 0;
            }
            _ => {}
        }
        last_value = num;
    }

    1
}

fn main() {
    let input = fs::read_to_string("resources/day2.txt").expect("File path incorrect.");

    println!("Day 1: {:}", input.lines().map(test_line).sum::<i32>());
}
