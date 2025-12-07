use std::{
    collections::{HashMap, HashSet},
    io::{self},
};
use utils::file_utils::read_lines;

pub fn day_7() -> io::Result<u32> {
    calc_answers("./inputs/day-7-input.txt")
}

pub fn day_7_part_2() -> io::Result<u64> {
    calc_quantum_splits("./inputs/day-7-input.txt")
}

struct SplitterSetup {
    lines: Vec<HashSet<u32>>,
    start_position: u32,
}

fn calc_answers(filename: &str) -> io::Result<u32> {
    let db = fetch_splitter_positions(filename)?;

    let mut beams = HashSet::new();
    beams.insert(db.start_position);
    let mut splits = 0;
    for l in db.lines {
        let mut new_beams = HashSet::new();
        for b in beams {
            if l.contains(&b) {
                new_beams.insert(b - 1);
                new_beams.insert(b + 1);
                splits += 1;
            } else {
                new_beams.insert(b);
            }
        }
        beams = new_beams;
    }

    Ok(splits as u32)
}

fn calc_quantum_splits(filename: &str) -> io::Result<u64> {
    let db = fetch_splitter_positions(filename)?;

    let mut beams = HashMap::new();
    beams.insert(db.start_position, 1);
    for l in db.lines {
        let mut new_beams = HashMap::new();
        for (b, timelines) in beams {
            if l.contains(&b) {
                new_beams
                    .entry(b - 1)
                    .and_modify(|t| *t += timelines)
                    .or_insert(timelines);
                new_beams
                    .entry(b + 1)
                    .and_modify(|t| *t += timelines)
                    .or_insert(timelines);
                //splits += 1;
            } else {
                new_beams
                    .entry(b)
                    .and_modify(|t| *t += timelines)
                    .or_insert(timelines);
            }
        }
        beams = new_beams;
    }

    Ok(beams.values().sum::<u64>())
}

fn fetch_splitter_positions(filename: &str) -> io::Result<SplitterSetup> {
    let mut result = SplitterSetup {
        lines: Vec::new(),
        start_position: 0,
    };

    let lines = read_lines(filename)?;
    for line in lines.map_while(Result::ok) {
        if line.is_empty() {
            continue;
        }
        let mut splitter_line = HashSet::new();
        for (x, c) in line.chars().enumerate() {
            if c == '^' {
                splitter_line.insert(x as u32);
            }
            if c == 'S' {
                result.start_position = x as u32;
            }
        }
        result.lines.push(splitter_line);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(calc_answers("./inputs/day-7-input-test.txt").unwrap(), 21);
    }

    #[test]
    fn test() {
        assert_eq!(calc_answers("./inputs/day-7-input.txt").unwrap(), 1703);
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            calc_quantum_splits("./inputs/day-7-input-test.txt").unwrap(),
            40
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            calc_quantum_splits("./inputs/day-7-input.txt").unwrap(),
            171692855075500
        );
    }
}
