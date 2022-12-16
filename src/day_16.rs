use std::io;

use itertools::Itertools;

use crate::file_utils::read_lines;

type NodeList = Vec<ValveNode>;

trait NodeListExt {
    fn fetch_index_by_name(&self, name: &str) -> Option<usize>;

    fn fetch_or_create_index_by_name(&mut self, name: &str) -> usize;

    fn fetch_connected_name(&self, index: usize) -> Vec<String>;

    fn fetch_names(&self, indexes: &Vec<usize>) -> Vec<String>;

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

    fn fetch_names(&self, indexes: &Vec<usize>) -> Vec<String> {
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

        return visited[destination_index];
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

        return visited;
    }
}

struct ValveNode {
    pub name: String,
    pub connected_indexes: Vec<usize>,
    pub flow_rate: i32,
    pub used: bool,
}

impl ValveNode {
    pub fn with_name(name: String) -> ValveNode {
        ValveNode {
            name,
            connected_indexes: Vec::new(),
            flow_rate: 0,
            used: false,
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
                .map(|c| node_list.fetch_or_create_index_by_name(c.clone()))
                .collect_vec();
            node_list[index].connected_indexes.extend(indexes);
        }
    }
    Ok(node_list)
}

fn open_valves(node_list: &mut NodeList) -> i32 {
    let mut current_node_index = 0;
    let mut remaining_minutes = 30;
    let mut pressure = 0;


    let mut debug = Vec::new();

    loop {
        let name = &node_list[current_node_index].name.clone();
        println!("At location {name}. Released pressure is {pressure}. Remaining time is {remaining_minutes}.");

        let mut indexed_flows = Vec::new();
        for (i, node) in node_list.iter().enumerate() {
            if !node.used {
                indexed_flows.push((i, node.flow_rate));
            }
        }

        if indexed_flows.len() == 0 {
            break;
        }

        indexed_flows.sort_by(|a, b| b.1.cmp(&a.1));
        let distances = node_list.fetch_all_journey_length(current_node_index);

        let mut scores: Vec<(usize, f64)> = Vec::new();
        for (index, flow_rate) in &indexed_flows {
            let distance = distances[*index];

            if distance + 1 > remaining_minutes || distance == 0 || *flow_rate == 0 {
                continue;
            }

            // idfk
            //let score = flow_rate as f64 / distance as f64;

            // dead end penalty
            let penalty = if node_list[*index].connected_indexes.len() == 1 {
                0.80
            } else {
                1.0
            };

            let adj_minutes = remaining_minutes - (distance + 1);

            let score = (adj_minutes * flow_rate) as f64 * penalty;
            //let score = (adj_minutes * flow_rate) as f64 - ((distance * distance) as f64 * 0.9);

            //let score = ( ( (remaining_minutes - (distance + 1)) * flow_rate )  + 1000 *  node_list[index].connected_indexes.len() as i32  )  as f64;

            // and find score of next closest valve
            let mut second_scores = Vec::new();
            let second_distances = node_list.fetch_all_journey_length(*index);
            for (second_index, second_flow_rate) in &indexed_flows {
                if second_index == index {
                    continue;
                }
                let second_score =
                    (adj_minutes - (second_distances[*second_index] + 1)) * second_flow_rate;

                second_scores.push((second_index, second_score));
            }

            second_scores.sort_by(|(_, a_score), (_, b_score)| b_score.cmp(&a_score));

            //let second_thing = 0.0;
            let second_thing = second_scores[0].1 as f64;
            //let second_thing = (second_scores.iter().take(2).map(|ss| ss.1).sum::<i32>()) as f64;
            //let second_thing = ((second_scores.iter().take(2).map(|ss| ss.1).sum::<i32>()) as f64) / second_scores.len() as f64;

            let total_score = score + second_thing;
            let curent_name = &node_list[*index].name.clone();
            println!(
                "    Score summary for {curent_name} -> {score} + {second_thing} = {total_score}"
            );
            for (i, score) in &second_scores {
                let score_name = &node_list[**i].name.clone();
                println!("        Secondary score {score_name}: {score}");
            }

            scores.push((*index, total_score));
        }

        if scores.len() == 0 {
            break;
        }

        scores.sort_by(|(_, a_score), (_, b_score)| b_score.total_cmp(&a_score));

        for (i, score) in &scores {
            let score_name = &node_list[*i].name.clone();
            println!("{score_name}: {score}");
        }

        let (travel_index, _) = scores[0];

        debug.push(format!("{name} pressure: {pressure} time: {remaining_minutes}"));
        // extra minute to open valve
        remaining_minutes -= distances[travel_index] + 1;
        node_list[travel_index].used = true;

        
        pressure += node_list[travel_index].flow_rate * remaining_minutes;
        current_node_index = travel_index;
    }
    let name = &node_list[current_node_index].name.clone();
    debug.push(format!("{name} pressure: {pressure} time: {remaining_minutes}"));
        

    for d in debug {
        println!("{d}");
    }

    pressure
}

pub fn day_16() -> io::Result<i32> {
    let mut node_list = build_nodes("./inputs/day-16-input.txt").unwrap();

    let result = open_valves(&mut node_list);
    Ok(result)
}

pub fn day_16_part_2() -> io::Result<i32> {
    todo!();
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
}
