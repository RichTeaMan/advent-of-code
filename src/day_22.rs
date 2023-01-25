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
    let result = cube_puzzle("./inputs/day-22-input.txt")?;
    Ok(result)
}

#[derive(PartialEq, Eq, Clone, Copy)]
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
    Same,
    OneClockwise,
    TwoClockwise,
    ThreeClockwise,
}

impl Orientation {
    fn combine(self, a: &Orientation) -> Orientation {
        let combined = (self.orientation_as_number() + a.orientation_as_number()) % 4;

        match combined {
            0 => Orientation::Same,
            1 => Orientation::OneClockwise,
            2 => Orientation::TwoClockwise,
            3 => Orientation::ThreeClockwise,
            _ => panic!("Bad orientation number: {combined}"),
        }
    }

    fn orientation_as_number(&self) -> usize {
        match self {
            Orientation::Same => 0,
            Orientation::OneClockwise => 1,
            Orientation::TwoClockwise => 2,
            Orientation::ThreeClockwise => 3,
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

#[derive(Debug)]
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

    fn fetch_face_id_at_location(&self, x: i32, y: i32) -> Option<usize> {
        if let Some(face) = self
            .faces
            .iter()
            .filter(|f| x >= f.x && x < f.x + self.size && y >= f.y && y < f.y + self.size)
            .next()
        {
            Some(face.id)
        } else {
            None
        }
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

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn calc_heading(direction: usize) -> Point {
        match (direction + 4) % 4 {
            0 => Point { x: 0, y: -1 },
            1 => Point { x: 1, y: 0 },
            2 => Point { x: 0, y: 1 },
            3 => Point { x: -1, y: 0 },
            _ => panic!("Unknown direction: {direction}"),
        }
    }
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
            let mut hacked_line = line.clone();
            hacked_line = hacked_line.replace("L", ",L").replace("R", ",R");
            let directions = hacked_line.split(&[',']);

            for direction in directions {
                let steps_str = direction.trim_start_matches(&['L', 'R', 'N']);
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

fn build_graph(map: &Map) -> MapGraph {
    let size = calc_3d_map_size(map);

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
                        orientation: Orientation::Same,
                    };
                    face.connections[NORTH_INDEX] = Some(connection);

                    let partner_connection = CubeFaceConnection {
                        cube_face_id: face.id,
                        orientation: Orientation::Same,
                    };
                    faces[up_face_id].connections[SOUTH_INDEX] = Some(partner_connection);
                }

                if let Some(left_face_id) = left_face_id_opt {
                    let connection = CubeFaceConnection {
                        cube_face_id: left_face_id,
                        orientation: Orientation::Same,
                    };
                    face.connections[WEST_INDEX] = Some(connection);

                    let partner_connection = CubeFaceConnection {
                        cube_face_id: face.id,
                        orientation: Orientation::Same,
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
                        Orientation::TwoClockwise => linked_orientation.resolve(direction_index),
                        Orientation::Same => direction_index,
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
                                .filter(|(_, c)| {
                                    c.is_some()
                                        && c.unwrap().cube_face_id == target_connection.cube_face_id
                                })
                                .map(|(i, _)| i)
                                .next()
                                .unwrap();

                            let source_direction = map_graph.faces[face_id]
                                .connections
                                .iter()
                                .enumerate()
                                .filter(|(_, c)| c.is_some() && c.unwrap().cube_face_id == via_id)
                                .map(|(i, _)| i)
                                .next()
                                .unwrap();

                            let resolved_target_direction = map_graph.faces[face_id].connections
                                [linked_direction_index]
                                .unwrap()
                                .orientation
                                .resolve(target_direction);
                            let resolved_source_direction = source_direction;

                            let directions =
                                vec![resolved_target_direction, resolved_source_direction];

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

                            let mut orientation = Orientation::OneClockwise;
                            if (dx == -1 && dy == 1) || (dx == 1 && dy == -1) {
                                orientation = Orientation::ThreeClockwise;
                            }

                            if (direction_index % 2) != 0 {
                                // is horizontal

                                if orientation == Orientation::OneClockwise {
                                    orientation = Orientation::ThreeClockwise;
                                } else {
                                    orientation = Orientation::OneClockwise;
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
        facing = match instruction.direction {
            Direction::LEFT => facing.rotate_left(),
            Direction::RIGHT => facing.rotate_right(),
            Direction::NONE => facing,
        };

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
    }

    // The final password is the sum of 1000 times the row, 4 times the column, and the facing.
    Ok(1000 * (y + 1) + 4 * (x + 1) + facing.fetch_digit())
}

fn cube_puzzle(file_path: &str) -> io::Result<i32> {
    let (map, instructions) = load_map(file_path)?;

    let cube = build_graph(&map);

    // start location is top left face, which is always the first
    let mut x = cube.faces[0].x;
    let mut y = cube.faces[0].y;

    let mut direction = EAST_INDEX;
    let mut face_id = cube.faces[0].id;

    for instruction in instructions {
        if instruction.direction == Direction::LEFT {
            direction = direction.wrapping_sub(1);
        } else if instruction.direction == Direction::RIGHT {
            direction += 1;
        }
        direction = direction % 4;
        let mut heading = Point::calc_heading(direction);

        for _ in 0..instruction.steps {
            let mut new_x = x + heading.x;
            let mut new_y = y + heading.y;

            let mut new_face_id = face_id;
            let mut new_direction = direction;
            let mut new_heading = heading;

            if Some(face_id) != cube.fetch_face_id_at_location(new_x, new_y) {
                // just moved face. hold onto your butts
                // FetchFaceAtLocation is not reliable until coords have been resolved

                let connection = cube.faces[face_id].connections[direction].unwrap();
                new_face_id = connection.cube_face_id;

                let mut pre_rot_x = new_x - cube.faces[face_id].x;
                let mut pre_rot_y = new_y - cube.faces[face_id].y;

                match direction {
                    NORTH_INDEX => {
                        pre_rot_y = cube.size - 1;
                    }
                    SOUTH_INDEX => {
                        pre_rot_y = 0;
                    }
                    WEST_INDEX => {
                        pre_rot_x = cube.size - 1;
                    }
                    EAST_INDEX => {
                        pre_rot_x = 0;
                    }
                    _ => panic!("Unexpected direction {direction}"),
                }
                debug_assert!(pre_rot_x >= 0 && pre_rot_x < cube.size);
                debug_assert!(pre_rot_y >= 0 && pre_rot_y < cube.size);

                let mut rot_x = pre_rot_x;
                let mut rot_y = pre_rot_y;

                // now do a rotation
                // maybe plus 1
                for _ in 0..((4 - connection.orientation.orientation_as_number()) % 4) {
                    let tx = rot_x;
                    let ty = rot_y;

                    rot_x = (cube.size - 1) - ty;
                    rot_y = tx;
                }

                debug_assert!(rot_x >= 0 && rot_x < cube.size);
                debug_assert!(rot_y >= 0 && rot_y < cube.size);

                new_x = rot_x + cube.faces[new_face_id].x;
                new_y = rot_y + cube.faces[new_face_id].y;

                new_direction =
                    (direction + (4 - connection.orientation.orientation_as_number())) % 4;
                new_heading = Point::calc_heading(new_direction);
            }

            let tile = *fetch_tile(&map, new_x, new_y).unwrap();

            if tile == MapSection::FLOOR {
                x = new_x;
                y = new_y;

                direction = new_direction;
                heading = new_heading;
                face_id = new_face_id;
            } else {
                break;
            }
        }
    }

    // get score
    // convert direction to right = 0
    let score_direction = direction.wrapping_sub(1) % 4;
    let score = score_direction as i32 + (4 * (x + 1)) + (1000 * (y + 1));
    Ok(score)
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

        let graph = build_graph(&small_map);

        assert_eq!(4, graph.size);
        assert_eq!(6, graph.faces.len());
        assert_eq!(24, graph.connection_count());

        assert_eq!(0, graph.faces[0].id);
        assert_eq!(8, graph.faces[0].x);
        assert_eq!(0, graph.faces[0].y);
        assert_eq!(1, graph.faces[0].connections[0].unwrap().cube_face_id);
        assert_eq!(
            Orientation::TwoClockwise,
            graph.faces[0].connections[0].unwrap().orientation
        );
        assert_eq!(5, graph.faces[0].connections[1].unwrap().cube_face_id);
        assert_eq!(
            Orientation::TwoClockwise,
            graph.faces[0].connections[1].unwrap().orientation
        );
        assert_eq!(3, graph.faces[0].connections[2].unwrap().cube_face_id);
        assert_eq!(
            Orientation::Same,
            graph.faces[0].connections[2].unwrap().orientation
        );
        assert_eq!(2, graph.faces[0].connections[3].unwrap().cube_face_id);
        assert_eq!(
            Orientation::OneClockwise,
            graph.faces[0].connections[3].unwrap().orientation
        );

        assert_eq!(1, graph.faces[1].id);
        assert_eq!(0, graph.faces[1].x);
        assert_eq!(4, graph.faces[1].y);
        assert_eq!(0, graph.faces[1].connections[0].unwrap().cube_face_id);
        assert_eq!(
            Orientation::TwoClockwise,
            graph.faces[1].connections[0].unwrap().orientation
        );
        assert_eq!(2, graph.faces[1].connections[1].unwrap().cube_face_id);
        assert_eq!(
            Orientation::Same,
            graph.faces[1].connections[1].unwrap().orientation
        );
        assert_eq!(4, graph.faces[1].connections[2].unwrap().cube_face_id);
        assert_eq!(
            Orientation::TwoClockwise,
            graph.faces[1].connections[2].unwrap().orientation
        );
        assert_eq!(5, graph.faces[1].connections[3].unwrap().cube_face_id);
        assert_eq!(
            Orientation::ThreeClockwise,
            graph.faces[1].connections[3].unwrap().orientation
        );

        assert_eq!(2, graph.faces[2].id);
        assert_eq!(4, graph.faces[2].x);
        assert_eq!(4, graph.faces[2].y);
        assert_eq!(0, graph.faces[2].connections[0].unwrap().cube_face_id);
        assert_eq!(
            Orientation::ThreeClockwise,
            graph.faces[2].connections[0].unwrap().orientation
        );
        assert_eq!(3, graph.faces[2].connections[1].unwrap().cube_face_id);
        assert_eq!(
            Orientation::Same,
            graph.faces[2].connections[1].unwrap().orientation
        );
        assert_eq!(4, graph.faces[2].connections[2].unwrap().cube_face_id);
        assert_eq!(
            Orientation::OneClockwise,
            graph.faces[2].connections[2].unwrap().orientation
        );
        assert_eq!(1, graph.faces[2].connections[3].unwrap().cube_face_id);
        assert_eq!(
            Orientation::Same,
            graph.faces[2].connections[3].unwrap().orientation
        );

        assert_eq!(3, graph.faces[3].id);
        assert_eq!(8, graph.faces[3].x);
        assert_eq!(4, graph.faces[3].y);
        assert_eq!(0, graph.faces[3].connections[0].unwrap().cube_face_id);
        assert_eq!(
            Orientation::Same,
            graph.faces[3].connections[0].unwrap().orientation
        );
        assert_eq!(5, graph.faces[3].connections[1].unwrap().cube_face_id);
        assert_eq!(
            Orientation::ThreeClockwise,
            graph.faces[3].connections[1].unwrap().orientation
        );
        assert_eq!(4, graph.faces[3].connections[2].unwrap().cube_face_id);
        assert_eq!(
            Orientation::Same,
            graph.faces[3].connections[2].unwrap().orientation
        );
        assert_eq!(2, graph.faces[3].connections[3].unwrap().cube_face_id);
        assert_eq!(
            Orientation::Same,
            graph.faces[3].connections[3].unwrap().orientation
        );

        assert_eq!(4, graph.faces[4].id);
        assert_eq!(8, graph.faces[4].x);
        assert_eq!(8, graph.faces[4].y);
        assert_eq!(3, graph.faces[4].connections[0].unwrap().cube_face_id);
        assert_eq!(
            Orientation::Same,
            graph.faces[4].connections[0].unwrap().orientation
        );
        assert_eq!(5, graph.faces[4].connections[1].unwrap().cube_face_id);
        assert_eq!(
            Orientation::Same,
            graph.faces[4].connections[1].unwrap().orientation
        );
        assert_eq!(1, graph.faces[4].connections[2].unwrap().cube_face_id);
        assert_eq!(
            Orientation::TwoClockwise,
            graph.faces[4].connections[2].unwrap().orientation
        );
        assert_eq!(2, graph.faces[4].connections[3].unwrap().cube_face_id);
        assert_eq!(
            Orientation::ThreeClockwise,
            graph.faces[4].connections[3].unwrap().orientation
        );

        assert_eq!(5, graph.faces[5].id);
        assert_eq!(12, graph.faces[5].x);
        assert_eq!(8, graph.faces[5].y);
        assert_eq!(3, graph.faces[5].connections[0].unwrap().cube_face_id);
        assert_eq!(
            Orientation::OneClockwise,
            graph.faces[5].connections[0].unwrap().orientation
        );
        assert_eq!(0, graph.faces[5].connections[1].unwrap().cube_face_id);
        assert_eq!(
            Orientation::TwoClockwise,
            graph.faces[5].connections[1].unwrap().orientation
        );
        assert_eq!(1, graph.faces[5].connections[2].unwrap().cube_face_id);
        assert_eq!(
            Orientation::OneClockwise,
            graph.faces[5].connections[2].unwrap().orientation
        );
        assert_eq!(4, graph.faces[5].connections[3].unwrap().cube_face_id);
        assert_eq!(
            Orientation::Same,
            graph.faces[5].connections[3].unwrap().orientation
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(5031, cube_puzzle("./inputs/day-22-input-test.txt").unwrap());
    }

    #[test]
    fn part_2_test() {
        assert_eq!(189097, cube_puzzle("./inputs/day-22-input.txt").unwrap());
    }

    #[test]
    fn instruction_parse_test() {
        let (_, instructions) = load_map("./inputs/day-22-input-test.txt").unwrap();

        // 10R5L5R10L4R5L5
        assert_eq!(7, instructions.len());
        assert_eq!(10, instructions[0].steps);

        assert_eq!(Direction::NONE, instructions[0].direction);
        assert_eq!(5, instructions[1].steps);
        assert_eq!(Direction::RIGHT, instructions[1].direction);
        assert_eq!(5, instructions[2].steps);
        assert_eq!(Direction::LEFT, instructions[2].direction);
        assert_eq!(10, instructions[3].steps);
        assert_eq!(Direction::RIGHT, instructions[3].direction);
        assert_eq!(4, instructions[4].steps);
        assert_eq!(Direction::LEFT, instructions[4].direction);
        assert_eq!(5, instructions[5].steps);
        assert_eq!(Direction::RIGHT, instructions[5].direction);
        assert_eq!(5, instructions[6].steps);
        assert_eq!(Direction::LEFT, instructions[6].direction);
    }
}
