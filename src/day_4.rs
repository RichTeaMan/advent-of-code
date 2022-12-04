use std::io::{self};

use itertools::Itertools;

use crate::file_utils::read_lines;

struct ElfRange {
    pub start: i32,
    pub end: i32,
}

impl ElfRange {
    pub fn from_str(str: &str) -> ElfRange {
        if let Some((a, b)) = str.split('-').collect_tuple() {
            let start = a.parse::<i32>().unwrap();
            let end = b.parse::<i32>().unwrap();

            ElfRange { start, end }
        } else {
            panic!("irregular hyphen");
        }
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

pub fn day_4() -> io::Result<i32> {
    fetch_overlapping_pairs("day-4-input.txt")
}

fn fetch_overlapping_pairs(filename: &str) -> io::Result<i32> {
    let mut overlapping_pairs = 0;

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }
        if let Some((a, b)) = line.split(',').collect_tuple() {
            let range_a = ElfRange::from_str(a);
            let range_b = ElfRange::from_str(b);

            if range_a.contains(&range_b) || range_b.contains(&range_a) {
                overlapping_pairs += 1;
            }
        } else {
            panic!("irregular comma");
        }
    }
    Ok(overlapping_pairs)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(fetch_overlapping_pairs("./day-4-input-test.txt").unwrap(), 2);
    }
    
    #[test]
    fn test() {
        assert_eq!(fetch_overlapping_pairs("./day-4-input.txt").unwrap(), 540);
    }
}
