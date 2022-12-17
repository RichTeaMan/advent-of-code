use std::{
    collections::{HashMap, HashSet},
    io,
};

use crate::file_utils::read_lines;

const ARENA_WIDTH: usize = 7;

const LEFT_MARGIN: i32 = 2;
const DOWN_MARGIN: i32 = 3;

const ROW_CACHE_SIZE: usize = 18;

#[derive(Eq, PartialEq, Hash)]
struct CaveSequenceState {
    jet_index: u16,
    shape_undex: u8,
    rows: [u8; ROW_CACHE_SIZE],
}

struct CaveSequenceInfo {
    height: i32,
    rock_number: u64,
}

fn calculate_height(blocks_to_drop: u64, filename: &str) -> io::Result<u64> {
    let mut arena = HashSet::new();

    for x in 0..ARENA_WIDTH {
        arena.insert((x as i32, 0));
    }

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

    let mut top: i32 = 0;

    let mut shape_id = 0;
    let mut shape = fetch_shape(shape_id);
    let mut shape_coord = (LEFT_MARGIN, DOWN_MARGIN as i32 + 1);
    let mut blocks_dropped = 0;

    let mut sequence_found = false;

    let mut sequence_map: HashMap<CaveSequenceState, CaveSequenceInfo> = HashMap::new();

    let mut top_mod: u64 = 0;

    for (total_jet_index, jet) in jets.iter().cycle().enumerate() {
        let jet_index = total_jet_index % jets.len();
        let mut x_move_amount = match jet {
            '<' => -1,
            '>' => 1,
            _ => panic!("Bad input"),
        };

        for (x, y) in &shape {
            let m_x = x + shape_coord.0 + x_move_amount;
            let m_y = y + shape_coord.1;

            if m_x < 0 || m_x >= ARENA_WIDTH as i32 || arena.contains(&(m_x, m_y)) {
                x_move_amount = 0;
                break;
            }
        }

        shape_coord.0 += x_move_amount;

        let mut y_move_amount = -1;

        for (x, y) in &shape {
            let m_x = x + shape_coord.0;
            let m_y = y + shape_coord.1 + y_move_amount;
            if m_y < 0 || arena.contains(&(m_x, m_y)) {
                y_move_amount = 0;
                break;
            }
        }

        shape_coord.1 += y_move_amount;

        if y_move_amount == 0 {
            for (x, y) in &shape {
                let m_x = x + shape_coord.0;
                let m_y = y + shape_coord.1;

                top = top.max(m_y);

                arena.insert((m_x, m_y));
            }

            if blocks_dropped > 0 && !sequence_found {
                let mut cache_rows = [0_u8; ROW_CACHE_SIZE];
                for i in 0..ROW_CACHE_SIZE {
                    let y = top - i as i32;
                    let mut res = 0;
                    for x in 0..ARENA_WIDTH as i32 {
                        res += if arena.contains(&(x, y)) { 1 << x } else { 0 };
                    }
                    cache_rows[i] = res;
                }

                let key = CaveSequenceState {
                    jet_index: jet_index as u16,
                    shape_undex: shape_id as u8,
                    rows: cache_rows,
                };

                if let Some(info) = sequence_map.get(&key) {
                    let height_delta = top - info.height;
                    let block_amount_delta = blocks_dropped - info.rock_number;
                    let adj = (blocks_to_drop - blocks_dropped) / block_amount_delta;

                    top_mod = height_delta as u64 * adj;
                    blocks_dropped += adj * block_amount_delta;
                    sequence_found = true;
                } else {
                    sequence_map.insert(
                        key,
                        CaveSequenceInfo {
                            height: top,
                            rock_number: blocks_dropped,
                        },
                    );
                }
            }

            shape_id += 1;
            shape_id = shape_id % 5;

            shape = fetch_shape(shape_id);
            shape_coord = (LEFT_MARGIN, top + DOWN_MARGIN as i32 + 1);
            assert!(shape_coord.1 >= 0);

            blocks_dropped += 1;
            if blocks_dropped == blocks_to_drop {
                break;
            }
        }
    }

    Ok(top as u64 + top_mod)
}

fn fetch_shape(shape_id: i32) -> Vec<(i32, i32)> {
    /*
    ....
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
    match shape_id {
        0 => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        1 => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        2 => vec![(2, 2), (2, 1), (2, 0), (1, 0), (0, 0)],
        3 => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        4 => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        _ => panic!("bad shape_id {shape_id}"),
    }
}

pub fn day_17() -> io::Result<u64> {
    let result = calculate_height(2022, "./inputs/day-17-input.txt")?;
    Ok(result)
}

pub fn day_17_part_2() -> io::Result<u64> {
    let result = calculate_height(1_000_000_000_000, "./inputs/day-17-input.txt")?;
    Ok(result)
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

    #[test]
    fn part_2_small_test() {
        let result = calculate_height(1_000_000_000_000, "./inputs/day-17-input-test.txt").unwrap();
        let expected = 1_514_285_714_288;
        assert_eq!(result, expected);
    }

    #[test]
    fn part_2_test() {
        let result = calculate_height(1_000_000_000_000, "./inputs/day-17-input.txt").unwrap();
        let expected = 1_577_650_429_835;
        assert_eq!(result, expected);
    }
}
