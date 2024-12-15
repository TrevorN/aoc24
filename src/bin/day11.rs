use std::fs;
use std::mem;

// Lol, lets try doing a linked list.
struct Stone {
    value: i64,
    next: Option<Box<Stone>>
}

impl Stone {
    fn advance(&mut self) {
        // Just update value if I'm zero.
        if self.value == 0 {
            self.value = 1;
            if let Some(next) = &mut self.next {
                next.advance();
            }
            return;
        }

        // If my value has an even number of digits, split!
        let strrep = self.value.to_string();
        let strrep_len = strrep.len();

        if strrep_len % 2 == 0 {
            let first = strrep[..strrep_len/2].parse::<i64>().unwrap();
            let second = strrep[strrep_len/2..].parse::<i64>().unwrap();

            // Mutate self to become the first stone in the split.
            self.value = first;
          
            if let Some(next) = &mut self.next {
                next.advance();
            }

            let mut next = None;
            mem::swap(&mut next, &mut self.next);
            
            let mut new_stone = Some(Box::new(Stone{ value: second, next: next}));
            mem::swap(&mut self.next, &mut new_stone);

            return;
        }

        self.value *= 2024;
        if let Some(next) = &mut self.next {
            next.advance();
        }
    }

    fn append(&mut self, tail: Box<Stone>) {
        if let Some(n) = &mut self.next {
            n.append(tail);
        } else {
            self.next = Some(tail);
        }
    }

    fn print(&self) {
        print!("{} ", self.value);
        if let Some(n) = &self.next {
            n.print();
            return;
        }
        println!();
    }

    fn len(&self) -> i64 {
        if let Some(n) = &self.next {
            1 + n.len()
        } else {
            1
        }
    }
}

fn parse_input(instr: &str) -> Stone {
    let mut nums = instr.split_ascii_whitespace().map(|x| x.parse::<i64>()).flatten();

    let mut result = Stone{ value: nums.next().expect("There will be at least one number in the input"), next: None };

    for num in nums {
        let next_stone = Box::new(Stone{value: num, next: None}); 
        result.append(next_stone);
    }

    result
}

fn main() {
    let input = fs::read_to_string("resources/day11.txt").unwrap();

    let mut stones = parse_input(&input);
    stones.print();

    for _ in 0..25 {
        stones.advance();
        stones.print();
    }

    println!("{}", stones.len());
}
