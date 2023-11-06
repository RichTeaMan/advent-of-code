use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{self},
};

use crate::file_utils::read_lines;

pub fn day_9() -> io::Result<i32> {
    find_risk_of_lowpoint("./inputs/day-9-input.txt")
}
pub fn day_9_part_2() -> io::Result<i32> {
    find_basins("./inputs/day-9-input.txt")
}

fn load_map(filename: &str) -> io::Result<HashMap<(i32, i32), i32>> {
    let mut map = HashMap::new();

    let lines = read_lines(filename)?;
    let mut y = 0;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        for (x, c) in line.trim().chars().enumerate() {
            let height = format!("{c}").parse::<i32>().unwrap();
            map.insert((x as i32, y), height);
        }

        y += 1;
    }
    Ok(map)
}

fn find_risk_of_lowpoint(filename: &str) -> io::Result<i32> {
    let map = load_map(filename)?;
    let mut risk = 0;
    'outer: for ((x, y), height) in &map {
        let adjacent: Vec<(i32, i32)> = vec![(x - 1, *y), (x + 1, *y), (*x, y - 1), (*x, y + 1)];

        for (adj_x, adj_y) in adjacent {
            if let Some(neighbour_height) = map.get(&(adj_x, adj_y)) {
                if neighbour_height <= height {
                    continue 'outer;
                }
            }
        }
        risk += height + 1;
    }
    Ok(risk)
}

fn find_basins(filename: &str) -> io::Result<i32> {
    let map = load_map(filename)?;
    let mut lowpoints = Vec::new();
    'outer: for ((x, y), height) in &map {
        let adjacent: Vec<(i32, i32)> = vec![(x - 1, *y), (x + 1, *y), (*x, y - 1), (*x, y + 1)];

        for (adj_x, adj_y) in adjacent {
            if let Some(neighbour_height) = map.get(&(adj_x, adj_y)) {
                if neighbour_height <= height {
                    continue 'outer;
                }
            }
        }
        lowpoints.push((x.to_owned(), y.to_owned()));
    }

    let mut basin_sizes = Vec::new();
    basin_sizes.push(1);
    basin_sizes.push(1);
    basin_sizes.push(1);

    let mut visited = HashSet::new();
    for lowpoint in lowpoints {
        if visited.contains(&lowpoint) {
            continue;
        }

        let mut queue = VecDeque::new();
        queue.push_back(lowpoint);
        let mut basin_size = 0;

        while let Some((x, y)) = queue.pop_front() {
            if visited.contains(&(x, y)) {
                continue;
            }
            basin_size += 1;
            visited.insert((x, y));

            let adjacent: Vec<(i32, i32)> = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
            for (adj_x, adj_y) in adjacent {
                if !visited.contains(&(adj_x, adj_y)) {
                    if let Some(adj_h) = map.get(&(adj_x, adj_y)) {
                        if *adj_h < 9 {
                            let new_coord = (adj_x, adj_y);
                            queue.push_back(new_coord);
                        }
                    }
                }
            }
        }
        basin_sizes.push(basin_size);
    }
    basin_sizes.sort();
    Ok(basin_sizes.iter().rev().take(3).product())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            find_risk_of_lowpoint("./inputs/day-9-input-test.txt").unwrap(),
            15
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            find_risk_of_lowpoint("./inputs/day-9-input.txt").unwrap(),
            524
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(find_basins("./inputs/day-9-input-test.txt").unwrap(), 1134);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(find_basins("./inputs/day-9-input.txt").unwrap(), 1235430);
    }
}
