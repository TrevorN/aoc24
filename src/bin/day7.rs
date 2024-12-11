use rayon::prelude::*;
use std::fs;

fn concat(a: i64, b: i64) -> i64 {
    format!("{:}{:}", a, b).parse::<i64>().unwrap()
}

fn operator_reduce<T: Iterator<Item = i64> + Clone>(
    target: i64,
    acc: i64,
    mut remaining: T,
) -> bool {
    let new_val = remaining.next();

    if let Some(x) = new_val {
        operator_reduce(target, acc * x, remaining.clone())
            || operator_reduce(target, acc + x, remaining.clone())
            || operator_reduce(target, concat(acc, x), remaining.clone())
    } else {
        target == acc
    }
}

fn part1(line: &str) -> i64 {
    let mut s = line.split(":");
    let result = s.next().unwrap().parse::<i64>().unwrap();

    let mut arguments = s
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap());

    let first = arguments.next().unwrap();

    result * operator_reduce(result, first, arguments) as i64
}

fn main() {
    let input = fs::read_to_string("resources/day7.txt").expect("File path incorrect.");

    println!(
        "Part 1: {:}.",
        input.as_parallel_string().lines().map(part1).sum::<i64>()
    );
}
