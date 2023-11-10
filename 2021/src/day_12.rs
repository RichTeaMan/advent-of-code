use std::{
    collections::{HashMap, VecDeque},
    io::{self},
    rc::Rc,
};

use itertools::Itertools;

use utils::file_utils::read_lines;

const START: &str = "start";
const END: &str = "end";

struct Node {
    name: String,
    parent: Option<Rc<Node>>,
    repeat: bool,
}

fn has_visited(node: Rc<Node>, name: &String) -> bool {
    if node.name == *name {
        return true;
    }
    if let Some(parent) = node.parent.clone() {
        return has_visited(parent, name);
    }
    false
}

fn fetch_has_repeated(node: Rc<Node>) -> bool {
    if node.repeat {
        return true;
    }
    if let Some(parent) = node.parent.clone() {
        return fetch_has_repeated(parent);
    }
    false
}

pub fn day_12() -> io::Result<i32> {
    find_paths("./inputs/day-12-input.txt")
}
pub fn day_12_part_2() -> io::Result<i32> {
    find_paths_with_repeat("./inputs/day-12-input.txt")
}

fn is_small(name: String) -> bool {
    name.chars().next().unwrap().is_lowercase()
}

fn load_map(filename: &str) -> io::Result<HashMap<String, Vec<String>>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        let (a, b) = line.split('-').map(|s| s.trim()).collect_tuple().unwrap();

        if b != START {
            if let Some(a_vec) = map.get_mut(a) {
                a_vec.push(b.to_string());
            } else {
                map.insert(a.to_string(), vec![b.to_string()]);
            }
        }

        if a != START {
            if let Some(b_vec) = map.get_mut(b) {
                b_vec.push(a.to_string());
            } else {
                map.insert(b.to_string(), vec![a.to_string()]);
            }
        }
    }
    Ok(map)
}

fn find_paths(filename: &str) -> io::Result<i32> {
    let map = load_map(filename)?;
    let mut current = VecDeque::new();
    let mut finished = Vec::new();

    let start = Node {
        name: START.to_string(),
        parent: None,
        repeat: false,
    };

    current.push_back(Rc::new(start));

    while let Some(node) = current.pop_front() {
        if node.name == *END {
            finished.push(node);
            continue;
        }

        let children = map.get(&node.name).unwrap();
        for child in children {
            // check if small cave has already been visited
            if is_small(child.to_string()) && has_visited(node.clone(), child) {
                continue;
            }

            let child_node = Node {
                name: child.to_string(),
                parent: Some(node.clone()),
                repeat: false,
            };
            current.push_back(Rc::new(child_node));
        }
    }
    Ok(finished.len() as i32)
}

fn find_paths_with_repeat(filename: &str) -> io::Result<i32> {
    let map = load_map(filename)?;
    let mut current = VecDeque::new();
    let mut finished = Vec::new();

    let start = Node {
        name: START.to_string(),
        parent: None,
        repeat: false,
    };

    current.push_back(Rc::new(start));

    while let Some(node) = current.pop_front() {
        if node.name == *END {
            finished.push(node);
            continue;
        }

        let children = map.get(&node.name).unwrap();
        let has_repeated = fetch_has_repeated(node.clone());
        for child in children {
            let mut repeat = false;
            // check if small cave has already been visited
            if is_small(child.to_string()) && has_visited(node.clone(), child) {
                // check if the small cave has already been visited twice
                if has_repeated {
                    continue;
                }
                repeat = true;
            }

            let child_node = Node {
                name: child.to_string(),
                parent: Some(node.clone()),
                repeat,
            };
            current.push_back(Rc::new(child_node));
        }
    }
    Ok(finished.len() as i32)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(find_paths("./inputs/day-12-input-test.txt").unwrap(), 226);
    }

    #[test]
    fn test() {
        assert_eq!(find_paths("./inputs/day-12-input.txt").unwrap(), 5228);
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            find_paths_with_repeat("./inputs/day-12-input-test.txt").unwrap(),
            3509
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            find_paths_with_repeat("./inputs/day-12-input.txt").unwrap(),
            131228
        );
    }
}
