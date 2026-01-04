use core::num;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{self}, iter,
};
use utils::file_utils::read_lines;

pub fn day_10() -> io::Result<u64> {
    calc_presses("./inputs/day-10-input.txt")
}

pub fn day_10_part_2() -> io::Result<u64> {
    calc_jolt_presses2("./inputs/day-10-input.txt")
}

struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<u16>>,
    jolts: Vec<u16>,
}

fn l(state: Vec<bool>, machine: &Machine) -> Vec<Vec<bool>> {
    let mut states = Vec::new();

    for button in &machine.buttons {
        let mut new_state = state.clone();

        for i in button {
            let current = new_state.get(*i as usize).unwrap().clone();
            let o = new_state.get_mut(*i as usize).unwrap();
            *o = !current;
        }

        debug_assert_ne!(state, new_state);

        states.push(new_state);
    }

    return states;
}

fn jolt_state_change(state: &Vec<u16>, machine: &Machine) -> Vec<Vec<u16>> {
    let mut states = Vec::with_capacity(machine.buttons.len());

    for button in &machine.buttons {
        let mut new_state = state.clone();

        for i in button {
            let o = new_state.get_mut(*i as usize).unwrap();
            *o += 1;
        }

        debug_assert_ne!(*state, new_state);

        states.push(new_state);
    }

    return states;
}

fn jolt_state_change2(state: &Vec<u16>, machine: &Machine) -> Vec<(Vec<u16>, usize)> {
    let mut states = Vec::with_capacity(machine.buttons.len());

    for (bi, button) in machine.buttons.iter().enumerate() {
        let mut new_state = state.clone();

        for i in button {
            let o = new_state.get_mut(*i as usize).unwrap();
            *o += 1;
        }

        debug_assert_ne!(*state, new_state);

        states.push((new_state, bi));
    }

    return states;
}


fn calc_presses(filename: &str) -> io::Result<u64> {
    let machines = fetch_machines(filename)?;

    let mut result = 0_u64;

    for machine in machines {
        let mut start_state = Vec::new();
        for _ in 0..(machine.lights.len() as usize) {
            start_state.push(false);
        }

        let mut states = HashSet::from_iter(l(start_state, &machine));
        let mut presses = 1;

        let mut previous_states = HashSet::new();

        loop {
            if states.contains(&machine.lights) {
                break;
            }
            presses += 1;
            let mut new_states = HashSet::new();
            for s in states {
                for n_s in l(s, &machine) {
                    if !previous_states.contains(&n_s) {
                        previous_states.insert(n_s.clone());
                        new_states.insert(n_s);
                    }
                }
            }
            states = new_states;
        }

        result += presses as u64;
    }
    Ok(result)
}

fn calc_jolt_presses(filename: &str) -> io::Result<u64> {
    let machines = fetch_machines(filename)?;

    let mut result = 0_u64;

    for (mi, machine) in machines.iter().enumerate() {
        println!(
            "Starting machine {} ([{}])",
            mi,
            machine
                .lights
                .iter()
                .map(|c| if *c { '#' } else { '.' })
                .join("")
        );
        let mut start_state = Vec::new();
        for _ in 0..(machine.jolts.len() as usize) {
            start_state.push(0_u16);
        }

        let mut queue = VecDeque::new();
        queue.push_front((start_state, 0));

        let mut answer = 0;
        for j in fetch_max_button_presses(&machine) {
            answer += j;
        }

        let mut previous_states = HashSet::new();

        while queue.len() > 0 {
            let (state, presses) = queue.pop_front().unwrap();
            if presses >= answer {
                continue;
            }
            let new_presses = presses + 1;
            for n_s in jolt_state_change(&state, &machine) {
                if n_s == machine.jolts {
                    answer = new_presses;
                    continue;
                }
                
                let mut should_add = true;
                for (i, n) in n_s.iter().enumerate() {
                    let existing = machine.jolts.get(i).unwrap();
                    if n > existing {
                        should_add = false;
                        break;
                    }
                }
                if previous_states.contains(&(n_s.clone(), new_presses)){
                    continue;
                }
                previous_states.insert((n_s.clone(), new_presses));

                if should_add {
                    queue.push_front((n_s, new_presses));
                }
            }
        }

        result += answer as u64;
    }
    Ok(result)
}


fn calc_jolt_presses2(filename: &str) -> io::Result<u64> {
    let machines = fetch_machines(filename)?;

    let mut result = 0_u64;

    for (mi, machine) in machines.iter().enumerate() {
        println!(
            "Starting machine {} ([{}])",
            mi,
            machine
                .lights
                .iter()
                .map(|c| if *c { '#' } else { '.' })
                .join("")
        );
        let mut start_state = Vec::new();
        for _ in 0..(machine.jolts.len() as usize) {
            start_state.push(0_u16);
        }
        let mut start_presses = Vec::new();
        for _ in 0..(machine.buttons.len()) {
            start_presses.push(0_u16);
        }

        let mut queue = VecDeque::new();
        queue.push_front((start_state, start_presses, 0));

        let button_presses = fetch_max_button_presses(&machine);
        let mut answer = 0;
        let mut combos = 1_u64;
        for j in &button_presses {
            answer += j;
            combos *= *j as u64;
        }
        println!("    {} combinations, {} presses -> {:?}", combos, answer, button_presses);

        let mut previous_states = HashSet::new();

        while queue.len() > 0 {
            let (state, press_values, presses) = queue.pop_front().unwrap();
            if presses >= answer {
                continue;
            }
            let new_presses = presses + 1;
            for (n_s, i_n_s) in jolt_state_change2(&state, &machine) {
                
                let mut new_press_values = press_values.clone();
                let p = new_press_values.get_mut(i_n_s).unwrap();
                *p += 1;

                if n_s == machine.jolts {
                    answer = new_presses;
                    println!("({}) {:?}", new_presses, new_press_values);
                    continue;
                }

                if  new_press_values.get(i_n_s).unwrap() > button_presses.get(i_n_s).unwrap() {
                    continue;
                }
                
                let mut should_add = true;
                for (i, n) in n_s.iter().enumerate() {
                    let existing = machine.jolts.get(i).unwrap();
                    if n > existing {
                        should_add = false;
                        break;
                    }
                }

                if previous_states.contains(&(n_s.clone(), new_presses)){
                    continue;
                }
                previous_states.insert((n_s.clone(), new_presses));

                if should_add {
                    queue.push_front((n_s, new_press_values, new_presses));
                }
            }
        }
        println!("    complete: {}", answer);

        result += answer as u64;
    }
    Ok(result)
}



fn fetch_max_button_presses(machine: &Machine) -> Vec<u16> {
    let mut button_max = Vec::new();

    for b in &machine.buttons {
        let mut max = machine.jolts.iter().max().unwrap();
        for i in b {
            let presses = machine.jolts.get(*i as usize).unwrap();
            if presses < max {
                max = presses;
            }
        }
        button_max.push(max.clone());
    }
    button_max
}


fn fetch_machines(filename: &str) -> io::Result<Vec<Machine>> {
    let mut result = Vec::new();

    let lines = read_lines(filename)?;
    let trims = ['(', ')', '[', ']', '{', '}'];
    for line in lines.map_while(Result::ok) {
        if line.is_empty() {
            continue;
        }
        let mut lights = None;
        let mut buttons = Vec::new();
        let mut jolts = None;
        let parts = line.split(" ");
        for part in parts {
            let mut p = part.trim().to_string();
            let is_light = p.starts_with('[');
            let is_button = p.starts_with('(');
            let is_jolt = p.starts_with('{');
            p = p.trim_matches(trims).to_string();

            if is_light {
                lights = Some(p.chars().into_iter().map(|c| c == '#').collect_vec());
            } else if is_button || is_jolt {
                let numbers = p
                    .split(',')
                    .map(|n| n.parse::<u16>().unwrap())
                    .collect_vec();
                if is_button {
                    buttons.push(numbers);
                } else {
                    jolts = Some(numbers);
                }
            } else {
                panic!("Unable to parse {}", p);
            }
        }
        result.push(Machine {
            lights: lights.unwrap(),
            buttons,
            jolts: jolts.unwrap(),
        });
    }
    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(calc_presses("./inputs/day-10-input-test.txt").unwrap(), 7);
    }

    #[test]
    fn test() {
        assert_eq!(calc_presses("./inputs/day-10-input.txt").unwrap(), 491);
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            calc_jolt_presses2("./inputs/day-10-input-test.txt").unwrap(),
            33
        );
    }
    //
    //   #[test]
    //   fn part_2_test() {
    //       assert_eq!(
    //           calc_shapes("./inputs/day-10-input.txt").unwrap(),
    //           1452422268
    //       );
    //   }
}
