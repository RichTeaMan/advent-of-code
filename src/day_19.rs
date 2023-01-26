use std::{collections::VecDeque, io};

use itertools::Itertools;

use crate::file_utils::read_lines;

const PART_1_TIME: i32 = 24;
const PART_2_TIME: i32 = 32;

struct Blueprint {
    pub id: i32,

    pub ore_robot_ore_cost: i32,

    pub clay_robot_ore_cost: i32,

    pub obsidian_robot_ore_cost: i32,
    pub obsidian_robot_clay_cost: i32,

    pub geode_robot_ore_cost: i32,
    pub geode_robot_obsidian_cost: i32,
}

impl Blueprint {
    fn max_ore_bots_needed(&self) -> i32 {
        vec![
            self.ore_robot_ore_cost,
            self.clay_robot_ore_cost,
            self.obsidian_robot_ore_cost,
            self.geode_robot_ore_cost,
        ]
        .iter()
        .max()
        .unwrap()
        .to_owned()
    }
    fn max_clay_bots_needed(&self) -> i32 {
        self.obsidian_robot_clay_cost
    }
    fn max_obsidian_bots_needed(&self) -> i32 {
        self.geode_robot_obsidian_cost
    }
}

#[derive(Default, Clone, Copy, Debug)]
struct State {
    pub blueprint_id: i32,

    /**
     * Gets the time this state exists on. It is not the remaining time.
     */
    pub time: i32,

    pub ore: i32,
    pub clay: i32,
    pub obsidian: i32,
    pub geode: i32,

    pub ore_robots: i32,
    pub clay_robots: i32,
    pub obsidian_robots: i32,
    pub geode_robots: i32,
}

impl State {
    fn time_to_given_ore(&self, desired: i32) -> i32 {
        if self.ore >= desired {
            return 0;
        }
        if self.ore_robots <= 0 {
            return i32::MAX;
        }

        let r = (desired - self.ore) % self.ore_robots;
        (desired - self.ore) / self.ore_robots + if r == 0 { 0 } else { 1 }
    }

    fn time_to_given_clay(&self, desired: i32) -> i32 {
        if self.clay >= desired {
            return 0;
        }
        if self.clay_robots <= 0 {
            return i32::MAX;
        }

        let r = (desired - self.clay) % self.clay_robots;
        (desired - self.clay) / self.clay_robots + if r == 0 { 0 } else { 1 }
    }

    fn time_to_given_obsidian(&self, desired: i32) -> i32 {
        if self.obsidian >= desired {
            return 0;
        }
        if self.obsidian_robots <= 0 {
            return i32::MAX;
        }

        let r = (desired - self.obsidian) % self.obsidian_robots;
        (desired - self.obsidian) / self.obsidian_robots + if r == 0 { 0 } else { 1 }
    }

    fn time_for_ore_bot_resources(&self, blueprint: &Blueprint) -> i32 {
        self.time_to_given_ore(blueprint.ore_robot_ore_cost)
    }

    fn time_for_clay_bot_resources(&self, blueprint: &Blueprint) -> i32 {
        self.time_to_given_ore(blueprint.clay_robot_ore_cost)
    }

    fn time_for_obsidian_bot_resources(&self, blueprint: &Blueprint) -> i32 {
        self.time_to_given_ore(blueprint.obsidian_robot_ore_cost)
            .max(self.time_to_given_clay(blueprint.obsidian_robot_clay_cost))
    }

    fn time_for_geode_bot_resources(&self, blueprint: &Blueprint) -> i32 {
        self.time_to_given_ore(blueprint.geode_robot_ore_cost)
            .max(self.time_to_given_obsidian(blueprint.geode_robot_obsidian_cost))
    }

    fn is_valid(&self) -> bool {
        self.ore >= 0 && self.clay >= 0 && self.obsidian >= 0 && self.geode >= 0
    }

    /**
     * Calculates the maximum number of geodes that can be created if resources are not an issue.
     */
    fn max_geodes(&self, total_time: i32) -> i32 {
        let remaining = total_time - self.time;
        let geodes = (((remaining - 1) * (remaining + 0)) / 2)
            + (remaining * self.geode_robots)
            + self.geode;

        geodes
    }

    fn quality_level(&self) -> i32 {
        self.geode * self.blueprint_id
    }
}

fn load_blueprints(filename: &str) -> io::Result<Vec<Blueprint>> {
    let mut blueprints = Vec::new();

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        // Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
        let parsed = sscanf::sscanf!(
            line,
            "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.",
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32
        );

        if let Ok((
            id,
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost,
        )) = parsed
        {
            let blueprint = Blueprint {
                id,
                ore_robot_ore_cost,
                clay_robot_ore_cost,
                obsidian_robot_ore_cost,
                obsidian_robot_clay_cost,
                geode_robot_ore_cost,
                geode_robot_obsidian_cost,
            };
            blueprints.push(blueprint);
        } else {
            panic!("Bad input: {line}");
        }
    }
    Ok(blueprints)
}

fn calculate_quality_levels(blueprints: Vec<Blueprint>, total_time: i32) -> i32 {
    let mut result = 0;
    for blueprint in blueprints {
        let best_state = simulate_blueprint(&blueprint, total_time);
        result += best_state.quality_level();
    }

    result
}

fn simulate_blueprint(blueprint: &Blueprint, total_time: i32) -> State {
    let mut start_state = State {
        blueprint_id: blueprint.id,
        ..Default::default()
    };

    start_state.ore_robots = 1;
    let mut best_state_opt: Option<State> = None;

    let max_ore_bots_needed = blueprint.max_ore_bots_needed();
    let max_clay_bots_needed = blueprint.max_clay_bots_needed();
    let max_obs_bots_needed = blueprint.max_obsidian_bots_needed();

    let mut stack: VecDeque<State> = VecDeque::new();
    stack.push_back(start_state);

    while let Some(mut current) = stack.pop_front() {
        debug_assert!(current.is_valid());

        if let Some(best_state) = &best_state_opt {
            let max = current.max_geodes(total_time);
            if max == 0 || max <= best_state.geode {
                continue;
            }
        }

        current.time += 1;
        if current.time >= total_time {
            current.ore += current.ore_robots;
            current.clay += current.clay_robots;
            current.obsidian += current.obsidian_robots;
            current.geode += current.geode_robots;

            if let Some(best_state) = &best_state_opt {
                if current.geode > best_state.geode {
                    best_state_opt = Some(current);
                }
            } else {
                best_state_opt = Some(current);
            }

            continue;
        }

        let mut did_action = false;

        // make ore robot
        if current.ore_robots > 0 && current.ore_robots < max_ore_bots_needed {
            let wait = current.time_for_ore_bot_resources(blueprint);
            let time = wait + current.time;
            if time < total_time {
                let mut ore_robot_choice = current.clone();

                for _ in 0..=wait {
                    ore_robot_choice.ore += current.ore_robots;
                    ore_robot_choice.clay += current.clay_robots;
                    ore_robot_choice.obsidian += current.obsidian_robots;
                    ore_robot_choice.geode += current.geode_robots;
                }

                ore_robot_choice.ore -= blueprint.ore_robot_ore_cost;
                ore_robot_choice.ore_robots += 1;
                ore_robot_choice.time = time;

                stack.push_front(ore_robot_choice);
                did_action = true;
            }
        }

        // make clay robot
        if current.ore_robots > 0 && current.clay_robots < max_clay_bots_needed {
            let wait = current.time_for_clay_bot_resources(blueprint);
            let time = wait + current.time;
            if time < total_time {
                let mut clay_robot_choice = current.clone();

                for _ in 0..=wait {
                    clay_robot_choice.ore += current.ore_robots;
                    clay_robot_choice.clay += current.clay_robots;
                    clay_robot_choice.obsidian += current.obsidian_robots;
                    clay_robot_choice.geode += current.geode_robots;
                }

                clay_robot_choice.ore -= blueprint.clay_robot_ore_cost;
                clay_robot_choice.clay_robots += 1;
                clay_robot_choice.time = time;

                stack.push_front(clay_robot_choice);
                did_action = true;
            }
        }

        // make obsidian robot
        if current.ore_robots > 0
            && current.clay_robots > 0
            && current.obsidian_robots < max_obs_bots_needed
        {
            let wait = current.time_for_obsidian_bot_resources(blueprint);
            let time = wait + current.time;
            if time < total_time {
                let mut obsidian_robot_choice = current.clone();

                for _ in 0..=wait {
                    obsidian_robot_choice.ore += current.ore_robots;
                    obsidian_robot_choice.clay += current.clay_robots;
                    obsidian_robot_choice.obsidian += current.obsidian_robots;
                    obsidian_robot_choice.geode += current.geode_robots;
                }

                obsidian_robot_choice.ore -= blueprint.obsidian_robot_ore_cost;
                obsidian_robot_choice.clay -= blueprint.obsidian_robot_clay_cost;
                obsidian_robot_choice.obsidian_robots += 1;
                obsidian_robot_choice.time = time;

                stack.push_front(obsidian_robot_choice);
                did_action = true;
            }
        }

        // make geode robot
        if current.ore_robots > 0 && current.obsidian_robots > 0 {
            let wait = current.time_for_geode_bot_resources(blueprint);
            let time = wait + current.time;
            if time < total_time {
                let mut geode_robot_choice = current.clone();

                for _ in 0..=wait {
                    geode_robot_choice.ore += current.ore_robots;
                    geode_robot_choice.clay += current.clay_robots;
                    geode_robot_choice.obsidian += current.obsidian_robots;
                    geode_robot_choice.geode += current.geode_robots;
                }

                geode_robot_choice.ore -= blueprint.geode_robot_ore_cost;
                geode_robot_choice.obsidian -= blueprint.geode_robot_obsidian_cost;
                geode_robot_choice.geode_robots += 1;
                geode_robot_choice.time = time;

                stack.push_front(geode_robot_choice);
                did_action = true;
            }
        }

        if !did_action {
            for _ in current.time..(total_time) {
                current.ore += current.ore_robots;
                current.clay += current.clay_robots;
                current.obsidian += current.obsidian_robots;
                current.geode += current.geode_robots;
                current.time += 1;
            }
            stack.push_front(current);
        }
    }
    if let Some(best_state) = best_state_opt {
        return best_state;
    } else {
        panic!(
            "A solution was not found for blueprint {bid}.",
            bid = blueprint.id
        );
    }
}

pub fn day_19() -> io::Result<i32> {
    let blueprints = load_blueprints("./inputs/day-19-input.txt")?;
    let result = calculate_quality_levels(blueprints, PART_1_TIME);
    Ok(result)
}

pub fn day_19_part_2() -> io::Result<i32> {
    let blueprints = load_blueprints("./inputs/day-19-input.txt")?;
    let results = blueprints
        .iter()
        .take(3)
        .map(|f| simulate_blueprint(f, PART_2_TIME).geode)
        .collect_vec();
    Ok(results.iter().product())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn max_geode_test() {
        let state = State {
            time: 10,
            ..Default::default()
        };
        let max = state.max_geodes(10);
        assert_eq!(0, max);
    }

    #[test]
    fn single_blueprint_test() {
        let blueprints = load_blueprints("./inputs/day-19-input-test.txt").unwrap();
        let best_state = simulate_blueprint(&blueprints[0], PART_1_TIME);
        assert_eq!(best_state.quality_level(), 9);
    }

    #[test]
    fn small_test() {
        let blueprints = load_blueprints("./inputs/day-19-input-test.txt").unwrap();
        let result = calculate_quality_levels(blueprints, PART_1_TIME);
        assert_eq!(result, 33);
    }

    #[test]
    fn test() {
        let blueprints = load_blueprints("./inputs/day-19-input.txt").unwrap();
        let result = calculate_quality_levels(blueprints, PART_1_TIME);
        assert_eq!(result, 1349);
    }

    #[test]
    fn part_2_small_test() {
        let blueprints = load_blueprints("./inputs/day-19-input-test.txt").unwrap();
        let result = simulate_blueprint(&blueprints[0], PART_2_TIME);
        assert_eq!(result.geode, 56);
    }

    #[test]
    pub fn part_2_test() {
        let blueprints = load_blueprints("./inputs/day-19-input.txt").unwrap();
        let results = blueprints
            .iter()
            .take(3)
            .map(|f| simulate_blueprint(f, PART_2_TIME).geode)
            .collect_vec();
        let result = results.iter().product();
        assert_eq!(21840, result);
    }
}
