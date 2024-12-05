use std::fs;

struct Ordinator {
    rules: Vec<(i32, i32)>,
}

impl Ordinator {
    fn add_rule(&mut self, rule: (i32, i32)) {
        self.rules.push(rule);
    }

    fn in_order(&self, put: (i32, i32)) -> bool {
        self.rules.iter().any(|x| *x == put)
    }

    fn out_of_order(&self, put: (i32, i32)) -> bool {
        self.rules.iter().any(|x| *x == (put.1, put.0))
    }
}

fn check_order<T: Iterator<Item = i32> + Clone>(mut order: T, ord: &Ordinator) -> bool {
    if let Some(uut) = order.next() {
        for remaining in order.clone() {
            if !ord.in_order((uut, remaining)) {
                return false;
            }
        }

        true && check_order(order, &ord)
    } else {
        true
    }
}

fn main() {
    let input = fs::read_to_string("resources/day5.txt").expect("File path incorrect.");

    let mut sections = input.split("\n\n");

    let rules = sections.next().unwrap();

    let updates = sections.next().unwrap();

    let mut ord = Ordinator { rules: Vec::new() };

    for rule in rules.lines() {
        let mut s = rule.split('|');

        ord.add_rule((
            s.next().unwrap().parse::<i32>().unwrap(),
            s.next().unwrap().parse::<i32>().unwrap(),
        ));
    }

    let mut total = 0;

    for update in updates.lines() {
        let pages = update.split(',').map(|x| x.parse::<i32>().unwrap());

        if check_order(pages.clone(), &ord) {
            let pagelist = pages.collect::<Vec<i32>>();

            total += pagelist[pagelist.len() / 2];
        }
    }

    println!("Part 1: {:}", total);
}
