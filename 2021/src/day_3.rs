use std::io::{self};

use crate::file_utils::read_lines;

pub fn day_3() -> io::Result<i32> {
    calc_power_consumption("./inputs/day-3-input.txt")
}
pub fn day_3_part_2() -> io::Result<i32> {
    panic!();
    //calc_position_with_aim("./inputs/day-3-input.txt")
}

fn calc_power_consumption(filename: &str) -> io::Result<i32> {
    let mut line_length: u32 = 0;

    let mut values = Vec::new();
    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        let mut value: i32 = 0;
        line_length = line.len().try_into().unwrap();
        for (a, b) in line.chars().enumerate() {
            if b == '1' {
                let exp = ((line.len() - 1) - a).try_into().unwrap();
                debug_assert!(exp < line.len().try_into().unwrap());
                value += 2_i32.pow(exp);
            }
        }
        values.push(value);
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for p in (0..line_length).rev() {
        let mut g_count = 0;
        let mut e_count = 0;
        let mask = 2_i32.pow(p);

        for value in &values {
            let bit = value & mask;
            if bit > 0 {
                g_count += 1;
            } else {
                e_count += 1;
            }
        }
        if g_count > e_count {
            gamma += mask;
        } else {
            epsilon += mask;
        }
    }

    Ok(gamma * epsilon)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            calc_power_consumption("./inputs/day-3-input-test.txt").unwrap(),
            198
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            calc_power_consumption("./inputs/day-3-input.txt").unwrap(),
            3882564
        );
    }

    //#[test]
    //fn part_2_small_test() {
    //    assert_eq!(
    //        calc_position_with_aim("./inputs/day-3-input-test.txt").unwrap(),
    //        900
    //    );
    //}
    //
    //#[test]
    //fn part_2_test() {
    //    assert_eq!(
    //        calc_position_with_aim("./inputs/day-3-input.txt").unwrap(),
    //        2101031224
    //    );
    //}
}
