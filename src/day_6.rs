use std::io::{self};

use itertools::Itertools;

use crate::file_utils::read_lines;

pub fn day_6() -> io::Result<i32> {
    fetch_pattern_position(4, "./inputs/day-6-input.txt")
}

pub fn day_6_part_2() -> io::Result<i32> {
    fetch_pattern_position(14, "./inputs/day-6-input.txt")
}

fn fetch_pattern_position(pattern_length: usize, filename: &str) -> io::Result<i32> {
    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        for (n, _) in line
            .chars()
            .into_iter()
            .skip(pattern_length - 1)
            .enumerate()
        {
            let sub = &line[n..(n + pattern_length)];
            if sub.chars().unique().count() == pattern_length {
                return Ok((n + pattern_length) as i32);
            }
        }
    }
    panic!("Pattern not found.");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            fetch_pattern_position(4, "./inputs/day-6-input-test.txt").unwrap(),
            7
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            fetch_pattern_position(14, "./inputs/day-6-input-test.txt").unwrap(),
            19
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            fetch_pattern_position(4, "./inputs/day-6-input.txt").unwrap(),
            1804
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            fetch_pattern_position(14, "./inputs/day-6-input.txt").unwrap(),
            2508
        );
    }
}
