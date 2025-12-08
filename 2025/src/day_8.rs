use itertools::Itertools;
use std::{
    collections::HashSet,
    io::{self},
};
use utils::file_utils::read_lines;

pub fn day_8() -> io::Result<u64> {
    calc_connections(1000, "./inputs/day-8-input.txt")
}

pub fn day_8_part_2() -> io::Result<u64> {
    calc_all_connections("./inputs/day-8-input.txt")
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct JunctionPosition {
    x: i64,
    y: i64,
    z: i64,
}

fn dist_squared(a: &JunctionPosition, b: &JunctionPosition) -> u64 {
    (a.x - b.x).abs().pow(2) as u64
        + (a.y - b.y).abs().pow(2) as u64
        + (a.z - b.z).abs().pow(2) as u64
}

fn calc_connections(connections: u32, filename: &str) -> io::Result<u64> {
    let db = fetch_positions(filename)?;

    let mut dist_sqr = Vec::new();

    for l in db.iter().combinations(2) {
        let (a, b) = l.iter().collect_tuple().unwrap();
        if a == b {
            continue;
        }
        // the reverse will be stored as well. im choosing to be ok with it
        let d = dist_squared(a, b);
        dist_sqr.push((**a, **b, d));
    }
    dist_sqr.sort_by(|a, b| b.2.cmp(&a.2));

    let mut circuits: Vec<HashSet<JunctionPosition>> = Vec::new();

    for _ in 0..connections {
        let (a, b, _) = dist_sqr.pop().unwrap();

        // check if either junction is already in circuit
        let mut found_indexes = Vec::new();
        for (i, c) in circuits.iter().enumerate() {
            if c.contains(&a) || c.contains(&b) {
                found_indexes.push(i);
            }
        }

        if found_indexes.is_empty() {
            let mut new_circuit: HashSet<JunctionPosition> = HashSet::new();
            new_circuit.insert(a);
            new_circuit.insert(b);
            circuits.push(new_circuit);
        } else if found_indexes.len() == 1 {
            let new_circuit = circuits.get_mut(*found_indexes.first().unwrap()).unwrap();
            new_circuit.insert(a);
            new_circuit.insert(b);
        }
        // two circuits new to be unified
        else if found_indexes.len() == 2 {
            let i_c1 = found_indexes.first().unwrap(); // this index should always be smaller
            let i_c2 = found_indexes.get(1).unwrap();

            let c2 = circuits.remove(*i_c2);
            let c1 = circuits.get_mut(*i_c1).unwrap();

            for j in c2.into_iter() {
                c1.insert(j);
            }
        } else {
            panic!("Illegal number of indicies {}", found_indexes.len());
        }
    }

    // multiply size of 3 largest circuits
    let result = circuits
        .iter()
        .map(|c| c.len() as u64)
        .sorted_by_key(|&w| std::cmp::Reverse(w))
        .take(3)
        .product();
    Ok(result)
}

fn calc_all_connections(filename: &str) -> io::Result<u64> {
    let db = fetch_positions(filename)?;

    let mut dist_sqr = Vec::new();

    for l in db.iter().combinations(2) {
        let (a, b) = l.iter().collect_tuple().unwrap();
        if a == b {
            continue;
        }
        // the reverse will be stored as well. im choosing to be ok with it
        let d = dist_squared(a, b);
        dist_sqr.push((**a, **b, d));
    }
    dist_sqr.sort_by(|a, b| b.2.cmp(&a.2));

    let mut circuits: Vec<HashSet<JunctionPosition>> = Vec::new();

    let mut latest_x_product = 0_u64;
    while circuits.first().map_or(0, |v| v.len()) < db.len() {
        let (a, b, _) = dist_sqr.pop().unwrap();

        // check if either junction is already in circuit
        let mut found_indexes = Vec::new();
        for (i, c) in circuits.iter().enumerate() {
            if c.contains(&a) || c.contains(&b) {
                found_indexes.push(i);
            }
        }

        if found_indexes.is_empty() {
            let mut new_circuit: HashSet<JunctionPosition> = HashSet::new();
            new_circuit.insert(a);
            new_circuit.insert(b);
            circuits.push(new_circuit);
        } else if found_indexes.len() == 1 {
            let new_circuit = circuits.get_mut(*found_indexes.first().unwrap()).unwrap();
            new_circuit.insert(a);
            new_circuit.insert(b);
        }
        // two circuits new to be unified
        else if found_indexes.len() == 2 {
            let i_c1 = found_indexes.first().unwrap(); // this index should always be smaller
            let i_c2 = found_indexes.get(1).unwrap();

            let c2 = circuits.remove(*i_c2);
            let c1 = circuits.get_mut(*i_c1).unwrap();

            for j in c2.into_iter() {
                c1.insert(j);
            }
        } else {
            panic!("Illegal number of indicies {}", found_indexes.len());
        }

        latest_x_product = (a.x * b.x) as u64;
    }

    debug_assert_eq!(circuits.len(), 1);

    Ok(latest_x_product)
}

fn fetch_positions(filename: &str) -> io::Result<Vec<JunctionPosition>> {
    let mut result = Vec::new();

    let lines = read_lines(filename)?;
    for line in lines.map_while(Result::ok) {
        if line.is_empty() {
            continue;
        }
        let (x, y, z) = line
            .split(',')
            .map(|p| p.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        let junction_position = JunctionPosition { x, y, z };
        result.push(junction_position);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            calc_connections(10, "./inputs/day-8-input-test.txt").unwrap(),
            40
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            calc_connections(1000, "./inputs/day-8-input.txt").unwrap(),
            123420
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            calc_all_connections("./inputs/day-8-input-test.txt").unwrap(),
            25272
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            calc_all_connections("./inputs/day-8-input.txt").unwrap(),
            673096646
        );
    }
}
