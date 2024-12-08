use std::{
    collections::HashSet,
    io::{self},
};
use utils::coordinate::char_coordinate_map_from_file;

pub fn day_6() -> io::Result<i32> {
    calc_visited("./inputs/day-6-input.txt")
}
pub fn day_6_part_2() -> io::Result<i32> {
    calc_obstruction_positions("./inputs/day-6-input.txt")
}

fn calc_visited(filename: &str) -> io::Result<i32> {
    let (map, width, height) = char_coordinate_map_from_file(filename, true)?;

    let mut position = map.iter().find(|(_, v)| v == &&'^').unwrap().0.clone();

    let right = (1, 0);
    let up = (0, -1);
    let left = (-1, 0);
    let down = (0, 1);

    let mut vector = up;

    let mut visited = HashSet::new();

    while position.x >= 0 && position.x < width as i32 && position.y >= 0 && position.y < height as i32 {
        visited.insert(position.clone());
        let mut new_position = position.clone();
        new_position.add_tuple(vector);
        if let Some(c) = map.get(&new_position) {
            if *c == '#' {
                // rotate

                vector = if vector == up {
                    right
                } else if vector == right {
                    down
                } else if vector == down {
                    left
                } else if vector == left {
                    up
                } else {
                    panic!("invalid rotation vector")
                };

                continue;
            }
        }
        position = new_position;
    }

    Ok(visited.len() as i32)
}

fn calc_obstruction_positions(filename: &str) -> io::Result<i32> {
    let ( mut map, width, height) = char_coordinate_map_from_file(filename, true)?;

    let start_position = map.iter().find(|(_, v)| v == &&'^').unwrap().0.clone();
    let mut position = start_position.clone();

    let right = (1, 0);
    let up = (0, -1);
    let left = (-1, 0);
    let down = (0, 1);

    let mut vector = up;

    let mut visited = HashSet::new();

    while position.x >= 0 && position.x < width as i32 && position.y >= 0 && position.y < height as i32 {
        visited.insert(position);
        let mut new_position = position.clone();
        new_position.add_tuple(vector);

        if let Some(c) = map.get(&new_position) {
            if *c == '#' {
                // rotate

                vector = if vector == up {
                    right
                } else if vector == right {
                    down
                } else if vector == down {
                    left
                } else if vector == left {
                    up
                } else {
                    panic!("invalid rotation vector")
                };

                continue;
            }
        }
        position = new_position;
    }

    let mut loop_count = 0;

    // new place obstructions in all those positions
    for obstruction in visited {
        if obstruction == start_position {
            continue;
        }
        map.insert(obstruction.clone(), '#');
        vector = up;
        position = start_position.clone();

        let mut obs_visited = HashSet::new();

        while position.x >= 0 && position.x < width as i32 && position.y >= 0 && position.y < height as i32 {
            let mut new_position = position.clone();
            new_position.add_tuple(vector);

            if let Some(c) = map.get(&new_position) {
                if *c == '#' {
                    let vector_position = (vector, new_position);
                    if obs_visited.contains(&vector_position) {
                        loop_count += 1;
                        break;
                    }
                    obs_visited.insert(vector_position);
                    
                    // rotate
                    vector = if vector == up {
                        right
                    } else if vector == right {
                        down
                    } else if vector == down {
                        left
                    } else if vector == left {
                        up
                    } else {
                        panic!("invalid rotation vector")
                    };

                    continue;
                }
            }
            position = new_position;
        }

        map.remove(&obstruction);
    }

    Ok(loop_count)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(calc_visited("./inputs/day-6-input-test.txt").unwrap(), 41);
    }

    #[test]
    fn test() {
        assert_eq!(calc_visited("./inputs/day-6-input.txt").unwrap(), 5404);
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            calc_obstruction_positions("./inputs/day-6-input-test.txt").unwrap(),
            6
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            calc_obstruction_positions("./inputs/day-6-input.txt").unwrap(),
            1984
        );
    }
}
