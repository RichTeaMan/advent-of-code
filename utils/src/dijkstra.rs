use std::collections::{BinaryHeap, HashSet};

use crate::coordinate::{Coordinate, CoordinateMap};

#[derive(Debug, PartialEq, Eq)]
struct DijkstraNode {
    pub coordinate: Coordinate,
    /**
     * Gets the cost of travelling to this node from the start node.
     */
    pub total_cost: i32,
}

impl Ord for DijkstraNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.total_cost.cmp(&self.total_cost)
    }
}

impl PartialOrd for DijkstraNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/**
 * Uses Dijkstra's algorithm to find the shortest path from the start node to the end node.
 *
 * The route is not returned, only the cost of traversing that path.
 *
 * Returns i32::MAX if no path is found.
 */
pub fn calc_route_cost(map: &CoordinateMap<i32>, start: &Coordinate, end: &Coordinate) -> i32 {
    let mut unvisited = HashSet::new();
    for c in map.keys() {
        unvisited.insert(*c);
    }

    let mut heap: BinaryHeap<DijkstraNode> = BinaryHeap::new();
    heap.push(DijkstraNode {
        coordinate: *start,
        total_cost: 0,
    });

    while let Some(current) = heap.pop() {
        let current_risk = current.total_cost;

        if current.coordinate == *end {
            return current_risk;
        }

        if !unvisited.contains(&current.coordinate) {
            continue;
        }

        debug_assert!(current_risk != i32::MAX);
        let adjacent = current.coordinate.orthogonal();

        for coordinate in adjacent {
            if unvisited.contains(&coordinate) {
                let new_risk = current_risk + map.get(&coordinate).unwrap();
                heap.push(DijkstraNode {
                    coordinate,
                    total_cost: new_risk,
                });
            }
        }
        unvisited.remove(&current.coordinate);
    }

    i32::MAX
}
