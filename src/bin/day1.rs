use std::fs;
use std::iter::zip;

fn main() {
    let input = fs::read_to_string("resources/day1.txt").expect("File path incorrect.");

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines(){
        let mut nums = line.split_ascii_whitespace();
        list1.push(nums.next().unwrap());
        list2.push(nums.next().unwrap());
    }

    list1.sort();
    list2.sort();

    let mut sum = 0;

    for (a, b) in zip(list1, list2) {
        sum += (a.parse::<i32>().unwrap() - b.parse::<i32>().unwrap()).abs();
    }

    println!(
        "Part One: {:?}",
        sum
    );
}
