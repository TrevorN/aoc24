use gcd;
use std::fs;

fn produce_nodes<T: Iterator<Item = (char, i32, i32)>>(
    val: (char, i32, i32),
    map: T,
) -> impl Iterator<Item = (i32, i32)> {
    map.map(move |i| {
        if val.0 != i.0 {
            return None;
        }

        if val.1 == i.1 && val.2 == i.2 {
            return None;
        }

        Some([
            (2 * val.1 - i.1, 2 * val.2 - i.2),
            (2 * i.1 - val.1, 2 * i.2 - val.2),
        ])
    })
    .flatten()
    .flatten()
}

fn produce_nodes_part2<T: Iterator<Item = (char, i32, i32)>>(
    val: (char, i32, i32),
    map: T,
) -> impl Iterator<Item = (i32, i32)> {
    map.map(move |i| {
        if val.0 != i.0 {
            return None;
        }

        if val.1 == i.1 && val.2 == i.2 {
            return None;
        }

        let gcd = gcd::binary_u32((val.1 - i.1).abs() as u32, (val.2 - i.2).abs() as u32) as i32;
        let x_slope = (val.1 - i.1) / gcd;
        let y_slope = (val.2 - i.2) / gcd;

        Some((-50..50).map(move |p| (val.1 + p * x_slope, val.2 + p * y_slope)))
    })
    .flatten()
    .flatten()
}

fn main() {
    let input = fs::read_to_string("resources/day8.txt").expect("File path incorrect.");

    let x_dimm = input.lines().count() as i32;
    let y_dimm = input.lines().next().unwrap().len() as i32;

    let nodes = input
        .lines()
        .enumerate()
        .map(|i| {
            i.1.chars().enumerate().map(move |j| {
                if j.1 == '.' {
                    return None;
                }

                return Some((j.1, i.0 as i32, j.0 as i32));
            })
        })
        .flatten()
        .flatten();

    let resonance = nodes
        .clone()
        .map(|x| produce_nodes_part2(x, nodes.clone()))
        .flatten()
        .filter(|x| x.0 >= 0 && x.1 >= 0 && x.0 < x_dimm && x.1 < y_dimm);

    let mut final_map = resonance.collect::<Vec<(i32, i32)>>();
    final_map.sort();
    final_map.dedup();

    for item in &final_map {
        println!("{:?}", item)
    }

    println!("{:}", final_map.len());
}
