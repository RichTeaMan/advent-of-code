use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{self},
};

use itertools::Itertools;
use utils::coordinate_3d::Coordinate3d;
use utils::file_utils::read_lines;

const ENERGY_LIMIT: i32 = 10;

const CHECK_DIMENSION: i32 = 50;

struct CubeInstruction {
    on: bool,
    x_1: i32,
    x_2: i32,
    y_1: i32,
    y_2: i32,
    z_1: i32,
    z_2: i32,
}

pub fn day_22() -> io::Result<i32> {
    calc_active_cubes_from_file("./inputs/day-22-input.txt")
}
pub fn day_22_part_2() -> io::Result<i32> {
    panic!() // find_first_synced_flash("./inputs/day-22-input.txt")
}

fn load_cubes(filename: &str) -> io::Result<Vec<CubeInstruction>> {
    let mut cube_instructions = Vec::new();

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        let (on_str, x_1, x_2, y_1, y_2, z_1, z_2) = sscanf::sscanf!(
            line,
            "{} x={}..{},y={}..{},z={}..{}",
            String,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32
        )
        .unwrap();

        let mut x = vec![x_1, x_2];
        x.sort();
        let (x_1, x_2) = x.iter().collect_tuple().unwrap();

        let mut y = vec![y_1, y_2];
        y.sort();
        let (y_1, y_2) = y.iter().collect_tuple().unwrap();

        let mut z = vec![z_1, z_2];
        z.sort();
        let (z_1, z_2) = z.iter().collect_tuple().unwrap();

        let on = on_str == "on";

        cube_instructions.push(CubeInstruction {
            on,
            x_1: *x_1,
            x_2: *x_2,
            y_1: *y_1,
            y_2: *y_2,
            z_1: *z_1,
            z_2: *z_2,
        });
    }
    Ok(cube_instructions)
}

fn calc_active_cubes(cube_instructions: &Vec<CubeInstruction>) -> i32 {
    let mut cubes = HashMap::new();
    for x in -CHECK_DIMENSION..=CHECK_DIMENSION {
        for y in -CHECK_DIMENSION..=CHECK_DIMENSION {
            for z in -CHECK_DIMENSION..=CHECK_DIMENSION {
                let cube = Coordinate3d::new(x, y, z);
                cubes.insert(cube, false);
            }
        }
    }

    for cube_instruction in cube_instructions {
        for x in cube_instruction.x_1..=cube_instruction.x_2 {
            if x < -CHECK_DIMENSION || x > CHECK_DIMENSION {
                continue;
            }
            for y in cube_instruction.y_1..=cube_instruction.y_2 {
                if y < -CHECK_DIMENSION || y > CHECK_DIMENSION {
                    continue;
                }
                for z in cube_instruction.z_1..=cube_instruction.z_2 {
                    if z < -CHECK_DIMENSION || z > CHECK_DIMENSION {
                        continue;
                    }
                    let cube = Coordinate3d::new(x, y, z);
                    if let Some(c) = cubes.get_mut(&cube) {
                        *c = cube_instruction.on;
                    }
                }
            }
        }
    }
    cubes.iter().filter(|c| *c.1).count() as i32
}

fn calc_active_cubes_from_file(filepath: &str) -> io::Result<i32> {
    let cube_instructions = load_cubes(filepath)?;
    Ok(calc_active_cubes(&cube_instructions))
}

fn calc_active_cubes_complicated(cube_instructions: &Vec<CubeInstruction>) -> i32 {
    let mut cubes = Vec::new();

    for cube_instruction in cube_instructions {
        // find overlaps
        for c in &cubes {
            if c.x_1 >= cube_instruction.x_1 && c.x_1 <= cube_instruction.x_2 {

                cubes
            }
        }

        for x in cube_instruction.x_1..=cube_instruction.x_2 {
            if x < -CHECK_DIMENSION || x > CHECK_DIMENSION {
                continue;
            }
            for y in cube_instruction.y_1..=cube_instruction.y_2 {
                if y < -CHECK_DIMENSION || y > CHECK_DIMENSION {
                    continue;
                }
                for z in cube_instruction.z_1..=cube_instruction.z_2 {
                    if z < -CHECK_DIMENSION || z > CHECK_DIMENSION {
                        continue;
                    }
                    let cube = Coordinate3d::new(x, y, z);
                    if let Some(c) = cubes.get_mut(&cube) {
                        *c = cube_instruction.on;
                    }
                }
            }
        }
    }
    cubes.iter().filter(|c| *c.1).count() as i32
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            calc_active_cubes_from_file("./inputs/day-22-input-test.txt").unwrap(),
            590784
        );
    }

    //  #[test]
    //  fn test() {
    //      assert_eq!(run_steps("./inputs/day-11-input.txt", 100).unwrap(), 1608);
    //  }
    //
    //  #[test]
    //  fn part_2_small_test() {
    //      assert_eq!(
    //          find_first_synced_flash("./inputs/day-11-input-test.txt").unwrap(),
    //          195
    //      );
    //  }
    //
    //  #[test]
    //  fn part_2_test() {
    //      assert_eq!(
    //          find_first_synced_flash("./inputs/day-11-input.txt").unwrap(),
    //          214
    //      );
    //  }
}
