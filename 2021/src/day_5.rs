use std::{
    cmp::Ordering,
    collections::HashMap,
    io::{self},
};

use utils::file_utils::read_lines;

pub fn day_5() -> io::Result<i32> {
    find_orthogonal_overlaps("./inputs/day-5-input.txt")
}
pub fn day_5_part_2() -> io::Result<i32> {
    find_overlaps("./inputs/day-5-input.txt")
}

struct Scan {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Scan {
    fn is_horizontal(&self) -> bool {
        self.y1 == self.y2
    }

    fn is_vertical(&self) -> bool {
        self.x1 == self.x2
    }
}

fn load_scans(filename: &str) -> io::Result<Vec<Scan>> {
    let mut scans: Vec<Scan> = Vec::new();

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        let parsed = sscanf::sscanf!(line, "{},{} -> {},{}", i32, i32, i32, i32);

        if let Ok((x1, y1, x2, y2)) = parsed {
            let scan = Scan { x1, y1, x2, y2 };
            scans.push(scan);
        } else {
            panic!("Unparseable input: {line}");
        }
    }
    Ok(scans)
}

fn determine_overlaps(orthogonal_only: bool, scans: &Vec<Scan>) -> i32 {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    for scan in scans {
        let start_coord = (scan.x1, scan.y1);
        let end_coord = (scan.x2, scan.y2);

        if orthogonal_only && (!scan.is_horizontal() && !scan.is_vertical()) {
            continue;
        }

        let offset_x = match scan.x1.cmp(&scan.x2) {
            Ordering::Equal => 0,
            Ordering::Greater => -1,
            Ordering::Less => 1,
        };
        let offset_y = match scan.y1.cmp(&scan.y2) {
            Ordering::Equal => 0,
            Ordering::Greater => -1,
            Ordering::Less => 1,
        };

        let offset = (offset_x, offset_y);
        if offset == (0, 0) {
            continue;
        }

        let mut on_end = false;
        let mut current_coord = start_coord;
        loop {
            let mut previous = 0;
            if let Some(value) = map.get(&current_coord) {
                previous = value.to_owned();
            }

            previous += 1;

            map.insert(current_coord, previous);

            if on_end {
                break;
            }

            current_coord = (current_coord.0 + offset.0, current_coord.1 + offset.1);
            if current_coord == end_coord {
                on_end = true;
            }
        }
    }

    map.iter().filter(|(_, v)| *v >= &2).count() as i32
}

fn find_orthogonal_overlaps(filename: &str) -> io::Result<i32> {
    let scans = load_scans(filename)?;
    Ok(determine_overlaps(true, &scans))
}

fn find_overlaps(filename: &str) -> io::Result<i32> {
    let scans = load_scans(filename)?;
    Ok(determine_overlaps(false, &scans))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            find_orthogonal_overlaps("./inputs/day-5-input-test.txt").unwrap(),
            5
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            find_orthogonal_overlaps("./inputs/day-5-input.txt").unwrap(),
            6311
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(find_overlaps("./inputs/day-5-input-test.txt").unwrap(), 12);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(find_overlaps("./inputs/day-5-input.txt").unwrap(), 19929);
    }
}
