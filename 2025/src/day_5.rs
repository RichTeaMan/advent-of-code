use std::{
    collections::HashSet,
    io::{self},
};
use utils::file_utils::read_lines;

pub fn day_5() -> io::Result<u64> {
    find_fresh("./inputs/day-5-input.txt")
}

pub fn day_5_part_2() -> io::Result<u64> {
    find_all_fresh("./inputs/day-5-input.txt")
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct IdRange {
    start: u64,
    end: u64,
}

struct FoodDb {
    fresh_ranges: Vec<IdRange>,
    ids: Vec<u64>,
}

fn find_fresh(filename: &str) -> io::Result<u64> {
    let db = fetch_db(filename)?;
    let mut fresh_count = 0;

    for id in db.ids {
        let mut found = false;
        for range in &db.fresh_ranges {
            if id >= range.start && id <= range.end {
                found = true;
                break;
            }
        }
        if found {
            fresh_count += 1;
        }
    }

    Ok(fresh_count)
}

fn find_all_fresh(filename: &str) -> io::Result<u64> {
    let db = fetch_db(filename)?;

    let mut ranges: HashSet<IdRange> = HashSet::from_iter(db.fresh_ranges);

    loop {
        let mut new_ranges = HashSet::new();

        for range in &ranges {
            let mut starts = vec![range.start];
            let mut ends = vec![range.end];
            for other_range in &ranges {
                if range.start == other_range.start && range.end == other_range.end {
                    continue;
                }

                if range.start >= other_range.start && range.start <= other_range.end {
                    starts.push(other_range.start);
                }
                if range.end >= other_range.start && range.end <= other_range.end {
                    ends.push(other_range.end);
                }
            }
            new_ranges.insert(IdRange {
                start: *starts.iter().min().unwrap(),
                end: *ends.iter().max().unwrap(),
            });
        }
        if ranges.eq(&new_ranges) {
            break;
        }
        ranges = new_ranges;
    }

    let fresh_count = ranges.iter().map(|r| (r.end - r.start) + 1).sum();

    Ok(fresh_count)
}
fn fetch_db(filename: &str) -> io::Result<FoodDb> {
    let mut result = FoodDb {
        fresh_ranges: Vec::new(),
        ids: Vec::new(),
    };

    let lines = read_lines(filename)?;
    for line in lines.map_while(Result::ok) {
        if line.is_empty() {
            continue;
        }

        if let Some((a, b)) = line.split_once("-") {
            result.fresh_ranges.push(IdRange {
                start: a.parse().unwrap(),
                end: b.parse().unwrap(),
            });
        } else {
            result.ids.push(line.parse().unwrap());
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(find_fresh("./inputs/day-5-input-test.txt").unwrap(), 3);
    }

    #[test]
    fn test() {
        assert_eq!(find_fresh("./inputs/day-5-input.txt").unwrap(), 640);
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(find_all_fresh("./inputs/day-5-input-test.txt").unwrap(), 14);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            find_all_fresh("./inputs/day-5-input.txt").unwrap(),
            365804144481581
        );
    }
}
