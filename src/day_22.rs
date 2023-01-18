use std::{collections::HashMap, io};

use crate::file_utils::read_lines;

type Map = HashMap<i32, HashMap<i32, MapSection>>;

const NORTH_INDEX: usize = 0;
const EAST_INDEX: usize = 1;
const SOUTH_INDEX: usize = 2;
const WEST_INDEX: usize = 3;

pub fn day_22() -> io::Result<i32> {
    let result = map_puzzle("./inputs/day-22-input.txt")?;
    Ok(result)
}

pub fn day_22_part_2() -> io::Result<i32> {
    todo!();
}

enum MapSection {
    WALL,
    FLOOR,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    LEFT,
    RIGHT,
    NONE,
}

#[derive(Debug)]
enum Facing {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Orientation {
    SAME,
    ONE_CLOCKWISE,
    TWO_CLOCKWISE,
    THREE_CLOCKWISE,
}

impl Orientation {
    fn combine(self, a: &Orientation) -> Orientation {
        let combined = (self.orientation_as_number() + a.orientation_as_number()) % 4;

        match combined {
            0 => Orientation::SAME,
            1 => Orientation::ONE_CLOCKWISE,
            2 => Orientation::TWO_CLOCKWISE,
            3 => Orientation::THREE_CLOCKWISE,
            _ => panic!("Bad orientation number: {combined}"),
        }
    }

    fn orientation_as_number(&self) -> usize {
        match self {
            Orientation::SAME => 0,
            Orientation::ONE_CLOCKWISE => 1,
            Orientation::TWO_CLOCKWISE => 2,
            Orientation::THREE_CLOCKWISE => 3,
            _ => panic!("Unknown orientation"),
        }
    }

    fn resolve(&self, direction: usize) -> usize {
        (self.orientation_as_number() + direction) % 4
    }
}

fn opposite_direction(direction: usize) -> usize {
    (direction + 2) % 4
}

impl Facing {
    fn rotate_left(&self) -> Facing {
        match self {
            Facing::NORTH => Facing::WEST,
            Facing::EAST => Facing::NORTH,
            Facing::SOUTH => Facing::EAST,
            Facing::WEST => Facing::SOUTH,
        }
    }

    fn rotate_right(&self) -> Facing {
        match self {
            Facing::NORTH => Facing::EAST,
            Facing::EAST => Facing::SOUTH,
            Facing::SOUTH => Facing::WEST,
            Facing::WEST => Facing::NORTH,
        }
    }

    fn fetch_digit(&self) -> i32 {
        match self {
            Facing::NORTH => 3,
            Facing::EAST => 0,
            Facing::SOUTH => 1,
            Facing::WEST => 2,
        }
    }
}

struct Instruction {
    pub steps: i32,
    pub direction: Direction,
}

#[derive(Debug)]
struct MapGraph {
    size: i32,

    faces: Vec<CubeFace>,
}

impl MapGraph {
    fn connection_count(&self) -> usize {
        let count = self.faces.iter().map(|f| f.connection_count()).sum();
        count
    }
}

#[derive(Debug)]
struct CubeFace {
    id: usize,
    x: i32,
    y: i32,

    connections: [Option<CubeFaceConnection>; 4],
}

impl CubeFace {
    fn new(id: usize, x: i32, y: i32) -> CubeFace {
        CubeFace {
            id,
            x,
            y,
            connections: [None, None, None, None],
        }
    }

    fn connection_count(&self) -> usize {
        self.connections.iter().filter(|c| c.is_some()).count()
    }
}

#[derive(Clone, Copy, Debug)]
struct CubeFaceConnection {
    cube_face_id: usize,

    orientation: Orientation,
}

fn normal_direction(direction: usize) -> (usize, usize) {
    match direction % 4 {
        NORTH_INDEX | SOUTH_INDEX => (WEST_INDEX, EAST_INDEX),
        EAST_INDEX | WEST_INDEX => (NORTH_INDEX, SOUTH_INDEX),
        _ => panic!("Unknown direction: {direction}"),
    }
}

fn load_map(filename: &str) -> io::Result<(Map, Vec<Instruction>)> {
    let mut map: Map = HashMap::new();
    let mut instructions = Vec::new();
    let lines = read_lines(filename)?;
    for (y, line) in lines.flatten().enumerate() {
        if line.is_empty() {
            continue;
        }

        if line.contains("L") || line.contains("R") {
            let directions = line.split_inclusive(&['L', 'R']);

            for direction in directions {
                let steps_str = direction.trim_end_matches(&['L', 'R']);
                let steps = steps_str.parse::<i32>().unwrap();
                instructions.push(Instruction {
                    steps,
                    direction: if direction.contains('L') {
                        Direction::LEFT
                    } else if direction.contains('R') {
                        Direction::RIGHT
                    } else {
                        Direction::NONE
                    },
                });
            }
            continue;
        }

        let mut map_line = HashMap::new();
        for (x, c) in line.chars().enumerate() {
            let section_opt = match c {
                ' ' => None,
                '.' => Some(MapSection::FLOOR),
                '#' => Some(MapSection::WALL),
                other => panic!("Unknown input {other}"),
            };
            if let Some(section) = section_opt {
                map_line.insert(x as i32, section);
            }
        }
        map.insert(y as i32, map_line);
    }
    Ok((map, instructions))
}

fn calc_3d_map_size(map: &Map) -> i32 {
    // calculate area of a single face
    let area = map.iter().map(|(_, line)| line.len()).sum::<usize>() as i32 / 6;

    // integer square root we have at home
    for i in 1..=4096 {
        if i * i == area {
            return i;
        }
    }
    panic!("Unable to find a integer square area for map.");
}

fn build_graph(map: &Map, size: i32) -> MapGraph {
    // construct faces
    let mut faces: Vec<CubeFace> = Vec::new();
    for face_row in 0..6 {
        for face_column in 0..6 {
            let y = face_row * size;
            let x = face_column * size;

            if fetch_tile(map, x, y).is_some() {
                // check if another face already touches this one
                // a face can only possibly be left or up
                let mut up_face_id_opt = None;
                let mut left_face_id_opt = None;
                for face in &faces {
                    if face.x == x && face.y == (y - size) {
                        up_face_id_opt = Some(face.id);
                    } else if face.x == (x - size) && face.y == y {
                        left_face_id_opt = Some(face.id);
                    }
                }
                let mut face = CubeFace::new(faces.len(), x, y);
                if let Some(up_face_id) = up_face_id_opt {
                    let connection = CubeFaceConnection {
                        cube_face_id: up_face_id,
                        orientation: Orientation::SAME,
                    };
                    face.connections[NORTH_INDEX] = Some(connection);

                    let partner_connection = CubeFaceConnection {
                        cube_face_id: face.id,
                        orientation: Orientation::SAME,
                    };
                    faces[up_face_id].connections[SOUTH_INDEX] = Some(partner_connection);
                }

                if let Some(left_face_id) = left_face_id_opt {
                    let connection = CubeFaceConnection {
                        cube_face_id: left_face_id,
                        orientation: Orientation::SAME,
                    };
                    face.connections[WEST_INDEX] = Some(connection);

                    let partner_connection = CubeFaceConnection {
                        cube_face_id: face.id,
                        orientation: Orientation::SAME,
                    };
                    faces[left_face_id].connections[EAST_INDEX] = Some(partner_connection);
                }

                faces.push(face);
            }
        }
    }

    let mut map_graph = MapGraph { faces, size };

    // cubes always have 12 edges. we have two edge structs per actual edge
    while map_graph.connection_count() < 12 * 2 {
        let connection_count = map_graph.connection_count();
        for face_id in 0..map_graph.faces.len() {
            debug_assert!(map_graph.faces[face_id].connection_count() > 0);

            if map_graph.faces[face_id].connection_count() == 4 {
                continue;
            }
            let mut face_resolved = false;
            for direction_index in 0..map_graph.faces[face_id].connections.len() {
                if face_resolved || map_graph.faces[face_id].connections[direction_index].is_some()
                {
                    continue;
                }

                // look for faces next to the missing direction
                let candidate_direction = normal_direction(direction_index);

                // are either tiles linked?
                let linked_directions = map_graph.faces[face_id]
                    .connections
                    .iter()
                    .enumerate()
                    .filter(|(i, c)| {
                        (*c).is_some()
                            && (*i == candidate_direction.0 || *i == candidate_direction.1)
                    })
                    .map(|(i, _)| i);

                for linked_direction_index in linked_directions {
                    let linked_orientation = map_graph.faces[face_id].connections
                        [linked_direction_index]
                        .unwrap()
                        .orientation;
                    let resolved_index = match linked_orientation {
                        Orientation::TWO_CLOCKWISE => linked_orientation.resolve(direction_index),
                        Orientation::SAME => direction_index,
                        _ => opposite_direction(linked_orientation.resolve(direction_index)),
                    };

                    let via_id = map_graph.faces[face_id].connections[linked_direction_index]
                        .unwrap()
                        .cube_face_id;

                    if let Some(target_connection) =
                        map_graph.faces[via_id].connections[resolved_index]
                    {
                        if face_id != target_connection.cube_face_id {
                            debug_assert_ne!(via_id, target_connection.cube_face_id);
                            debug_assert_ne!(via_id, face_id);
                            debug_assert_ne!(face_id, target_connection.cube_face_id);

                            let mut dx = 0;
                            let mut dy = 0;

                            let target_direction = map_graph.faces[via_id]
                                .connections
                                .iter()
                                .enumerate()
                                .filter(|(i, c)| {
                                    c.is_some()
                                        && c.unwrap().cube_face_id == target_connection.cube_face_id
                                })
                                .map(|(i, _)| i)
                                .next()
                                .unwrap();

                            println!("Creating source connection. source {face_id} -> {via_id}");
                            let source_direction = map_graph.faces[face_id]
                                .connections
                                .iter()
                                .enumerate()
                                .filter(|(i, c)| c.is_some() && c.unwrap().cube_face_id == via_id)
                                .map(|(i, _)| i)
                                .next()
                                .unwrap();

                            println!(
                                "resolving...{o:?}, {target_direction}",
                                o = map_graph.faces[face_id].connections[linked_direction_index]
                            );
                            let resolved_target_direction = map_graph.faces[face_id].connections
                                [linked_direction_index]
                                .unwrap()
                                .orientation
                                .resolve(target_direction);
                            let resolved_source_direction = source_direction;

                            let directions =
                                vec![resolved_target_direction, resolved_source_direction];

                            let target_id = target_connection.cube_face_id;
                            println!("    Fetch rotation: Source ID {face_id}, Target ID {target_id}, Via ID {via_id}. TD {target_direction} RTD {resolved_target_direction} | SD {source_direction} RSD {resolved_source_direction}");

                            println!("{directions:?}");

                            for direction in directions {
                                match direction {
                                    NORTH_INDEX => {
                                        dy -= 1;
                                    }
                                    SOUTH_INDEX => {
                                        dy += 1;
                                    }
                                    EAST_INDEX => {
                                        dx += 1;
                                    }
                                    WEST_INDEX => {
                                        dx -= 1;
                                    }
                                    _ => panic!("Unknown direction {direction}"),
                                }
                            }

                            debug_assert!(dx == -1 || dx == 1, "dx: {dx}");
                            debug_assert!(dy == -1 || dy == 1, "dy: {dy}");

                            let mut orientation = Orientation::ONE_CLOCKWISE;
                            if (dx == -1 && dy == 1) || (dx == 1 && dy == -1) {
                                orientation = Orientation::THREE_CLOCKWISE;
                            }

                            if (direction_index % 2) != 0 {
                                // is horizontal

                                if orientation == Orientation::ONE_CLOCKWISE {
                                    orientation = Orientation::THREE_CLOCKWISE;
                                } else {
                                    orientation = Orientation::ONE_CLOCKWISE;
                                }
                            }

                            let source_orientation = map_graph.faces[face_id].connections
                                [linked_direction_index]
                                .unwrap()
                                .orientation;
                            let orientation = orientation
                                .combine(&source_orientation)
                                .combine(&target_connection.orientation);

                            debug_assert!(
                                map_graph.faces[face_id].connections[direction_index].is_none()
                            );

                            let connection = CubeFaceConnection {
                                cube_face_id: target_connection.cube_face_id,
                                orientation,
                            };
                            map_graph.faces[face_id].connections[direction_index] =
                                Some(connection);

                            face_resolved = true;
                            break;
                        }
                    }
                }
            }
        }

        if connection_count == map_graph.connection_count() {
            println!("{map_graph:?}");
            println!();

            for face in &map_graph.faces {
                println!("{face:?}");
                println!();
            }

            panic!("No new connection. {connection_count}");
        }
    }

    map_graph
}

fn map_puzzle(filename: &str) -> io::Result<i32> {
    let (map, instructions) = load_map(filename)?;

    // start is left most top tile

    let mut x = map
        .get(&0)
        .unwrap()
        .keys()
        .into_iter()
        .min()
        .unwrap()
        .to_owned();
    let mut y = 0;
    let mut facing = Facing::EAST;

    for instruction in instructions {
        for _ in 0..instruction.steps {
            let delta = match facing {
                Facing::NORTH => (0, -1),
                Facing::EAST => (1, 0),
                Facing::SOUTH => (0, 1),
                Facing::WEST => (-1, 0),
            };
            let mut new_x = x + delta.0;
            let mut new_y = y + delta.1;

            // do a wrap around
            if fetch_tile(&map, new_x, new_y).is_none() {
                if delta.0 < 0 {
                    new_x = max_x(&map, new_y);
                } else if delta.0 > 0 {
                    new_x = min_x(&map, new_y);
                }
                if delta.1 < 0 {
                    new_y = max_y(&map, new_x);
                } else if delta.1 > 0 {
                    new_y = min_y(&map, new_x);
                }
            }

            if let Some(tile) = fetch_tile(&map, new_x, new_y) {
                match tile {
                    MapSection::WALL => {
                        break;
                    }
                    MapSection::FLOOR => {
                        x = new_x;
                        y = new_y;
                    }
                }
            } else {
                panic!("Could not fetch tile with wrapped coords {new_x}, {new_y}")
            }
        }

        facing = match instruction.direction {
            Direction::LEFT => facing.rotate_left(),
            Direction::RIGHT => facing.rotate_right(),
            Direction::NONE => facing,
        };
    }

    // The final password is the sum of 1000 times the row, 4 times the column, and the facing.
    Ok(1000 * (y + 1) + 4 * (x + 1) + facing.fetch_digit())
}

fn fetch_tile(map: &Map, x: i32, y: i32) -> Option<&MapSection> {
    if let Some(line) = map.get(&y) {
        return line.get(&x);
    }
    None
}

fn min_x(map: &Map, y: i32) -> i32 {
    map.get(&y)
        .unwrap()
        .keys()
        .into_iter()
        .min()
        .unwrap()
        .to_owned()
}

fn max_x(map: &Map, y: i32) -> i32 {
    map.get(&y)
        .unwrap()
        .keys()
        .into_iter()
        .max()
        .unwrap()
        .to_owned()
}

fn min_y(map: &Map, x: i32) -> i32 {
    let mut ys = Vec::new();
    for (y, line) in map {
        if line.get(&x).is_some() {
            ys.push(y);
        }
    }
    *ys.iter().min().unwrap().to_owned()
}

fn max_y(map: &Map, x: i32) -> i32 {
    let mut ys = Vec::new();
    for (y, line) in map {
        if line.get(&x).is_some() {
            ys.push(y);
        }
    }
    *ys.iter().max().unwrap().to_owned()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(map_puzzle("./inputs/day-22-input-test.txt").unwrap(), 6032);
    }

    #[test]
    fn test() {
        assert_eq!(map_puzzle("./inputs/day-22-input.txt").unwrap(), 103224);
    }

    #[test]
    fn calc_3d_map_size_test() {
        let (small_map, _) = load_map("./inputs/day-22-input-test.txt").unwrap();
        let (map, _) = load_map("./inputs/day-22-input.txt").unwrap();

        assert_eq!(4, calc_3d_map_size(&small_map));
        assert_eq!(50, calc_3d_map_size(&map));
    }

    #[test]
    fn build_graph_test() {
        let (small_map, _) = load_map("./inputs/day-22-input-test.txt").unwrap();

        let graph = build_graph(&small_map, 4);

        assert_eq!(4, graph.size);
        assert_eq!(6, graph.faces.len());
        assert_eq!(24, graph.connection_count());

        assert_eq!(0, graph.faces[0].id);
        assert_eq!(8, graph.faces[0].x);
        assert_eq!(0, graph.faces[0].y);
        assert_eq!(1, graph.faces[0].connections[0].unwrap().cube_face_id);
        assert_eq!(
            Orientation::TWO_CLOCKWISE,
            graph.faces[0].connections[0].unwrap().orientation
        );
        assert_eq!(5, graph.faces[0].connections[1].unwrap().cube_face_id);
        assert_eq!(
            Orientation::TWO_CLOCKWISE,
            graph.faces[0].connections[1].unwrap().orientation
        );
        assert_eq!(3, graph.faces[0].connections[2].unwrap().cube_face_id);
        assert_eq!(
            Orientation::SAME,
            graph.faces[0].connections[2].unwrap().orientation
        );
        assert_eq!(2, graph.faces[0].connections[3].unwrap().cube_face_id);
        assert_eq!(
            Orientation::ONE_CLOCKWISE,
            graph.faces[0].connections[3].unwrap().orientation
        );

        assert_eq!(1, graph.faces[1].id);
        assert_eq!(0, graph.faces[1].x);
        assert_eq!(4, graph.faces[1].y);
        assert_eq!(0, graph.faces[1].connections[0].unwrap().cube_face_id);
        assert_eq!(
            Orientation::TWO_CLOCKWISE,
            graph.faces[1].connections[0].unwrap().orientation
        );
        assert_eq!(2, graph.faces[1].connections[1].unwrap().cube_face_id);
        assert_eq!(
            Orientation::SAME,
            graph.faces[1].connections[1].unwrap().orientation
        );
        assert_eq!(4, graph.faces[1].connections[2].unwrap().cube_face_id);
        assert_eq!(
            Orientation::TWO_CLOCKWISE,
            graph.faces[1].connections[2].unwrap().orientation
        );
        assert_eq!(5, graph.faces[1].connections[3].unwrap().cube_face_id);
        assert_eq!(
            Orientation::THREE_CLOCKWISE,
            graph.faces[1].connections[3].unwrap().orientation
        );

        assert_eq!(2, graph.faces[2].id);
        assert_eq!(4, graph.faces[2].x);
        assert_eq!(4, graph.faces[2].y);
        assert_eq!(0, graph.faces[2].connections[0].unwrap().cube_face_id);
        assert_eq!(
            Orientation::THREE_CLOCKWISE,
            graph.faces[2].connections[0].unwrap().orientation
        );
        assert_eq!(3, graph.faces[2].connections[1].unwrap().cube_face_id);
        assert_eq!(
            Orientation::SAME,
            graph.faces[2].connections[1].unwrap().orientation
        );
        assert_eq!(4, graph.faces[2].connections[2].unwrap().cube_face_id);
        assert_eq!(
            Orientation::ONE_CLOCKWISE,
            graph.faces[2].connections[2].unwrap().orientation
        );
        assert_eq!(1, graph.faces[2].connections[3].unwrap().cube_face_id);
        assert_eq!(
            Orientation::SAME,
            graph.faces[2].connections[3].unwrap().orientation
        );

        assert_eq!(3, graph.faces[3].id);
        assert_eq!(8, graph.faces[3].x);
        assert_eq!(4, graph.faces[3].y);
        assert_eq!(0, graph.faces[3].connections[0].unwrap().cube_face_id);
        assert_eq!(
            Orientation::SAME,
            graph.faces[3].connections[0].unwrap().orientation
        );
        assert_eq!(5, graph.faces[3].connections[1].unwrap().cube_face_id);
        assert_eq!(
            Orientation::THREE_CLOCKWISE,
            graph.faces[3].connections[1].unwrap().orientation
        );
        assert_eq!(4, graph.faces[3].connections[2].unwrap().cube_face_id);
        assert_eq!(
            Orientation::SAME,
            graph.faces[3].connections[2].unwrap().orientation
        );
        assert_eq!(2, graph.faces[3].connections[3].unwrap().cube_face_id);
        assert_eq!(
            Orientation::SAME,
            graph.faces[3].connections[3].unwrap().orientation
        );

        assert_eq!(4, graph.faces[4].id);
        assert_eq!(8, graph.faces[4].x);
        assert_eq!(8, graph.faces[4].y);
        assert_eq!(3, graph.faces[4].connections[0].unwrap().cube_face_id);
        assert_eq!(
            Orientation::SAME,
            graph.faces[4].connections[0].unwrap().orientation
        );
        assert_eq!(5, graph.faces[4].connections[1].unwrap().cube_face_id);
        assert_eq!(
            Orientation::SAME,
            graph.faces[4].connections[1].unwrap().orientation
        );
        assert_eq!(1, graph.faces[4].connections[2].unwrap().cube_face_id);
        assert_eq!(
            Orientation::TWO_CLOCKWISE,
            graph.faces[4].connections[2].unwrap().orientation
        );
        assert_eq!(2, graph.faces[4].connections[3].unwrap().cube_face_id);
        assert_eq!(
            Orientation::THREE_CLOCKWISE,
            graph.faces[4].connections[3].unwrap().orientation
        );

        assert_eq!(5, graph.faces[5].id);
        assert_eq!(12, graph.faces[5].x);
        assert_eq!(8, graph.faces[5].y);
        assert_eq!(3, graph.faces[5].connections[0].unwrap().cube_face_id);
        assert_eq!(
            Orientation::ONE_CLOCKWISE,
            graph.faces[5].connections[0].unwrap().orientation
        );
        assert_eq!(0, graph.faces[5].connections[1].unwrap().cube_face_id);
        assert_eq!(
            Orientation::TWO_CLOCKWISE,
            graph.faces[5].connections[1].unwrap().orientation
        );
        assert_eq!(1, graph.faces[5].connections[2].unwrap().cube_face_id);
        assert_eq!(
            Orientation::ONE_CLOCKWISE,
            graph.faces[5].connections[2].unwrap().orientation
        );
        assert_eq!(4, graph.faces[5].connections[3].unwrap().cube_face_id);
        assert_eq!(
            Orientation::SAME,
            graph.faces[5].connections[3].unwrap().orientation
        );
    }
}
