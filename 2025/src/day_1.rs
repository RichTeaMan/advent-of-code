use std::io::{self};
use utils::file_utils::read_lines;

#[derive(PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

struct Rotation {
    direction: Direction,
    distance: i16,
}

pub fn day_1() -> io::Result<u16> {
    Ok(solve_rotations("./inputs/day-1-input.txt")?.0)
}
pub fn day_1_part_2() -> io::Result<u16> {
    Ok(solve_rotations("./inputs/day-1-input.txt")?.1)
}
fn solve_rotations(filename: &str) -> io::Result<(u16, u16)> {
    let rotations = fetch_rotations(filename)?;

    let mut position = 50;
    let max = 99;

    let mut zeros = 0;
    let mut passing_zeros = 0;

    for rotation in rotations {
        // doing it the crap way
        let step: i16 = if rotation.direction == Direction::Left {
            -1
        } else {
            1
        };
        for _ in 0..rotation.distance {
            position += step;

            if position == max + 1 {
                position = 0;
            }
            if position == -1 {
                position = max;
            }
            if position == 0 {
                passing_zeros += 1;
            }
        }
        if position == 0 {
            zeros += 1;
        }
    }

    Ok((zeros as u16, passing_zeros as u16))
}

fn fetch_rotations(filename: &str) -> io::Result<Vec<Rotation>> {
    let mut result = Vec::new();

    let lines = read_lines(filename)?;
    for line in lines.map_while(Result::ok) {
        if line.is_empty() {
            continue;
        }
        let parsed = sscanf::sscanf!(line, "{}{}", String, i16,);

        if let Ok((direction, distance)) = parsed {
            let direction_enum = match direction.as_str() {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Bad direction"),
            };
            let rotation = Rotation {
                direction: direction_enum,
                distance,
            };
            result.push(rotation);
        } else {
            panic!("Unparseable input: {line}");
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
            solve_rotations("./inputs/day-1-input-test.txt").unwrap().0,
            3
        );
    }

    #[test]
    fn test() {
        assert_eq!(solve_rotations("./inputs/day-1-input.txt").unwrap().0, 1092);
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            solve_rotations("./inputs/day-1-input-test.txt").unwrap().1,
            6
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(solve_rotations("./inputs/day-1-input.txt").unwrap().1, 6616);
    }
}
