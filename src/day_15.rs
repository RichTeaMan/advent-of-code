use std::{
    collections::{HashMap, HashSet},
    io,
};

use itertools::Itertools;

use crate::file_utils::read_lines;

type Map = HashMap<i32, HashMap<i32, char>>;

const SENSOR_SYMBOL: char = 'S';
const BEACON_SYMBOL: char = 'B';
const SENSOR_COVERAGE_SYMBOL: char = '#';

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

trait MapExt {
    fn fetch(&self, x: i32, y: i32) -> Option<char>;

    fn create_location(&mut self, x: i32, y: i32, c: char, overwrite: bool);

    //fn create_beacon(&mut self, x: i32, y:i32, c:char);

    //fn create_sensor(&mut self, x: i32, y:i32, c:char);

    fn y_limits(&self) -> (i32, i32);

    fn x_limits(&self) -> (i32, i32);

    fn create_from_sensors(sensors: Vec<Sensor>) -> Self;

    fn draw_map(&self);
}

impl MapExt for Map {
    fn fetch(&self, x: i32, y: i32) -> Option<char> {
        if let Some(row) = self.get(&y) {
            return row.get(&x).copied();
        }
        None
    }

    fn create_location(&mut self, x: i32, y: i32, c: char, overwrite: bool) {
        if !self.contains_key(&y) {
            self.insert(y, HashMap::new());
        }

        if let Some(row) = self.get_mut(&y) {
            if overwrite || !row.contains_key(&x) {
                row.insert(x, c);
            }
        } else {
            panic!("Lost a row.");
        }
    }

    fn y_limits(&self) -> (i32, i32) {
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;

        for key in self.keys() {
            if key < &min_y {
                min_y = *key;
            } else if key > &max_y {
                max_y = *key;
            }
        }
        (min_y, max_y)
    }

    fn x_limits(&self) -> (i32, i32) {
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;

        for row in self.values() {
            for key in row.keys() {
                if key < &min_x {
                    min_x = *key;
                } else if key > &max_x {
                    max_x = *key;
                }
            }
        }
        (min_x, max_x)
    }

    fn create_from_sensors(sensors: Vec<Sensor>) -> Self {
        let mut map = Map::new();

        for sensor in &sensors {
            map.create_location(sensor.x, sensor.y, SENSOR_SYMBOL, true);
            map.create_location(sensor.beacon.x, sensor.beacon.y, BEACON_SYMBOL, true);

            // now for the tricky part
            let distance = sensor.beacon_distance();

            let draw_filler = true; //sensor.x == 8 && sensor.y == 7;

            for length in 0..=distance {
                for i in 0..=length {
                    let d_x = i;
                    let d_y = length - i;

                    if draw_filler {
                        map.create_location(sensor.x + d_x, sensor.y + d_y, '#', false);
                        map.create_location(sensor.x + d_x, sensor.y - d_y, '#', false);
                        map.create_location(sensor.x - d_x, sensor.y + d_y, '#', false);
                        map.create_location(sensor.x - d_x, sensor.y - d_y, '#', false);
                    }
                }
            }
        }
        map
    }

    #[allow(dead_code)]
    fn draw_map(&self) {
        let (min_y, max_y) = self.y_limits();
        let (min_x, max_x) = self.x_limits();

        println!("({min_x},{min_y}) -> ({max_x},{max_y})");
        for y in min_y..=max_y {
            print!("{y:0>3} ");
            for x in min_x..=max_x {
                if let Some(c) = self.fetch(x, y) {
                    print!("{c}");
                } else {
                    print!(".");
                }
            }
            println!();
        }
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

fn impossible_beacons_in_row(row_number: i32, sensors: Vec<Sensor>) -> i32 {
    let mut row: HashSet<i32> = HashSet::new();
    let mut beacons: HashSet<i32> = HashSet::new();
    for sensor in &sensors {
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

        for x in 0..=x_r {
            row.insert(sensor.x - x);
            row.insert(sensor.x + x);
        }
    }
    (row.len() - beacons.len()) as i32
}

pub fn day_15() -> io::Result<i32> {
    let sensors = read_slice_from_file("./inputs/day-15-input.txt")?;
    Ok(impossible_beacons_in_row(2_000_000, sensors))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        let sensors = read_slice_from_file("./inputs/day-15-input-test.txt").unwrap();
        let result = impossible_beacons_in_row(10, sensors);
        assert_eq!(result, 26);
    }

    #[test]
    fn test() {
        let sensors = read_slice_from_file("./inputs/day-15-input.txt").unwrap();
        let result = impossible_beacons_in_row(2_000_000, sensors);
        assert_eq!(result, 5367037);
    }
}
