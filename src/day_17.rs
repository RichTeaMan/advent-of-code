use std::io;

use crate::file_utils::read_lines;

const ARENA_HEIGHT: usize = 10_000;
const ARENA_WIDTH: usize = 7;

const LEFT_MARGIN: i32 = 2;
const DOWN_MARGIN: i32 = 3;

fn calculate_height(blocks_to_drop: i32, filename: &str) -> io::Result<i32> {
    let mut arena = [[false; ARENA_WIDTH]; ARENA_HEIGHT];

    let mut jets = Vec::new();
    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }
        for c in line.chars() {
            jets.push(c);
        }
    }
    // 'highest' point, but the lowest numbered because array indexing
    let mut highest = ARENA_HEIGHT as i32 - 1;

    let mut shape_id = 0;
    let mut shape = fetch_shape(shape_id);
    let mut shape_coord = (LEFT_MARGIN, highest - DOWN_MARGIN);
    let mut blocks_dropped = 0;

    for jet in jets.iter().cycle() {

        let mut x_move_amount = match jet {
            '<' => -1,
            '>' => 1,
            _ => panic!("Bad input"),
        };

        for (x, y) in &shape {
            let m_x = x + shape_coord.0 + x_move_amount;
            let m_y = y + shape_coord.1;
            if m_x < 0 || m_x >= ARENA_WIDTH as i32 || arena[m_y as usize][m_x as usize] {
                x_move_amount = 0;
                break;
            }
        }

        shape_coord.0 += x_move_amount;

        let mut y_move_amount = 1;

        for (x, y) in &shape {
            let m_x = x + shape_coord.0;
            let m_y = y + shape_coord.1 + y_move_amount;
            if m_y >= ARENA_HEIGHT as i32 || arena[m_y as usize][m_x as usize] {
                y_move_amount = 0;
                break;
            }
        }

        shape_coord.1 += y_move_amount;

        if y_move_amount == 0 {
            for (x, y) in &shape {
                let m_x = x + shape_coord.0;
                let m_y = y + shape_coord.1;

                highest = highest.min(m_y);

                arena[m_y as usize][m_x as usize] = true;
            }
            shape_id += 1;
            shape_id = shape_id % 5;

            shape = fetch_shape(shape_id);
            shape_coord = (
                LEFT_MARGIN,
                highest - DOWN_MARGIN - shape.last().unwrap().1 - 1,
            );

            blocks_dropped += 1;

            if blocks_dropped == blocks_to_drop {
                for j in 9975..ARENA_HEIGHT {
                    for c in arena[j] {
                        let cs = if c { "#" } else { "." };
                        print!("{cs}");
                    }
                    println!();
                }
                break;
            }
        }
    }

    Ok((ARENA_HEIGHT as i32 - highest) as i32)
}

fn fetch_shape(shape_id: i32) -> Vec<(i32, i32)> {
    /*
    ####

    .#.
    ###
    .#.

    ..#
    ..#
    ###

    #
    #
    #
    #

    ##
    ##
    */
    println!("{shape_id}");
    match shape_id {
        0 => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        1 => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        2 => vec![(2, 0), (2, 1), (0, 2), (1, 2), (2, 2)],
        3 => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        4 => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        _ => panic!("bad shape_id {shape_id}"),
    }
}

pub fn day_17() -> io::Result<i32> {
    let result = calculate_height(2022, "./inputs/day-17-input.txt").unwrap();
    Ok(result)
}

pub fn day_17_part_2() -> io::Result<i32> {
    todo!();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        let result = calculate_height(2022, "./inputs/day-17-input-test.txt").unwrap();
        assert_eq!(result, 3068);
    }

    #[test]
    fn test() {
        let result = calculate_height(2022, "./inputs/day-17-input.txt").unwrap();
        assert_eq!(result, 3193);
    }
}
