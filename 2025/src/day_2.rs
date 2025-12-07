use std::{
    io::{self},
};
use utils::file_utils::read_lines;

pub fn day_2() -> io::Result<u64> {
    solve_invalid_ids("./inputs/day-2-input.txt")
}
pub fn day_2_part_2() -> io::Result<u64> {
    solve_deep_invalid_ids("./inputs/day-2-input.txt")
}

fn solve_invalid_ids(filename: &str) -> io::Result<u64> {
    let ranges = fetch_range(filename)?;

    let mut invalid_id_sum: u64 = 0;

    for (a, b) in ranges {
        for v in a..=b {
            let v_str = v.to_string();

            if !v_str.len().is_multiple_of(2) {
                continue;
            }

            let (v1_str, v2_str) = v_str.split_at(v_str.len() / 2);
            debug_assert_eq!(v1_str.len(), v2_str.len());

            if v1_str == v2_str {
                invalid_id_sum += v_str.parse::<u64>().unwrap();
            }
        }
    }

    Ok(invalid_id_sum)
}

fn solve_deep_invalid_ids(filename: &str) -> io::Result<u64> {
    let ranges = fetch_range(filename)?;

    let mut invalid_id_sum: u64 = 0;

    // shamelessly stolen from https://github.com/MizardX/AdventOfCode_2025/blob/main/src/day_02.rs
    // my previous solution split the number string. functional, but took around 450ms. this is much faster
    for (a, b) in ranges {
        for v in a..=b {
            let invalid = match v {
                10..=99 => v.is_multiple_of(11),
                100..=999 => v.is_multiple_of(111),
                1000..=9999 => v.is_multiple_of(101) || v.is_multiple_of(1111),
                10000..=99999 => v.is_multiple_of(11111),
                100000..=999999 => v.is_multiple_of(1001) || v.is_multiple_of(10101) || v.is_multiple_of(111111),
                1000000..=9999999 => v.is_multiple_of(1111111),
                10000000..=99999999 => v.is_multiple_of(10001) || v.is_multiple_of(1010101) || v.is_multiple_of(11111111),
                100000000..=999999999 => v.is_multiple_of(1001001) || v.is_multiple_of(111111111),
                1000000000..=9999999999 => {
                    v.is_multiple_of(100001) || v.is_multiple_of(101010101) || v.is_multiple_of(1111111111)
                }
                _ => false,
            };

            if invalid {
                invalid_id_sum += v;
            }
        }
    }

    Ok(invalid_id_sum)
}

fn fetch_range(filename: &str) -> io::Result<Vec<(u64, u64)>> {
    let mut result = Vec::new();

    let lines = read_lines(filename)?;
    for line in lines.map_while(Result::ok) {
        if line.is_empty() {
            continue;
        }
        for range in line.split(",") {
            if range.is_empty() {
                continue;
            }
            let (a, b) = range.split_once("-").unwrap();
            result.push((a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()));
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
            solve_invalid_ids("./inputs/day-2-input-test.txt").unwrap(),
            1227775554
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            solve_invalid_ids("./inputs/day-2-input.txt").unwrap(),
            30323879646
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            solve_deep_invalid_ids("./inputs/day-2-input-test.txt").unwrap(),
            4174379265
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            solve_deep_invalid_ids("./inputs/day-2-input.txt").unwrap(),
            43872163557
        );
    }
}
