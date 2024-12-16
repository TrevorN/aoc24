use std::fs;

struct Memory {
    table: Vec<(i64, i64, i64)>,
}

impl Memory {
    fn new() -> Memory {
        Memory { table: vec![] }
    }

    fn recall(&self, height: i64, value: i64) -> Option<i64> {
        for (h, v, total) in self.table.iter() {
            if *h == height && *v == value {
                println!("I WEMEMBER!");
                return Some(*total);
            }
        }

        None
    }

    fn memorize(&mut self, height: i64, value: i64, total: i64) {
        // I don't need to worry about duplicates because the calling functions would have already
        // called "recall".
        self.table.push((height, value, total));
    }
}

fn count_stones(height: i64, value: i64, mem: &mut Memory) -> i64 {
    println!("h: {}, v: {}", height, value);
    if height == 0 {
        return 1;
    }

    if let Some(count) = mem.recall(height, value) {
        return count;
    }

    if value == 0 {
        let result = count_stones(height - 1, 1, mem);
        mem.memorize(height, value, result);
        return result;
    }

    let strval = value.to_string();
    println!("{} -> {}", value, strval);
    if strval.len() % 2 == 0 {
        let first = strval[..strval.len() / 2].parse::<i64>().unwrap();
        let second = strval[strval.len() / 2..].parse::<i64>().unwrap();
        println!("{} -> {}, {}", value, first, second);

        let first_result = count_stones(height - 1, first, mem);
        let second_result = count_stones(height - 1, second, mem);
        mem.memorize(height, value, first_result + second_result);
        return first_result + second_result;
    }

    let result = count_stones(height - 1, value * 2024, mem);
    mem.memorize(height, value, result);
    result
}

fn parse_input(instr: &str) -> Vec<i64> {
    instr
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>())
        .flatten()
        .collect::<Vec<i64>>()
}

fn main() {
    let input = fs::read_to_string("resources/day11.txt").unwrap();

    let stones = parse_input(&input);

    println!("{:?}", stones);

    let mut memory = Memory::new();

    // Instead of breadth first, we want to go depth first!
    println!(
        "Total: {}",
        stones
            .iter()
            .map(|x| count_stones(75, *x, &mut memory))
            .sum::<i64>()
    );
}
