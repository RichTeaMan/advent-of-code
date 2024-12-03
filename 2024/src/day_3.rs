use regex::Regex;
use std::io::{self};
use utils::file_utils::read_lines;

pub fn day_3() -> io::Result<i32> {
    calc_mul_instructions("./inputs/day-3-input.txt")
}
pub fn day_3_part_2() -> io::Result<i32> {
    calc_do_mul_instructions("./inputs/day-3-input.txt")
}

fn calc_mul_instructions(filename: &str) -> io::Result<i32> {
    let lines = read_lines(filename)?;

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut result = 0;
    for line in lines.flatten() {
        for (_, [arg_a_str, arg_b_str]) in re.captures_iter(&line).map(|c| c.extract()) {
            let arg_a = arg_a_str.parse::<i32>().unwrap();
            let arg_b = arg_b_str.parse::<i32>().unwrap();
            result += arg_a * arg_b;
        }
    }
    Ok(result)
}

fn calc_do_mul_instructions(filename: &str) -> io::Result<i32> {
    let lines = read_lines(filename)?;
    let mult_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut mult_enabled = true;

    let mut result = 0;
    for line in lines.flatten() {
        for segment in line.split(')') {
            if let Some((_, [arg_a_str, arg_b_str])) = mult_re
                .captures(format!("{})", segment).as_str())
                .map(|c| c.extract())
            {
                if !mult_enabled {
                    continue;
                }
                let arg_a = arg_a_str.parse::<i32>().unwrap();
                let arg_b = arg_b_str.parse::<i32>().unwrap();
                result += arg_a * arg_b;
            } else if segment.contains("do(") {
                mult_enabled = true;
            } else if segment.contains("don't(") {
                mult_enabled = false;
            }
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            calc_mul_instructions("./inputs/day-3-input-test.txt").unwrap(),
            161
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            calc_mul_instructions("./inputs/day-3-input.txt").unwrap(),
            178538786
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            calc_do_mul_instructions("./inputs/day-3-input-test-2.txt").unwrap(),
            48
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            calc_do_mul_instructions("./inputs/day-3-input.txt").unwrap(),
            102467299
        );
    }
}
