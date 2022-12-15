use std::{collections::HashSet, io};

use crate::file_utils::read_lines;

struct Sensor {
    x: i32,
    y: i32,
    beacon: Beacon,
}

struct Beacon {
    x: i32,
    y: i32,
}

impl Sensor {
    fn beacon_distance(&self) -> i32 {
        (self.x - self.beacon.x).abs() + (self.y - self.beacon.y).abs()
    }
}

fn read_slice_from_file(filename: &str) -> io::Result<Vec<Sensor>> {
    let lines = read_lines(filename)?;

    let mut sensors = Vec::new();
    for line in lines.flatten() {
        let parsed = sscanf::sscanf!(
            line,
            "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            i32,
            i32,
            i32,
            i32
        );

        if let Ok((sensor_x, sensor_y, beacon_x, beacon_y)) = parsed {
            let sensor = Sensor {
                x: sensor_x,
                y: sensor_y,
                beacon: Beacon {
                    x: beacon_x,
                    y: beacon_y,
                },
            };
            sensors.push(sensor);
        } else {
            panic!("Unparseable input: {line}");
        }
    }
    Ok(sensors)
}

fn impossible_beacons_in_row(row_number: i32, sensors: &Vec<Sensor>) -> (i32, Option<i32>) {
    let mut ranges: Vec<(i32, i32)> = Vec::new();
    let mut beacons: HashSet<i32> = HashSet::new();
    for sensor in sensors {
        // now for the tricky part
        let distance = sensor.beacon_distance();

        if sensor.y - distance >= row_number || sensor.y + distance <= row_number {
            continue;
        }
        if sensor.beacon.y == row_number {
            beacons.insert(sensor.beacon.x);
        }
        // x range
        let x_r = distance - (row_number - sensor.y).abs();
        ranges.push((sensor.x - x_r, sensor.x + x_r + 1));
    }

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    ranges = purge_ranges(ranges);

    let gap = if ranges.len() == 2 {
        Some(ranges[0].1)
    } else {
        None
    };

    let mut non_gaps = 0;
    for (start_range, end_range) in ranges {
        non_gaps += end_range - start_range;
    }

    (non_gaps - (beacons.len() as i32), gap)
}

fn purge_ranges(mut ranges: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut purged = Vec::new();

    if ranges.len() <= 1 {
        return ranges;
    }

    ranges.reverse();
    let mut current = ranges.pop().unwrap();

    loop {
        if let Some(range) = ranges.pop() {
            if current.1 < range.0 {
                purged.push(current);
                current = range;
            } else if current.1 >= range.0 && current.1 <= range.1 {
                current.1 = range.1;
            }
        } else {
            break;
        }
    }
    purged.push(current);
    purged
}

fn find_missing_beacon_frequency(sensors: &Vec<Sensor>, row_count: i32) -> i64 {
    for y in 0..row_count {
        if let (_, Some(x)) = impossible_beacons_in_row(y, &sensors) {
            return (4_000_000_i64 * x as i64) + (y as i64);
        }
    }
    panic!("Unable to find result.");
}

pub fn day_15() -> io::Result<i32> {
    let sensors = read_slice_from_file("./inputs/day-15-input.txt")?;
    let (result, _) = impossible_beacons_in_row(2_000_000, &sensors);
    Ok(result)
}

pub fn day_15_part_2() -> io::Result<i64> {
    let sensors = read_slice_from_file("./inputs/day-15-input.txt")?;
    let result = find_missing_beacon_frequency(&sensors, 4_000_000);
    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn purges_ranges_test() {
        let a = vec![(10, 19), (20, 29), (30, 39)];
        assert_eq!(vec![(10, 19), (20, 29), (30, 39)], purge_ranges(a));

        let b = vec![(10, 25), (12, 29), (30, 39)];
        assert_eq!(vec![(10, 29), (30, 39)], purge_ranges(b));

        let c = vec![(10, 19), (15, 29), (28, 39), (50, 56)];
        assert_eq!(vec![(10, 39), (50, 56)], purge_ranges(c));

        let d = vec![];
        assert_eq!(Vec::new() as Vec<(i32, i32)>, purge_ranges(d));

        let e = vec![(10, 5)];
        assert_eq!(vec![(10, 5)], purge_ranges(e));
    }

    #[test]
    fn small_test() {
        let sensors = read_slice_from_file("./inputs/day-15-input-test.txt").unwrap();
        let (result, _) = impossible_beacons_in_row(10, &sensors);
        assert_eq!(result, 26);
    }

    #[test]
    fn test() {
        let sensors = read_slice_from_file("./inputs/day-15-input.txt").unwrap();
        let (result, _) = impossible_beacons_in_row(2_000_000, &sensors);
        assert_eq!(result, 5367037);
    }

    #[test]
    fn part_2_small_test() {
        let sensors = read_slice_from_file("./inputs/day-15-input-test.txt").unwrap();
        let result = find_missing_beacon_frequency(&sensors, 20);
        assert_eq!(result, 56000011);
    }

    #[test]
    fn part_2_test() {
        let sensors = read_slice_from_file("./inputs/day-15-input.txt").unwrap();
        let result = find_missing_beacon_frequency(&sensors, 4_000_000);
        assert_eq!(result, 11914583249288);
    }
}
