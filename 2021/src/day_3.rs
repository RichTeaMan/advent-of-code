use std::io::{self};

use crate::file_utils::read_lines;

pub fn day_3() -> io::Result<i32> {
    calc_power_consumption("./inputs/day-3-input.txt")
}
pub fn day_3_part_2() -> io::Result<i32> {
    calc_gas("./inputs/day-3-input.txt")
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

fn calc_gas(filename: &str) -> io::Result<i32> {
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

    let mut oxygen_filter = values.clone();
    let mut co2_filter = values.clone();
    for p in (0..line_length).rev() {
        let mask = 2_i32.pow(p);

        if oxygen_filter.len() > 1 {
            let mut target = 0;
            let mut one_count = 0;
            let mut zero_count = 0;
            for value in &oxygen_filter {
                let bit = value & mask;
                if bit > 0 {
                    one_count += 1;
                } else {
                    zero_count += 1;
                }
            }
            if one_count >= zero_count {
                target += mask;
            }

            let mut new_oxy = Vec::new();
            for value in oxygen_filter {
                if value & mask == target {
                    new_oxy.push(value);
                }
            }
            oxygen_filter = new_oxy;
        }

        if co2_filter.len() > 1 {
            let mut target = 0;
            let mut one_count = 0;
            let mut zero_count = 0;
            for value in &co2_filter {
                let bit = value & mask;
                if bit > 0 {
                    one_count += 1;
                } else {
                    zero_count += 1;
                }
            }
            if one_count < zero_count {
                target += mask;
            }

            let mut new_co2 = Vec::new();
            for value in co2_filter {
                if value & mask == target {
                    new_co2.push(value);
                }
            }
            co2_filter = new_co2;
        }
    }

    debug_assert_eq!(oxygen_filter.len(), 1);
    debug_assert_eq!(co2_filter.len(), 1);

    Ok(oxygen_filter[0] * co2_filter[0])
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

    #[test]
    fn part_2_small_test() {
        assert_eq!(calc_gas("./inputs/day-3-input-test.txt").unwrap(), 230);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(calc_gas("./inputs/day-3-input.txt").unwrap(), 3385170);
    }
}
