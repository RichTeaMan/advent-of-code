use std::{
    collections::HashSet,
    io::{self},
};

use itertools::Itertools;

use crate::file_utils::read_lines;

pub fn day_8() -> io::Result<i32> {
    let visible_trees = fetch_visible_trees("./inputs/day-8-input.txt")?;
    Ok(visible_trees)
}

pub fn day_8_part_2() -> io::Result<i32> {
    let score = fetch_best_score("./inputs/day-8-input.txt")?;
    Ok(score)
}

fn fetch_best_score(filename: &str) -> io::Result<i32> {
    let mut highest_score = 0;

    let trees = load_trees(filename)?;

    let row_count = fetch_row_count(&trees);
    let column_count = fetch_column_count(&trees);

    for y in 1..(row_count - 1) {
        for x in 1..(column_count - 1) {
            let mut left = 0;
            let mut up = 0;
            let mut right = 0;
            let mut down = 0;

            let tree = fetch_tree(&trees, x, y);

            // walk left
            for z in (0..(x)).rev() {
                let a = z;
                let b = y;
                let f_tree = fetch_tree(&trees, a, b);
                left += 1;
                if f_tree >= tree {
                    break;
                }
            }

            // walk right
            for z in (x + 1)..column_count {
                let a = z;
                let b = y;
                let f_tree = fetch_tree(&trees, a, b);
                right += 1;
                if f_tree >= tree {
                    break;
                }
            }

            // walk up
            for z in (0..(y)).rev() {
                let a = x;
                let b = z;
                let f_tree = fetch_tree(&trees, a, b);
                up += 1;
                if f_tree >= tree {
                    break;
                }
            }

            // walk down
            for z in (y + 1)..row_count {
                let a = x;
                let b = z;
                let f_tree = fetch_tree(&trees, a, b);
                down += 1;
                if f_tree >= tree {
                    break;
                }
            }
            let score = left * up * right * down;

            if score > highest_score {
                highest_score = score;
            }
        }
    }

    Ok(highest_score)
}

fn fetch_visible_trees(filename: &str) -> io::Result<i32> {
    let mut visible_tree_count = HashSet::new();

    let trees = load_trees(filename)?;

    // left perspective
    for y in 0..fetch_row_count(&trees) {
        let mut tallest = -1;
        for x in 0..fetch_column_count(&trees) {
            let tree = fetch_tree(&trees, x, y);
            if tree > tallest {
                tallest = tree;
                visible_tree_count.insert((x, y));
            }
        }
    }
    // right perspective
    for y in 0..fetch_row_count(&trees) {
        let mut tallest = -1;
        for x in (0..fetch_column_count(&trees)).rev() {
            let tree = fetch_tree(&trees, x, y);
            if tree > tallest {
                tallest = tree;
                visible_tree_count.insert((x, y));
            }
        }
    }
    // top perspective
    for x in 0..fetch_column_count(&trees) {
        let mut tallest = -1;
        for y in 0..fetch_row_count(&trees) {
            let tree = fetch_tree(&trees, x, y);
            if tree > tallest {
                tallest = tree;
                visible_tree_count.insert((x, y));
            }
        }
    }
    // bottom perspective
    for x in 0..fetch_column_count(&trees) {
        let mut tallest = -1;
        for y in (0..fetch_row_count(&trees)).rev() {
            let tree = fetch_tree(&trees, x, y);
            if tree > tallest {
                tallest = tree;
                visible_tree_count.insert((x, y));
            }
        }
    }

    Ok(visible_tree_count.len() as i32)
}

/**
 * Returns all trees as a vector of rows, eg [y, x].
 */
fn load_trees(filename: &str) -> io::Result<Vec<Vec<i32>>> {
    let mut tree_rows = Vec::new();
    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        let tree_row = line
            .chars()
            .map(|c| {
                if let Ok(tree) = c.to_string().parse::<i32>() {
                    tree
                } else {
                    panic!("Could not parse row.");
                }
            })
            .collect_vec();
        tree_rows.push(tree_row);
    }
    Ok(tree_rows)
}

fn fetch_tree(trees: &[Vec<i32>], x: i32, y: i32) -> i32 {
    trees[y as usize][x as usize]
}

fn fetch_row_count(trees: &[Vec<i32>]) -> i32 {
    trees.len() as i32
}

fn fetch_column_count(trees: &[Vec<i32>]) -> i32 {
    trees[0].len() as i32
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            fetch_visible_trees("./inputs/day-8-input-test.txt").unwrap(),
            21
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            fetch_visible_trees("./inputs/day-8-input.txt").unwrap(),
            1546
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            fetch_best_score("./inputs/day-8-input-test.txt").unwrap(),
            8
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            fetch_best_score("./inputs/day-8-input.txt").unwrap(),
            519064
        );
    }
}
