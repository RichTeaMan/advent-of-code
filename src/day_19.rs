use std::{
    collections::{HashSet, VecDeque},
    default, io,
};

use itertools::Itertools;

use crate::file_utils::read_lines;

const TOTAL_TIME: i32 = 24;

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

#[derive(Debug)]
enum RobotDecision {
    ORE,
    CLAY,
    OBSIDIAN,
    GEODE,
    NONE,
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

    fn message(&self) {
        println!(
            "{r} ore-collecting robot; you now have {m} ore.",
            r = self.ore_robots,
            m = self.ore
        );
        println!(
            "{r} clay-collecting robots; you now have {m} clay.",
            r = self.clay_robots,
            m = self.clay
        );
        println!(
            "{r} obsidian-collecting robots; you now have {m} obsidian.",
            r = self.obsidian_robots,
            m = self.obsidian
        );
        println!(
            "{r} geode-cracking robots; you now have {m} geodes.",
            r = self.geode_robots,
            m = self.geode
        );
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

fn calculate_quality_levels(blueprints: Vec<Blueprint>) -> i32 {
    let mut result = 0;
    for blueprint in blueprints {
        let best_state = simulate_blueprint(&blueprint);

        print!("{best_state:?}");

        result += best_state.quality_level();
    }

    result
}

fn heuristic(blueprint: &Blueprint, start_state: State) -> State {
    let mut current = start_state;
    for t in 0..TOTAL_TIME {
        let decision = robot_decision(blueprint, &current);

        match decision {
            RobotDecision::ORE => {
                current.ore -= blueprint.ore_robot_ore_cost;
            }
            RobotDecision::CLAY => {
                current.ore -= blueprint.clay_robot_ore_cost;
            }
            RobotDecision::OBSIDIAN => {
                current.ore -= blueprint.obsidian_robot_ore_cost;
                current.clay -= blueprint.obsidian_robot_clay_cost;
            }
            RobotDecision::GEODE => {
                current.ore -= blueprint.geode_robot_ore_cost;
                current.obsidian -= blueprint.geode_robot_obsidian_cost;
            }
            RobotDecision::NONE => (),
        }

        assert!(current.ore >= 0);
        assert!(current.clay >= 0);
        assert!(current.obsidian >= 0);
        assert!(current.geode >= 0);

        current.ore += current.ore_robots;
        current.clay += current.clay_robots;
        current.obsidian += current.obsidian_robots;
        current.geode += current.geode_robots;
        current.time += 1;

        match decision {
            RobotDecision::ORE => {
                current.ore_robots += 1;
            }
            RobotDecision::CLAY => {
                current.clay_robots += 1;
            }
            RobotDecision::OBSIDIAN => {
                current.obsidian_robots += 1;
            }
            RobotDecision::GEODE => {
                current.geode_robots += 1;
            }
            RobotDecision::NONE => (),
        }

        let r = t + 1;
        println!("Minute {r} Bulding {decision:?}.");
        current.message();
        println!();
    }

    current
}

fn robot_decision(blueprint: &Blueprint, state: &State) -> RobotDecision {
    if state.time == 21 {
        println!("debug");
    }

    // can make geode?
    if state.ore >= blueprint.geode_robot_ore_cost
        && state.obsidian >= blueprint.geode_robot_obsidian_cost
    {
        return RobotDecision::GEODE;
    }

    let time_to_geode_bot = state.time_for_geode_bot_resources(blueprint);
    if time_to_geode_bot < i32::MAX {
        let mut obs_state = state.clone();
        obs_state.obsidian_robots += 1;
        obs_state.obsidian -= blueprint.geode_robot_obsidian_cost;
        obs_state.ore -= blueprint.geode_robot_ore_cost;

        let time_to_geode_with_extra_obs = obs_state.time_for_geode_bot_resources(blueprint) + 1;

        if time_to_geode_bot >= time_to_geode_with_extra_obs && obs_state.is_valid() {
            return RobotDecision::GEODE;
        }
        if time_to_geode_bot == 1 {
            return RobotDecision::NONE;
        }
    }

    if state.ore >= blueprint.obsidian_robot_ore_cost
        && state.clay >= blueprint.obsidian_robot_clay_cost
    {
        return RobotDecision::OBSIDIAN;
    }

    let time_to_obs_bot = state.time_for_obsidian_bot_resources(blueprint);
    if time_to_obs_bot < i32::MAX {
        let mut clay_state = state.clone();
        clay_state.clay_robots += 1;
        clay_state.ore -= blueprint.clay_robot_ore_cost;
        let time_to_obs_with_extra_clay = clay_state.time_for_obsidian_bot_resources(blueprint) + 1;

        let mut ore_state = state.clone();
        ore_state.ore_robots += 1;
        ore_state.ore -= blueprint.ore_robot_ore_cost;
        let time_to_obs_with_extra_ore = ore_state.time_for_obsidian_bot_resources(blueprint) + 1;

        let mut optimal = time_to_obs_bot;
        let mut selection = RobotDecision::NONE;
        if optimal >= time_to_obs_with_extra_clay && clay_state.is_valid() {
            selection = RobotDecision::CLAY;
            optimal = time_to_obs_with_extra_clay;
        }
        if optimal >= time_to_obs_with_extra_ore && ore_state.is_valid() {
            selection = RobotDecision::ORE
        }
        return selection;
    }

    let time_to_clay_bot = state.time_for_clay_bot_resources(blueprint);
    if time_to_clay_bot == 0 {
        return RobotDecision::CLAY;
    }

    let time_to_ore_bot = state.time_for_ore_bot_resources(blueprint);
    if time_to_ore_bot == 0 {
        let mut ore_state = state.clone();
        ore_state.ore_robots += 1;
        ore_state.ore -= blueprint.ore_robot_ore_cost;
        let time_to_clay_with_extra_ore = ore_state.time_for_ore_bot_resources(blueprint) + 1;

        if time_to_clay_bot <= time_to_clay_with_extra_ore {
            return RobotDecision::ORE;
        }
    }

    RobotDecision::NONE
}

fn simulate_blueprint(blueprint: &Blueprint) -> State {
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

    let mut counter = 0_u64;
    let mut max_geode_culls = 0_u64;
    while let Some(mut current) = stack.pop_front() {
        if counter % 1_000_000_000 == 0 {
            //if counter % 10000 == 0 {
            println!(
                "Loop {c}. {s} items in stack.",
                c = counter,
                s = stack.len()
            );
        }
        counter += 1;
        
        debug_assert!(current.is_valid());
        
        //current.debug_states.push(current.clone());
        
        //current.ore += current.ore_robots;
        //current.clay += current.clay_robots;
        //current.obsidian += current.obsidian_robots;
        //current.geode += current.geode_robots;
        
        if let Some(best_state) = &best_state_opt {
            let max = current.max_geodes(TOTAL_TIME);
            if max == 0 || max <= best_state.geode {
                max_geode_culls +=1;
                //println!("culled");
                continue;
            }
        }
        
        if current.ore_robots > max_ore_bots_needed
        || current.clay_robots > max_clay_bots_needed
        || current.obsidian_robots > max_obs_bots_needed
        {
            continue;
        }
        
        current.time += 1;
        if current.time == TOTAL_TIME {

            current.ore += current.ore_robots;
            current.clay += current.clay_robots;
            current.obsidian += current.obsidian_robots;
            current.geode += current.geode_robots;

            if let Some(best_state) = &best_state_opt {
                //if current.ore_robots == 1 && current.clay_robots == 3 && current.obsidian_robots == 1 {
                    //if current.obsidian_robots > best_state.obsidian_robots {
                        //if current.clay_robots + current.ore_robots > best_state.clay_robots + best_state.ore_robots {
                if current.geode > best_state.geode {
                    println!("best state on {counter}: {g}", g = current.geode);
                    best_state_opt = Some(current);
                }
            } else {
                println!("first best state on {counter}: {g}", g = current.geode);
                best_state_opt = Some(current);
            }
            
            continue;
        }

        let mut new_states = Vec::new();

        // make ore robot
        if 
        current.ore_robots < max_ore_bots_needed 
        && 
        current.ore >= blueprint.ore_robot_ore_cost 
        {
            let mut ore_robot_choice = current.clone();
            ore_robot_choice.ore -= blueprint.ore_robot_ore_cost;
            ore_robot_choice.ore_robots += 1;
            new_states.push(ore_robot_choice);
        }

        // make clay robot
        if 
        current.clay_robots < max_clay_bots_needed 
        &&
        current.ore >= blueprint.clay_robot_ore_cost
         {
            let mut clay_robot_choice = current.clone();
            clay_robot_choice.ore -= blueprint.clay_robot_ore_cost;
            clay_robot_choice.clay_robots += 1;
            new_states.push(clay_robot_choice);
        }

        // make obsidian robot
        if 
        current.obsidian_robots < max_obs_bots_needed
        && current.ore >= blueprint.obsidian_robot_ore_cost
            && current.clay >= blueprint.obsidian_robot_clay_cost
            
        {
            let mut obsidian_robot_choice = current.clone();
            obsidian_robot_choice.ore -= blueprint.obsidian_robot_ore_cost;
            obsidian_robot_choice.clay -= blueprint.obsidian_robot_clay_cost;
            obsidian_robot_choice.obsidian_robots += 1;
            new_states.push(obsidian_robot_choice);
        }

        // make geode robot
        if current.ore >= blueprint.geode_robot_ore_cost
            && current.obsidian >= blueprint.geode_robot_obsidian_cost
        {
            let mut geode_robot_choice = current.clone();
            geode_robot_choice.ore -= blueprint.geode_robot_ore_cost;
            geode_robot_choice.obsidian -= blueprint.geode_robot_obsidian_cost;
            geode_robot_choice.geode_robots += 1;



            // eurgh


            geode_robot_choice.ore += current.ore_robots;
            geode_robot_choice.clay += current.clay_robots;
            geode_robot_choice.obsidian += current.obsidian_robots;
            geode_robot_choice.geode += current.geode_robots;
            stack.push_front(geode_robot_choice);
            continue;


            //new_states.push(geode_robot_choice);
        }

        let new_state_count = new_states.len();
        //new_states.reverse();
        for mut s in new_states {
            s.ore += current.ore_robots;
            s.clay += current.clay_robots;
            s.obsidian += current.obsidian_robots;
            s.geode += current.geode_robots;

            stack.push_front(s);
        }

        if new_state_count != 3 {
            current.ore += current.ore_robots;
            current.clay += current.clay_robots;
            current.obsidian += current.obsidian_robots;
            current.geode += current.geode_robots;

            // do nothing
            stack.push_back(current);
        }
    }

    println!("Blueprint {bid} checked {s} states. Max geode culls: {max_geode_culls}.",bid = blueprint.id, s= counter);

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
    let result = calculate_quality_levels(blueprints);
    Ok(result)
}

pub fn day_19_part_2() -> io::Result<i32> {
    todo!();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn max_geode_test() {
        let state = State {

            time : 10,
            ..Default::default()
        };
        let max = state.max_geodes(10);
    assert_eq!(0, max);
    }

    #[test]
    fn single_blueprint_test() {
        let blueprints = load_blueprints("./inputs/day-19-input-test.txt").unwrap();
        let best_state = simulate_blueprint(&blueprints[0]);

        println!("=== {t} minutes ===", t = best_state.time);
        best_state.message();

        //println!("{best_state:?}");
        assert_eq!(best_state.quality_level(), 9);
    }

    #[test]
    fn small_test() {
        let blueprints = load_blueprints("./inputs/day-19-input-test.txt").unwrap();
        let result = calculate_quality_levels(blueprints);
        assert_eq!(result, 33);
    }

    #[test]
    fn test() {
        let blueprints = load_blueprints("./inputs/day-19-input.txt").unwrap();
        let result = calculate_quality_levels(blueprints);
        assert_eq!(result, 1349);
    }
}
