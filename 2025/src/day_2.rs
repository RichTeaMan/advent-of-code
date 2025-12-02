use std::{
    collections::HashMap,
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

            if v_str.len() % 2 != 0 {
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

    let max_length = ranges.iter().map(|r| r.1.to_string().len()).max().unwrap();

    let mut factor_lookup = HashMap::new();
    for l in 2..=max_length {
        let mut factors = Vec::new();

        for i in 1..l {
            if l % i == 0 {
                factors.push(i);
            }
        }
        factor_lookup.insert(l, factors);
    }

    let mut invalid_id_sum: u64 = 0;

    for (a, b) in ranges {
        for v in a..=b {
            let v_str = v.to_string();

            // ehh?
            if v_str.len() == 1 {
                continue;
            }
            for factor in factor_lookup.get(&v_str.len()).unwrap() {
                let mut chunk = v_str.clone();
                let mut last_chunk_opt = None;
                let mut unequal_found = false;

                while !chunk.is_empty() && !unequal_found {
                    let c = chunk.clone();
                    let (a, b) = c.split_at(*factor);
                    if let Some(last_chunk) = last_chunk_opt {
                        if last_chunk == a {
                            last_chunk_opt = Some(a.to_string());
                        } else {
                            // not equal found, abort
                            unequal_found = true;
                            break;
                        }
                    } else {
                        last_chunk_opt = Some(a.to_string());
                    }
                    chunk = b.to_string();
                    debug_assert!(
                        chunk.is_empty() || chunk.len() % factor == 0 || chunk.len() < c.len()
                    );
                }

                if !unequal_found {
                    invalid_id_sum += v_str.parse::<u64>().unwrap();
                    break;
                }
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
