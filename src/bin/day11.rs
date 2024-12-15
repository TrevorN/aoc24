use std::fs;

// Lol, lets try doing a linked list.
struct Stones {
    values: Vec<i64>,
}

impl Stones {
    fn advance(&mut self) {
        let mut index = 0;

        loop {
            if index >= self.values.len() {
                return;
            }

            let v = self.values[index];

            if v == 0 {
                self.values[index] = 1;
                index += 1;
                continue;
            }

            // If my value has an even number of digits, split!
            let strrep = v.to_string();
            let strrep_len = strrep.len();

            if strrep_len % 2 == 0 {
                let first = strrep[..strrep_len / 2].parse::<i64>().unwrap();
                let second = strrep[strrep_len / 2..].parse::<i64>().unwrap();

                // Mutate self to become the first stone in the split.
                self.values[index] = first;
                self.values.insert(index + 1, second);

                index += 2;
                continue;
            }

            self.values[index] *= 2024;
            index += 1;
        }
    }

    fn append(&mut self, tail: i64) {
        self.values.push(tail);
    }

    fn print(&self) {
        for v in self.values.iter() {
            print!("{}, ", v);
        }
        println!();
    }

    fn len(&self) -> i64 {
        self.values.len() as i64
    }
}

fn parse_input(instr: &str) -> Stones {
    let nums = instr
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>())
        .flatten();

    Stones {
        values: nums.collect::<Vec<i64>>(),
    }
}

fn main() {
    let input = fs::read_to_string("resources/day11.txt").unwrap();

    let mut stones = parse_input(&input);

    for i in 0..75 {
        stones.advance();
        println!("Completed blink: {}.", i);
    }

    println!("{}", stones.len());
}
