use rayon::prelude::*;
use std::fs;

#[derive(Clone, Debug, PartialEq)]
enum Tile {
    Mark,
    Open(i32),
}

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
struct Guard {
    position: (i32, i32),
    direction: Direction,
}

impl Guard {
    fn turn(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn next_pos(&self) -> (i32, i32) {
        let mut position = self.position;
        match self.direction {
            Direction::Up => position.1 -= 1,
            Direction::Down => position.1 += 1,
            Direction::Left => position.0 -= 1,
            Direction::Right => position.0 += 1,
        };

        position
    }

    fn advance(&mut self) {
        self.position = self.next_pos();
    }
}

fn parse_line(line: &str) -> Vec<Tile> {
    line.chars()
        .map(|x| match x {
            '#' => Tile::Mark,
            _ => Tile::Open(0),
        })
        .collect::<Vec<Tile>>()
}

fn parse_guard(lines: &str) -> Option<Guard> {
    for (row, line) in lines.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == '^' {
                return Some(Guard {
                    position: (col as i32, row as i32),
                    direction: Direction::Up,
                });
            }
        }
    }

    None
}

fn exited(guard: &Guard, grid: &Vec<Vec<Tile>>) -> bool {
    let (x, y) = guard.position;

    if x < 0 {
        return true;
    }

    if y < 0 {
        return true;
    }

    if y >= grid.len() as i32 {
        return true;
    }

    if x >= grid[0].len() as i32 {
        return true;
    }

    false
}

fn blocked(guard: &Guard, grid: &Vec<Vec<Tile>>) -> bool {
    let mut next = guard.clone();
    next.advance();
    if exited(&next, &grid) {
        return false;
    }

    let (x, y) = next.position;
    grid[y as usize][x as usize] == Tile::Mark
}

fn loops(mut guard: Guard, mut grid: Vec<Vec<Tile>>) -> bool {
    let mut previous_guards = vec![guard.clone()];
    loop {
        //println!("Guard: {:?}", guard);

        if exited(&guard, &grid) {
            println!("Exiting!");
            return false;
        }

        let (x, y) = guard.position;

        grid[y as usize][x as usize] = Tile::Open(1);

        if blocked(&guard, &grid) {
            guard.turn();
            if previous_guards.contains(&guard) {
                println!("It loops!");
                return true;
            }
            previous_guards.push(guard.clone());

            continue;
        }

        guard.advance();
        if previous_guards.contains(&guard) {
            println!("It loops!");
            return true;
        }
        previous_guards.push(guard.clone());
    }
    false
}

fn part2(guard: Guard, reference_grid: Vec<Vec<Tile>>, grid: Vec<Vec<Tile>>) -> i32 {
    grid.par_iter()
        .enumerate()
        .map(|i| {
            i.1.par_iter()
                .enumerate()
                .map(|j| {
                    if *j.1 == Tile::Open(1) && guard.position != (j.0 as i32, i.0 as i32) {
                        let mut new_grid = reference_grid.clone();
                        new_grid[i.0][j.0] = Tile::Mark;
                        return loops(guard.clone(), new_grid) as i32;
                    } else {
                        return 0;
                    }
                })
                .sum::<i32>()
        })
        .sum::<i32>()
}

fn main() {
    let input = fs::read_to_string("resources/day6.txt").expect("File path incorrect.");

    let parsed_grid = input.lines().map(parse_line).collect::<Vec<Vec<Tile>>>();

    let mut grid = parsed_grid.clone();

    let parsed_guard = parse_guard(&input).unwrap();

    let mut guard = parsed_guard.clone();

    loop {
        //println!("Guard: {:?}", guard);

        if exited(&guard, &grid) {
            break;
        }

        let (x, y) = guard.position;
        grid[y as usize][x as usize] = Tile::Open(1);

        if blocked(&guard, &grid) {
            guard.turn();
            continue;
        }

        guard.advance();
    }

    println!(
        "Part 1: {:?}",
        grid.iter()
            .map(|x| x
                .iter()
                .map(|y| match y {
                    Tile::Mark => {
                        0
                    }
                    Tile::Open(x) => {
                        *x
                    }
                })
                .sum::<i32>())
            .sum::<i32>()
    );
    println!("Part 2: {:}", part2(parsed_guard, parsed_grid, grid));
}
