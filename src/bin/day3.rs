use std::fs;

fn parse_digit(digit: &str) -> Option<(i32, i32)> {
    let mut idx: i32 = 0;
    let mut result: i32 = 0;

    for c in digit.chars() {
        if !c.is_ascii_digit() {
            break;
        }

        idx += 1;
        result *= 10;
        result += c.to_digit(10)? as i32;
    }

    if idx == 0 {
        return None;
    }

    Some((result, idx))
}

// Return None if there are no remaining matches.
fn find_all_muls(memory: &str) -> Option<i32> {
    let mul_idx = memory.find("mul(")?;

    let first_int_chars = memory.get(mul_idx + 4..)?;

    let Some((first_int, comma_idx)) = parse_digit(first_int_chars) else {
        return find_all_muls(first_int_chars);
    };

    let Some(comma_chars) = first_int_chars.get(comma_idx as usize..) else {
        return find_all_muls(first_int_chars);
    };

    let Some(second_int_chars) = comma_chars.strip_prefix(",") else {
        return find_all_muls(comma_chars);
    };

    let Some((second_int, paren_idx)) = parse_digit(second_int_chars) else {
        return find_all_muls(second_int_chars);
    };

    let Some(close_paren_chars) = second_int_chars.get(paren_idx as usize..) else {
        return find_all_muls(second_int_chars);
    };

    let Some(next_string) = close_paren_chars.strip_prefix(")") else {
        return find_all_muls(close_paren_chars);
    };

    println!("{:}, {:}", first_int, second_int);

    Some(first_int * second_int + find_all_muls(next_string).unwrap_or(0))
}

fn find_all_muls_part2(memory: &str, enabled: bool) -> Option<i32> {
    // If we're disabled, scan until we find a do, then resume.
    if !enabled {
        let do_idx = memory.find("do()")?;
        println!("enabling");
        return find_all_muls_part2(memory.get(do_idx + 4..)?, true);
    }

    let mul_idx = memory.find("mul(")?;

    if let Some(dont_idx) = memory.find("don't()") {
        if dont_idx < mul_idx {
            println!("disabling");
            return find_all_muls_part2(memory.get(dont_idx + 6..)?, false);
        }
    }

    let first_int_chars = memory.get(mul_idx + 4..)?;

    let Some((first_int, comma_idx)) = parse_digit(first_int_chars) else {
        return find_all_muls_part2(first_int_chars, enabled);
    };

    let Some(comma_chars) = first_int_chars.get(comma_idx as usize..) else {
        return find_all_muls_part2(first_int_chars, enabled);
    };

    let Some(second_int_chars) = comma_chars.strip_prefix(",") else {
        return find_all_muls_part2(comma_chars, enabled);
    };

    let Some((second_int, paren_idx)) = parse_digit(second_int_chars) else {
        return find_all_muls_part2(second_int_chars, enabled);
    };

    let Some(close_paren_chars) = second_int_chars.get(paren_idx as usize..) else {
        return find_all_muls_part2(second_int_chars, enabled);
    };

    let Some(next_string) = close_paren_chars.strip_prefix(")") else {
        return find_all_muls_part2(close_paren_chars, enabled);
    };

    println!("{:}, {:}", first_int, second_int);

    Some(first_int * second_int + find_all_muls_part2(next_string, enabled).unwrap_or(0))
}

fn main() {
    let input = fs::read_to_string("resources/day3.txt").expect("File path incorrect.");

    println!("Part 1: {:}", find_all_muls(&input).unwrap());
    println!("Part 2: {:}", find_all_muls_part2(&input, true).unwrap());
}
