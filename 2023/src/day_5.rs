use itertools::Itertools;
use std::{
    collections::VecDeque,
    io::{self},
};
use utils::file_utils::read_lines;

#[derive(Debug)]
struct AlmanacRange {
    start: i64,
    length: i64,
}

#[derive(Default)]
struct AlmanacMap {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

struct Almanac {
    seeds: Vec<i64>,
    seed_to_soil_map: Vec<AlmanacMap>,
    soil_to_fertilizer_map: Vec<AlmanacMap>,
    fertilizer_to_water_map: Vec<AlmanacMap>,
    water_to_light_map: Vec<AlmanacMap>,
    light_to_temperature_map: Vec<AlmanacMap>,
    temperature_to_humidity_map: Vec<AlmanacMap>,
    humidity_to_location_map: Vec<AlmanacMap>,
}

pub fn day_5() -> io::Result<i64> {
    fetch_lowest_location("./inputs/day-5-input.txt")
}
pub fn day_5_part_2() -> io::Result<i64> {
    fetch_seed_range_lowest_location("./inputs/day-5-input.txt")
}

fn fetch_almanac(filename: &str) -> io::Result<Almanac> {
    let mut almananc_maps = VecDeque::new();
    for _ in 0..7 {
        almananc_maps.push_back(Vec::new());
    }
    let mut map_index = -1_i32;
    let mut seeds = None;

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        if line.contains("seeds:") {
            seeds = Some(
                line.split(' ')
                    .skip(1)
                    .map(|p| p.trim())
                    .filter(|p| !p.is_empty())
                    .map(|p| p.parse::<i64>().unwrap())
                    .collect_vec(),
            );
            continue;
        }

        if line.contains("map") {
            map_index += 1;
            continue;
        }

        let (destination, source, range) = line
            .split(' ')
            .map(|p| p.trim())
            .filter(|p| !p.is_empty())
            .map(|p| p.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();

        let maps = almananc_maps.get_mut(map_index as usize).unwrap();
        maps.push(AlmanacMap {
            destination_range_start: destination,
            source_range_start: source,
            range_length: range,
        });
    }
    let almanac = Almanac {
        seeds: seeds.unwrap(),
        seed_to_soil_map: almananc_maps.pop_front().unwrap(),
        soil_to_fertilizer_map: almananc_maps.pop_front().unwrap(),
        fertilizer_to_water_map: almananc_maps.pop_front().unwrap(),
        water_to_light_map: almananc_maps.pop_front().unwrap(),
        light_to_temperature_map: almananc_maps.pop_front().unwrap(),
        temperature_to_humidity_map: almananc_maps.pop_front().unwrap(),
        humidity_to_location_map: almananc_maps.pop_front().unwrap(),
    };
    Ok(almanac)
}

fn resolve_map_value(maps: &Vec<AlmanacMap>, value: i64) -> i64 {
    for map in maps {
        if value >= map.source_range_start && value < map.source_range_start + map.range_length {
            return (value - map.source_range_start) + map.destination_range_start;
        }
    }

    value
}

fn resolve_map_ranges(maps: &Vec<AlmanacMap>, ranges: Vec<AlmanacRange>) -> Vec<AlmanacRange> {
    let mut ranges_to_process = VecDeque::new();
    for r in ranges {
        debug_assert!(r.start >= 0);
        debug_assert!(r.length > 0);
        ranges_to_process.push_back(r);
    }

    let mut result_ranges = Vec::new();

    while let Some(range) = ranges_to_process.pop_front() {
        let mut map_found = false;
        for map in maps {
            if range.start >= map.source_range_start
                && range.start < map.source_range_start + map.range_length
            {
                let dest_range_start =
                    (range.start - map.source_range_start) + map.destination_range_start;

                // does range fit entirely in map?
                if range.length <= map.range_length {
                    result_ranges.push(AlmanacRange {
                        start: dest_range_start,
                        length: range.length,
                    });
                } else {
                    let r1_start = dest_range_start;
                    let r1_length = (map.source_range_start + map.range_length) - range.start;
                    let r2_start = map.source_range_start + map.range_length;
                    let r2_length = range.length - r1_length;

                    result_ranges.push(AlmanacRange {
                        start: r1_start,
                        length: r1_length,
                    });
                    ranges_to_process.push_back(AlmanacRange {
                        start: r2_start,
                        length: r2_length,
                    });
                }
                map_found = true;
                break;
            } else if range.start + range.length >= map.source_range_start
                && range.start + range.length < map.source_range_start + map.range_length
            {

                let r1_start = range.start;
                let r1_length = (map.source_range_start - range.start) - 1;                
                let r2_start = map.destination_range_start;
                let r2_length = range.length - r1_length;
                if range.start == r1_start && range.length == r1_length {
                    panic!("Duped range: {:?}", range);
                }

                ranges_to_process.push_back(AlmanacRange {
                    start: r1_start,
                    length: r1_length,
                });
                result_ranges.push(AlmanacRange {
                    start: r2_start,
                    length: r2_length,
                });

                map_found = true;
                break;
            }
        }
        if !map_found {
            result_ranges.push(range);
        }
    }

    result_ranges
}

fn fetch_lowest_location(filename: &str) -> io::Result<i64> {
    let almanac = fetch_almanac(filename)?;

    let mut locations = Vec::new();

    for seed in &almanac.seeds {
        let soil = resolve_map_value(&almanac.seed_to_soil_map, *seed);
        let fertilizer = resolve_map_value(&almanac.soil_to_fertilizer_map, soil);
        let water = resolve_map_value(&almanac.fertilizer_to_water_map, fertilizer);
        let light = resolve_map_value(&almanac.water_to_light_map, water);
        let temperature = resolve_map_value(&almanac.light_to_temperature_map, light);
        let humidity = resolve_map_value(&almanac.temperature_to_humidity_map, temperature);
        let location = resolve_map_value(&almanac.humidity_to_location_map, humidity);
        locations.push(location)
    }
    locations.sort();
    Ok(locations.first().unwrap().to_owned())
}

fn fetch_seed_range_lowest_location(filename: &str) -> io::Result<i64> {
    let almanac = fetch_almanac(filename)?;

    let mut seed_ranges = Vec::new();
    for chunk in almanac.seeds.chunks(2) {
        seed_ranges.push(AlmanacRange {
            start: chunk.first().unwrap().to_owned(),
            length: chunk.get(1).unwrap().to_owned(),
        });
    }

    let mut locations = Vec::new();

    let soil = resolve_map_ranges(&almanac.seed_to_soil_map, seed_ranges);
    let fertilizer = resolve_map_ranges(&almanac.soil_to_fertilizer_map, soil);
    let water = resolve_map_ranges(&almanac.fertilizer_to_water_map, fertilizer);
    let light = resolve_map_ranges(&almanac.water_to_light_map, water);
    let temperature = resolve_map_ranges(&almanac.light_to_temperature_map, light);
    let humidity = resolve_map_ranges(&almanac.temperature_to_humidity_map, temperature);
    let location_ranges = resolve_map_ranges(&almanac.humidity_to_location_map, humidity);
    for location_range in location_ranges {
        locations.push(location_range.start);
        //for location in location_range.start..(location_range.start + location_range.length) {
        //    locations.push(location)
        //}
    }

    locations.sort();
    //println!("debug: {:?}", locations);
    Ok(locations.first().unwrap().to_owned())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            fetch_lowest_location("./inputs/day-5-input-test.txt").unwrap(),
            35
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            fetch_lowest_location("./inputs/day-5-input.txt").unwrap(),
            227653707
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            fetch_seed_range_lowest_location("./inputs/day-5-input-test.txt").unwrap(),
            46
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            fetch_seed_range_lowest_location("./inputs/day-5-input.txt").unwrap(),
            78775051
        );
    }
}
