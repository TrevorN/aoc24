use std::fs;
use std::iter::zip;

// Given a kernel and a search space, it counts the number of occurances in the space.
fn convolutional_count(kernel: &Vec<Vec<Option<u8>>>, search_space: &[&[u8]]) -> i32 {
    let kernel_height = kernel.len();
    let kernel_width = kernel[0].len();

    let search_space_height = search_space.len();
    let search_space_width = search_space.len();

    let mut result = 0;

    for x in 0..=search_space_width - kernel_width {
        'coord: for y in 0..=search_space_height - kernel_height {
            for i in 0..kernel_width {
                for j in 0..kernel_height {
                    if let Some(tile) = kernel[j][i] {
                        if tile != search_space[y + j][x + i] {
                            continue 'coord;
                        }
                    }
                }
            }
            result += 1;
        }
    }

    result
}

// Generate a vector of kernels.
fn generate_kernels() -> Vec<Vec<Vec<Option<u8>>>> {
    let mut result = Vec::new();

    // L/R
    result.push(vec![vec![Some(b'X'), Some(b'M'), Some(b'A'), Some(b'S')]]);
    result.push(vec![vec![Some(b'S'), Some(b'A'), Some(b'M'), Some(b'X')]]);

    // U/D
    result.push(vec![
        vec![Some(b'X')],
        vec![Some(b'M')],
        vec![Some(b'A')],
        vec![Some(b'S')],
    ]);
    result.push(vec![
        vec![Some(b'S')],
        vec![Some(b'A')],
        vec![Some(b'M')],
        vec![Some(b'X')],
    ]);

    // Diagonals
    let up: &[usize] = &[0, 1, 2, 3];
    let down: &[usize] = &[3, 2, 1, 0];
    for v_x in [down, up] {
        for v_y in [&down, &up] {
            let mut diag = vec![vec![None; 4]; 4];

            for ((x, y), c) in zip(zip(v_x.iter(), v_y.iter()), [b'X', b'M', b'A', b'S'].iter()) {
                diag[*x][*y] = Some(*c)
            }

            result.push(diag)
        }
    }

    result
}

// Generate a vector of kernels.
fn generate_kernels_part2() -> Vec<Vec<Vec<Option<u8>>>> {
    let mut result = Vec::new();

    // Diagonals
    for v_x in [true, false] {
        for v_y in [true, false] {
            let mut diag = vec![vec![None; 3]; 3];
            diag[1][1] = Some(b'A');

            if v_x {
                diag[0][0] = Some(b'M');
                diag[2][2] = Some(b'S');
            } else {
                diag[0][0] = Some(b'S');
                diag[2][2] = Some(b'M');
            }

            if v_y {
                diag[0][2] = Some(b'M');
                diag[2][0] = Some(b'S');
            } else {
                diag[0][2] = Some(b'S');
                diag[2][0] = Some(b'M');
            }

            result.push(diag)
        }
    }

    result
}
// Count all occurances of XMAS in all directions in the puzzle input.
fn count_xmas(puzzle: &str, kernels: Vec<Vec<Vec<Option<u8>>>>) -> i32 {
    let puzzle_vec = puzzle.lines().map(|x| x.as_bytes()).collect::<Vec<&[u8]>>();

    kernels
        .iter()
        .map(|x| convolutional_count(x, &puzzle_vec))
        .sum()
}

fn main() {
    let input = fs::read_to_string("resources/day4.txt").expect("File path incorrect.");

    println!("Part 1: {:}", count_xmas(&input, generate_kernels()));
    println!("Part 2: {:}", count_xmas(&input, generate_kernels_part2()));
}
