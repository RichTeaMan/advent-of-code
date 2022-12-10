use std::io;

use itertools::Itertools;

use crate::file_utils::read_lines;

#[derive(PartialEq, Eq, Clone, Hash, Debug, Copy, Ord, PartialOrd)]
struct CPU {
    pub x_reg: i32,

    pub cycle_count: i32,
}
impl CPU {
    fn new() -> CPU {
        CPU {
            x_reg: 1,
            cycle_count: 0,
        }
    }

    fn addx(&mut self, value: i32) {
        self.x_reg += value;
        self.cycle_count += 2;
    }

    fn noop(&mut self) {
        self.cycle_count += 1;
    }
}

pub fn day_10() -> io::Result<i32> {
    let positions = measure_cycles(20, 40, "./inputs/day-10-input.txt")?;
    Ok(positions)
}

pub fn day_10_part_2() -> io::Result<i32> {
    todo!();
}

fn measure_cycles(
    start_sample_cycle: i32,
    step_sample_cycle: i32,
    filename: &str,
) -> io::Result<i32> {
    let mut result = 0;
    let mut last_x = 0;

    let mut cpu = CPU::new();

    let mut sample_cycle = start_sample_cycle - 1;

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("noop") {
            cpu.noop();
        } else if line.starts_with("addx") {
            if let Some((_, v_s)) = line.split(' ').collect_tuple() {
                let v = v_s.parse::<i32>().unwrap();
                cpu.addx(v);
            }
        } else {
            panic!("Unknown command: {line}");
        }

        if cpu.cycle_count > sample_cycle {
            result += (sample_cycle + 1) * last_x;
            sample_cycle += step_sample_cycle;
        }
        last_x = cpu.x_reg;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            measure_cycles(20, 40, "./inputs/day-10-input-test.txt").unwrap(),
            13140
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            measure_cycles(20, 40, "./inputs/day-10-input.txt").unwrap(),
            15220
        );
    }
}
