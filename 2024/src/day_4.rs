use std::io::{self};
use utils::coordinate::{COORDINATE_VECTORS, Coordinate, char_coordinate_map_from_file};

pub fn day_4() -> io::Result<i32> {
    find_xmas("./inputs/day-4-input.txt")
}
pub fn day_4_part_2() -> io::Result<i32> {
    find_cross_mas("./inputs/day-4-input.txt")
}

fn find_xmas(filename: &str) -> io::Result<i32> {
    let (map, _, _) = char_coordinate_map_from_file(filename, false)?;

    let check_str = "MAS";
    let mut found_count = 0;
    for (coord, c) in map.iter() {
        if *c != 'X' {
            continue;
        }

        for coord_vec in COORDINATE_VECTORS {
            let mut current_coord = coord.clone();

            let mut found = true;
            for check_c in check_str.chars() {
                let check_coord = current_coord + Coordinate::from(coord_vec.clone());

                if let Some(v) = map.get(&check_coord) {
                    if *v == check_c {
                        current_coord = check_coord;
                    } else {
                        found = false;
                        break;
                    }
                } else {
                    found = false;
                    break;
                }
            }
            if found {
                found_count += 1;
            }
        }
    }
    Ok(found_count)
}

fn find_cross_mas(filename: &str) -> io::Result<i32> {
    let (map, _, _) = char_coordinate_map_from_file(filename, false)?;

    let mut found_count = 0;
    for (coord, c) in map.iter() {
        if *c != 'A' {
            continue;
        }

        let top_left_coord = coord.clone() + Coordinate::from((-1, -1));
        let top_left_opt = map.get(&top_left_coord);
        let req_bottom_right;

        if top_left_opt == Some(&'M') {
            req_bottom_right = Some(&'S');
        } else if top_left_opt == Some(&'S') {
            req_bottom_right = Some(&'M');
        } else {
            continue;
        }

        let bottom_right_coord = coord.clone() + Coordinate::from((1, 1));
        if map.get(&bottom_right_coord) != req_bottom_right {
            continue;
        }

        let top_right_coord = coord.clone() + Coordinate::from((1, -1));
        let top_right_opt = map.get(&top_right_coord);
        let req_bottom_left;

        if top_right_opt == Some(&'M') {
            req_bottom_left = Some(&'S');
        } else if top_right_opt == Some(&'S') {
            req_bottom_left = Some(&'M');
        } else {
            continue;
        }

        let bottom_left_coord = coord.clone() + Coordinate::from((-1, 1));
        if map.get(&bottom_left_coord) != req_bottom_left {
            continue;
        }

        found_count += 1;
    }
    Ok(found_count)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(find_xmas("./inputs/day-4-input-test.txt").unwrap(), 18);
    }

    #[test]
    fn test() {
        assert_eq!(find_xmas("./inputs/day-4-input.txt").unwrap(), 2599);
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(find_cross_mas("./inputs/day-4-input-test.txt").unwrap(), 9);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(find_cross_mas("./inputs/day-4-input.txt").unwrap(), 1948);
    }
}
