use std::{
    collections::{HashMap, HashSet},
    io,
};

use itertools::Itertools;

use crate::file_utils::read_lines;

type NodeList = Vec<ValveNode>;

trait NodeListExt {
    fn fetch_index_by_name(&self, name: &str) -> Option<usize>;

    fn fetch_or_create_index_by_name(&mut self, name: &str) -> usize;

    fn fetch_connected_name(&self, index: usize) -> Vec<String>;

    fn fetch_names(&self, indexes: &[usize]) -> Vec<String>;

    fn fetch_journey_length(&self, current_index: usize, destination_index: usize) -> i32;

    fn fetch_all_journey_length(&self, current_index: usize) -> Vec<i32>;
}

impl NodeListExt for NodeList {
    fn fetch_index_by_name(&self, name: &str) -> Option<usize> {
        for (i, node) in self.iter().enumerate() {
            if node.name == name {
                return Some(i);
            }
        }
        None
    }

    fn fetch_or_create_index_by_name(&mut self, name: &str) -> usize {
        let trimmed = name.trim();
        if let Some(i) = self.fetch_index_by_name(trimmed) {
            i
        } else {
            let i = self.len();
            self.push(ValveNode::with_name(trimmed.to_string()));
            i
        }
    }

    fn fetch_connected_name(&self, index: usize) -> Vec<String> {
        let mut names = Vec::new();
        for i in &self[index].connected_indexes {
            names.push(self[*i].name.clone());
        }
        names
    }

    fn fetch_names(&self, indexes: &[usize]) -> Vec<String> {
        let mut names = Vec::new();
        for i in indexes {
            names.push(self[*i].name.clone());
        }
        names
    }

    fn fetch_journey_length(&self, current_index: usize, destination_index: usize) -> i32 {
        let mut visited = vec![-1; self.len()];
        visited[current_index] = 0;

        let mut indexes_to_check = Vec::new();
        for c in &self[current_index].connected_indexes {
            visited[*c] = 1;
            indexes_to_check.push(*c);
        }
        let mut steps = 2;
        while visited[destination_index] == -1 || !indexes_to_check.is_empty() {
            let mut new_indexes: Vec<usize> = Vec::new();
            for i in indexes_to_check {
                for c in &self[i].connected_indexes {
                    if visited[*c] == -1 {
                        visited[*c] = steps;
                        new_indexes.push(*c);
                    }
                }
            }
            steps += 1;
            indexes_to_check = new_indexes;
        }

        visited[destination_index]
    }

    fn fetch_all_journey_length(&self, current_index: usize) -> Vec<i32> {
        let mut visited = vec![-1; self.len()];
        visited[current_index] = 0;

        let mut indexes_to_check = Vec::new();
        for c in &self[current_index].connected_indexes {
            visited[*c] = 1;
            indexes_to_check.push(*c);
        }
        let mut steps = 2;
        while !indexes_to_check.is_empty() {
            let mut new_indexes: Vec<usize> = Vec::new();
            for i in indexes_to_check {
                for c in &self[i].connected_indexes {
                    if visited[*c] == -1 {
                        visited[*c] = steps;
                        new_indexes.push(*c);
                    }
                }
            }
            steps += 1;
            indexes_to_check = new_indexes;
        }

        visited
    }
}

struct ValveNode {
    pub name: String,
    pub connected_indexes: Vec<usize>,
    pub flow_rate: i32,
}

impl ValveNode {
    pub fn with_name(name: String) -> ValveNode {
        ValveNode {
            name,
            connected_indexes: Vec::new(),
            flow_rate: 0,
        }
    }
}

fn build_nodes(filename: &str) -> io::Result<NodeList> {
    let mut node_list = NodeList::new();

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        // yup.
        let fixed = line
            .replace("tunnels", "tunnel")
            .replace("valves", "valve")
            .replace("leads", "lead");

        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        let parsed = sscanf::sscanf!(
            fixed,
            "Valve {} has flow rate={}; tunnel lead to valve {}",
            String,
            i32,
            String
        );
        if let Ok((name, flow_rate, connected)) = parsed {
            let index = node_list.fetch_or_create_index_by_name(name.as_str());

            node_list[index].flow_rate = flow_rate;
            let indexes = connected
                .split(',')
                .map(|c| node_list.fetch_or_create_index_by_name(c))
                .collect_vec();
            node_list[index].connected_indexes.extend(indexes);
        }
    }
    Ok(node_list)
}

#[derive(Clone)]
struct JourneyResult {
    pub pressure_released: i32,

    pub valves_with_time: Vec<(String, i32)>,

    pub time_remaining: i32,

    pub visited: HashSet<usize>,

    pub current_index: usize,
}

fn journey_recurse(
    journey_lengths: &Vec<Vec<i32>>,
    node_list: &NodeList,
    journey_result: JourneyResult,
) -> Vec<JourneyResult> {
    let mut results = Vec::new();

    for (i, node) in node_list.iter().enumerate() {
        if node.flow_rate > 0 && !journey_result.visited.contains(&i) {
            let distance = journey_lengths[journey_result.current_index][i];

            let updated_time = journey_result.time_remaining - (distance + 1);
            if updated_time < 0 {
                continue;
            }

            let mut sub_journey_result = journey_result.clone();

            sub_journey_result.time_remaining = updated_time;
            sub_journey_result.pressure_released +=
                sub_journey_result.time_remaining * node_list[i].flow_rate;
            sub_journey_result
                .valves_with_time
                .push((node_list[i].name.clone(), sub_journey_result.time_remaining));
            sub_journey_result.visited.insert(i);
            sub_journey_result.current_index = i;

            let result = journey_recurse(journey_lengths, node_list, sub_journey_result);
            results.extend(result);
        }
    }

    results.push(journey_result);
    results
}

fn open_valves(node_list: &mut NodeList) -> i32 {
    let start_node_name = "AA";
    let start_node_index = node_list.fetch_index_by_name("AA").unwrap();
    let remaining_minutes = 30;

    let mut visited = HashSet::new();
    visited.insert(start_node_index);
    let mut journey_lengths = Vec::new();
    for (i, node) in node_list.iter().enumerate() {
        let lengths = node_list.fetch_all_journey_length(i);
        journey_lengths.push(lengths);

        if node.flow_rate == 0 {
            visited.insert(i);
        }
    }

    let journey_result = JourneyResult {
        pressure_released: 0,
        valves_with_time: vec![(start_node_name.to_string(), 30)],
        time_remaining: remaining_minutes,
        visited,
        current_index: start_node_index,
    };
    let mut results = journey_recurse(&journey_lengths, node_list, journey_result);

    results.sort_by(|a, b| b.pressure_released.cmp(&a.pressure_released));

    results[0].pressure_released
}

fn open_valves_with_two(node_list: &mut NodeList) -> i32 {
    let start_node_name = "AA";
    let start_node_index = node_list.fetch_index_by_name("AA").unwrap();
    let remaining_minutes = 26;

    let mut journey_lengths = Vec::new();
    for (i, _) in node_list.iter().enumerate() {
        let lengths = node_list.fetch_all_journey_length(i);
        journey_lengths.push(lengths);
    }

    let journey_result = JourneyResult {
        pressure_released: 0,
        valves_with_time: vec![(start_node_name.to_string(), 30)],
        time_remaining: remaining_minutes,
        visited: HashSet::new(),
        current_index: start_node_index,
    };
    let results = journey_recurse(&journey_lengths, node_list, journey_result);
    let mut deduped = HashMap::new();
    for r in results {
        let key = r.visited.iter().map(|v| 1_usize << v).sum::<usize>();

        let pressure = i32::max(
            *deduped.get(&key).unwrap_or(&r.pressure_released),
            r.pressure_released,
        );
        deduped.insert(key, pressure);
    }

    let mut not_overlapping_results = Vec::new();
    for (k1, v1) in &deduped {
        for (k2, v2) in &deduped {
            if k1 & k2 == 0 {
                not_overlapping_results.push(v1 + v2);
            }
        }
    }

    not_overlapping_results.sort();
    not_overlapping_results.reverse();

    not_overlapping_results[0]
}

pub fn day_16() -> io::Result<i32> {
    let mut node_list = build_nodes("./inputs/day-16-input.txt").unwrap();

    let result = open_valves(&mut node_list);
    Ok(result)
}

pub fn day_16_part_2() -> io::Result<i32> {
    let mut node_list = build_nodes("./inputs/day-16-input.txt").unwrap();

    let result = open_valves_with_two(&mut node_list);
    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn build_nodes_test() {
        let node_list = build_nodes("./inputs/day-16-input-test.txt").unwrap();

        for node in &node_list {
            println!(
                "{n}: flow: {f} children: {c:?}",
                n = node.name,
                f = node.flow_rate,
                c = node_list.fetch_names(&node.connected_indexes)
            );
        }
        assert_eq!(3, node_list[0].connected_indexes.len());
    }

    #[test]
    fn fetch_journey_length_test() {
        let node_list = build_nodes("./inputs/day-16-input-test.txt").unwrap();

        let aa = node_list.fetch_index_by_name("AA").unwrap();
        let hh = node_list.fetch_index_by_name("HH").unwrap();

        assert_eq!(5, node_list.fetch_journey_length(aa, hh));
    }

    #[test]
    fn small_test() {
        let mut node_list = build_nodes("./inputs/day-16-input-test.txt").unwrap();

        let result = open_valves(&mut node_list);
        assert_eq!(result, 1651);
    }

    #[test]
    fn test() {
        let mut node_list = build_nodes("./inputs/day-16-input.txt").unwrap();

        let result = open_valves(&mut node_list);
        assert_eq!(result, 2059);
    }

    #[test]
    fn part_2_small_test() {
        let mut node_list = build_nodes("./inputs/day-16-input-test.txt").unwrap();

        let result = open_valves_with_two(&mut node_list);
        assert_eq!(result, 1707);
    }

    #[test]
    fn part_2_test() {
        let mut node_list = build_nodes("./inputs/day-16-input.txt").unwrap();

        let result = open_valves_with_two(&mut node_list);
        assert_eq!(result, 2790);
    }
}
