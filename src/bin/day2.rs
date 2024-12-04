use std::fs;

fn test_line<I: Iterator<Item = i32> + Clone + DoubleEndedIterator>(nums: I) -> i32 {
    if !(nums.clone().is_sorted() || nums.clone().rev().is_sorted()) {
        return 0;
    }

    let mut nums = nums.clone();
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

    println!(
        "Part 1: {:}",
        input
            .lines()
            .map(|x| test_line(
                x.split_ascii_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
            ))
            .sum::<i32>()
    );

    // Day 2, Brute Force :(
    let mut count = 0;
    'outer: for line in input.lines() {
        let nums = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap());
        if test_line(nums.clone()) == 1 {
            count += 1;
            continue;
        }

        let nums = nums.collect::<Vec<i32>>();
        for i in 0..nums.len() {
            let mut one_less = nums.clone();
            one_less.remove(i);

            if test_line(one_less.into_iter()) == 1 {
                count += 1;
                continue 'outer;
            }
        }
    }

    println!("Part 2: {:}", count);
}
