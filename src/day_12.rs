use std::{collections::VecDeque, io};

use crate::file_utils::read_lines;

struct Map {
    cells: Vec<Vec<i32>>,

    start: (i32, i32),
    end: (i32, i32),
}

impl Map {
    pub fn new() -> Map {
        Map {
            cells: Vec::new(),
            start: (0, 0),
            end: (0, 0),
        }
    }

    pub fn add_row(&mut self, row: Vec<i32>) {
        self.cells.push(row);
    }

    pub fn fetch_cell(&self, x: i32, y: i32) -> Option<i32> {
        let mut result = None;

        if y >= 0 && x >= 0 {
            let x_s = x as usize;
            let y_s = y as usize;
            if y_s < self.cells.len() && x_s < self.cells[y_s].len() {
                let cell = self.cells[y_s][x_s];
                result = Some(cell);
            }
        }
        result
    }

    pub fn width(&self) -> i32 {
        self.cells[0].len() as i32
    }

    pub fn height(&self) -> i32 {
        self.cells.len() as i32
    }
}

enum Direction {
    North,
    East,
    South,
    West,
    None,
}

struct MapTree {
    pub nodes: Vec<MapTreeNode>,
}

impl MapTree {
    pub fn new() -> MapTree {
        MapTree { nodes: Vec::new() }
    }

    /**
     * Creates a root, returning its index.
     */
    pub fn create_root(&mut self, x: i32, y: i32) -> usize {
        self.nodes.push(MapTreeNode {
            parent_index: None,
            children_index: Vec::new(),
            direction: Direction::None,
            x,
            y,
            step_count: 0,
        });
        self.nodes.len() - 1
    }

    pub fn create_node(
        &mut self,
        x: i32,
        y: i32,
        parent_index: usize,
        direction: Direction,
    ) -> usize {
        self.nodes.push(MapTreeNode {
            parent_index: Some(parent_index),
            children_index: Vec::new(),
            direction: direction,
            x,
            y,
            step_count: self.nodes[parent_index].step_count + 1,
        });
        let index = self.nodes.len() - 1;
        self.nodes[parent_index].children_index.push(index);
        index
    }

    pub fn fetch_node_coords(&mut self, index: usize) -> (i32, i32) {
        (self.nodes[index].x, self.nodes[index].y)
    }

    pub fn location_visited(&self, x: i32, y: i32) -> bool {
        // I believe there's a chance a short-cut could be found that this function
        // would disallow, but that hasn't happened in the test cases so *shrug*. 
        for node in &self.nodes {
            if node.x == x && node.y == y {
                return true;
            }
        }
        return false;
    }
}

struct MapTreeNode {
    pub parent_index: Option<usize>,
    pub children_index: Vec<usize>,
    /**
     * Direction of travel that brought traveller to this node.
     */
    pub direction: Direction,
    pub x: i32,
    pub y: i32,
    pub step_count: i32,
}

pub fn day_12() -> io::Result<i32> {
    let positions = climb_sim("./inputs/day-12-input.txt")?;
    Ok(positions)
}

pub fn day_12_part_2() -> io::Result<i64> {
    todo!();
}

fn build_map(filename: &str) -> io::Result<Map> {
    let mut map = Map::new();

    let lines = read_lines(filename)?;
    let mut y = 0;
    let mut start_opt = None;
    let mut end_opt = None;
    for line in lines.flatten() {
        let mut x = 0;
        if line.is_empty() {
            continue;
        }

        let mut row = Vec::new();
        for c in line.chars() {
            let elevation = match c {
                'S' => {
                    start_opt = Some((x, y));
                    fetch_elevation('a')
                }
                'E' => {
                    end_opt = Some((x, y));
                    fetch_elevation('z')
                }
                _ => fetch_elevation(c),
            };
            row.push(elevation);
            x += 1;
        }
        map.add_row(row);
        y += 1;
    }
    if let Some(start) = start_opt {
        map.start = start;
    } else {
        panic!("Start position was not found.");
    }
    if let Some(end) = end_opt {
        map.end = end;
    } else {
        panic!("End position was not found.");
    }
    Ok(map)
}

fn climb_sim(filename: &str) -> io::Result<i32> {
    let jump_height = 1;

    let map = build_map(filename)?;
    let mut map_tree = MapTree::new();

    let current_node_index = map_tree.create_root(map.start.0, map.start.1);

    let mut check_stack = VecDeque::new();
    check_stack.push_front(current_node_index);

    while check_stack.len() > 0 {
        let index = check_stack.pop_front().unwrap();

        let (current_x, current_y) = map_tree.fetch_node_coords(index);
        let current_elevation = map.fetch_cell(current_x, current_y).unwrap();

        // north
        if let Some(left_index) = check_cell(
            &map,
            &mut map_tree,
            jump_height,
            index,
            current_elevation,
            current_x,
            current_y,
            Direction::North,
        ) {
            check_stack.push_back(left_index);
        }

        // east
        if let Some(east_index) = check_cell(
            &map,
            &mut map_tree,
            jump_height,
            index,
            current_elevation,
            current_x,
            current_y,
            Direction::East,
        ) {
            check_stack.push_back(east_index);
        }

        // south
        if let Some(south_index) = check_cell(
            &map,
            &mut map_tree,
            jump_height,
            index,
            current_elevation,
            current_x,
            current_y,
            Direction::South,
        ) {
            check_stack.push_back(south_index);
        }

        // west
        if let Some(west_index) = check_cell(
            &map,
            &mut map_tree,
            jump_height,
            index,
            current_elevation,
            current_x,
            current_y,
            Direction::West,
        ) {
            check_stack.push_back(west_index);
        }
    }

    let mut results = Vec::new();
    for i in 0..map_tree.nodes.len() {
        if map_tree.nodes[i].x == map.end.0 && map_tree.nodes[i].y == map.end.1 {
            results.push((i, map_tree.nodes[i].step_count));
        }
    }

    if results.len() == 0 {
        panic!("No route found");
    }

    results.sort_by(|(_, a), (_, b)| a.cmp(b));

    let result = results[0];

    let mut display = Vec::new();
    let mut display_index = result.0;
    loop {
        if let Some(p_r) = map_tree.nodes[display_index].parent_index {
            let c = match map_tree.nodes[display_index].direction {
                Direction::North => '^',
                Direction::East => '>',
                Direction::South => 'v',
                Direction::West => '<',
                Direction::None => '?',
            };
            display.push((map_tree.nodes[p_r].x, map_tree.nodes[p_r].y, c));
            display_index = p_r;
        } else {
            break;
        }
    }

    //print_journey(map, display);

    Ok(map_tree.nodes[result.0].step_count)
}

#[allow(dead_code)]
fn print_journey(map: Map, display: Vec<(i32, i32, char)>) {
    for y in 0..map.height() {
        for x in 0..map.width() {
            if x == map.start.0 && y == map.start.1 {
                print!("S");
            } else if x == map.end.0 && y == map.end.1 {
                print!("E");
            } else {
                let mut to_print = '.';
                for (dx, dy, c) in &display {
                    if y == *dy && x == *dx {
                        to_print = *c;
                        break;
                    }
                }
                print!("{to_print}");
            }
        }
        println!("");
    }
}

fn check_cell(
    map: &Map,
    map_tree: &mut MapTree,
    jump_height: i32,
    index: usize,
    current_elevation: i32,
    current_x: i32,
    current_y: i32,
    direction: Direction,
) -> Option<usize> {
    let new_x = match direction {
        Direction::East => current_x + 1,
        Direction::West => current_x - 1,
        _ => current_x,
    };
    let new_y = match direction {
        Direction::North => current_y - 1,
        Direction::South => current_y + 1,
        _ => current_y,
    };
    if let Some(elevation) = map.fetch_cell(new_x, new_y) {
        if jump_height + current_elevation >= elevation
            && !map_tree.location_visited(new_x, new_y)
        {
            let new_index = map_tree.create_node(new_x, new_y, index, direction);
            return Some(new_index);
        }
    }
    None
}

fn fetch_elevation(c: char) -> i32 {
    ((c as u8) - ('a' as u8)) as i32
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn fetch_elevation_test() {
        assert_eq!(0, fetch_elevation('a'));
        assert_eq!(25, fetch_elevation('z'));
    }

    #[test]
    fn small_test() {
        assert_eq!(climb_sim("./inputs/day-12-input-test.txt").unwrap(), 31);
    }

    #[test]
    fn test() {
        assert_eq!(climb_sim("./inputs/day-12-input.txt").unwrap(), 380);
    }
}
