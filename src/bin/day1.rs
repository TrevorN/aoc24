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

    for (a, b) in zip(list1.iter(), list2.iter()) {
        sum += (a.parse::<i32>().unwrap() - b.parse::<i32>().unwrap()).abs();
    }

    println!(
        "Part One: {:?}",
        sum
    );

    // Part 2
    let nums1 = list1.into_iter().map(|x| x.parse::<i32>().unwrap());
    let nums2 = list2.into_iter().map(|x| x.parse::<i32>().unwrap());

    let mut counts = Vec::new();

    let mut last_num = -1;
    let mut count = 0;

    for num in nums2 {
        if num != last_num {
            counts.push((last_num, count));
            println!("{:?}: {:?}", last_num, count);
            count = 1;
            last_num = num;
        } else {
            count += 1;
        }
    }

    println!("Part Two: {:?}", nums1.map(|x|
            {
                for (n, c) in counts.iter() {
                    if *n == x {
                        return *n * *c;
                    }
                }

                return 0;
            }
            ).sum::<i32>());
}
