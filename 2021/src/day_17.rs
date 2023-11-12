use std::{collections::HashSet, io};

use utils::{coordinate::Coordinate, file_utils::read_lines};

pub fn day_17() -> io::Result<i32> {
    find_highest_shot_from_file("./inputs/day-17-input.txt")
}
pub fn day_17_part_2() -> io::Result<i32> {
    find_working_vectors_from_file("./inputs/day-17-input.txt")
}

struct TargetArea {
    x_1: i32,
    y_1: i32,

    x_2: i32,
    y_2: i32,
}

struct ProbeState {
    position: Coordinate,
    vector: Coordinate,
}

fn load_target_area(filename: &str) -> io::Result<TargetArea> {
    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        let (x_1, x_2, y_2, y_1) =
            sscanf::sscanf!(line, "target area: x={}..{}, y={}..{}", i32, i32, i32, i32).unwrap();
        return Ok(TargetArea { x_1, y_1, x_2, y_2 });
    }
    panic!("Failed to find target area.")
}

fn process_probe_state(probe_state: &ProbeState) -> ProbeState {
    let new_position = probe_state.position + probe_state.vector;

    let drag = match probe_state.vector.x.cmp(&0) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => -1,
    };

    let new_vector = probe_state.vector + Coordinate::new(drag, -1);
    ProbeState {
        position: new_position,
        vector: new_vector,
    }
}

fn find_highest_shot_from_file(filename: &str) -> io::Result<i32> {
    let target_area = load_target_area(filename)?;
    let (highest_shot, _) = find_best_shot(&target_area);
    Ok(highest_shot)
}

fn find_working_vectors_from_file(filename: &str) -> io::Result<i32> {
    let target_area = load_target_area(filename)?;
    let (_, vectors) = find_best_shot(&target_area);
    Ok(vectors)
}

fn find_best_shot(target_area: &TargetArea) -> (i32, i32) {
    let mut high_y = 0;
    let mut working_vectors = HashSet::new();
    for y in -1_000..1_000 {
        for x in 1..1_000 {
            let mut probe_states = Vec::new();
            let mut probe_state = ProbeState {
                position: Coordinate::origin(),
                vector: Coordinate::new(x, y),
            };
            while probe_state.position.y >= target_area.y_2
                && probe_state.position.x <= target_area.x_2
            {
                let new_probe_state = process_probe_state(&probe_state);
                probe_states.push(probe_state);
                probe_state = new_probe_state;
            }
            if let Some(last_probe_state) = probe_states.last() {
                if last_probe_state.position.x >= target_area.x_1
                    && last_probe_state.position.x <= target_area.x_2
                    && last_probe_state.position.y <= target_area.y_1
                    && last_probe_state.position.y >= target_area.y_2
                {
                    working_vectors.insert((x, y));
                    let local_high_y = probe_states.iter().map(|p| p.position.y).max().unwrap();
                    if local_high_y > high_y {
                        high_y = local_high_y;
                    }
                }
            }
        }
    }

    (high_y, working_vectors.len() as i32)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn step_test() {
        let init = ProbeState {
            position: Coordinate::origin(),
            vector: Coordinate::new(7, 2),
        };

        let s_1 = process_probe_state(&init);
        assert_eq!(s_1.position, Coordinate::new(7, 2));

        let s_2 = process_probe_state(&s_1);
        assert_eq!(s_2.position, Coordinate::new(13, 3));

        let s_3 = process_probe_state(&s_2);
        assert_eq!(s_3.position, Coordinate::new(18, 3));

        let s_4 = process_probe_state(&s_3);
        assert_eq!(s_4.position, Coordinate::new(22, 2));

        let s_5 = process_probe_state(&s_4);
        assert_eq!(s_5.position, Coordinate::new(25, 0));

        let s_6 = process_probe_state(&s_5);
        assert_eq!(s_6.position, Coordinate::new(27, -3));

        let s_7 = process_probe_state(&s_6);
        assert_eq!(s_7.position, Coordinate::new(28, -7));
    }

    #[test]
    fn small_test() {
        assert_eq!(
            find_highest_shot_from_file("./inputs/day-17-input-test.txt").unwrap(),
            45
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            find_highest_shot_from_file("./inputs/day-17-input.txt").unwrap(),
            10011
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            find_working_vectors_from_file("./inputs/day-17-input-test.txt").unwrap(),
            112
        );
    }
    //
    //  #[test]
    //  fn part_2_test() {
    //      assert_eq!(
    //          count_polymer_parts(40, "./inputs/day-14-input.txt").unwrap(),
    //          3318837563123
    //      );
    //  }
}
