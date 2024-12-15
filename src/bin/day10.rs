use std::fs;

fn read_input(map: &str) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    for line in map.lines() {
        let mut ch = Vec::new();
        for c in line.chars() {
            ch.push(c.to_digit(10).unwrap() as i32);
        }
        result.push(ch);
    }

    result
}

fn count_destinations_inner(map: &Vec<Vec<i32>>, start: (i32, i32), dest: &mut Vec<(i32, i32)>) {
    let (x, y) = start;

    if map[x as usize][y as usize] == 9 {
        println!("Found a destination!");
        dest.push(start);
        return;
    }

    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let new_x = x + dx;
        let new_y = y + dy;
        let xdim = map.len() as i32;
        let ydim = map[0].len() as i32;

        if new_x < 0 || new_y < 0 || new_x >= xdim || new_y >= ydim {
            //println!("Map at {:}, {:} is OOB.", new_x, new_y);
            continue;
        }

        if map[new_x as usize][new_y as usize] == map[x as usize][y as usize] + 1 {
            println!("{:}, {:} is uphill from {:}, {:}.", new_x, new_y, x, y);
            count_destinations_inner(map, (new_x, new_y), dest);
        }
    }
}

fn count_destinations(map: &Vec<Vec<i32>>, start: (i32, i32)) -> i32 {
    let mut destinations = vec![];
    count_destinations_inner(map, start, &mut destinations);
    // Count the number of unique entries in result.
    println!(
        "Destinations for {:}, {:}: {:?}",
        start.0, start.1, destinations
    );
    destinations.sort();
    destinations.dedup();
    destinations.len() as i32
}

fn count_destinations_part2(map: &Vec<Vec<i32>>, start: (i32, i32)) -> i32 {
    let mut destinations = vec![];
    count_destinations_inner(map, start, &mut destinations);
    // Count the number of unique entries in result.
    println!(
        "Destinations for {:}, {:}: {:?}",
        start.0, start.1, destinations
    );
    destinations.len() as i32
}

fn count_trails(map: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;

    for (i, row) in map.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == 0 {
                println!("Counting from {:}, {:}.", i, j);
                result += count_destinations(map, (i as i32, j as i32));
            }
        }
    }

    result
}

fn count_trails_part2(map: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;

    for (i, row) in map.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == 0 {
                println!("Counting from {:}, {:}.", i, j);
                result += count_destinations_part2(map, (i as i32, j as i32));
            }
        }
    }

    result
}

fn main() {
    let input = fs::read_to_string("resources/day10.txt").unwrap();

    let map = read_input(&input);

    map.iter()
        .map(|x| {
            println!("{:?}", x);
        })
        .collect::<Vec<_>>();

    println!("Part 1: {:}", count_trails(&map));
    println!("Part 2: {:}", count_trails_part2(&map));
}
