use std::{
    collections::HashSet,
    io::{self},
};

use itertools::Itertools;

use crate::file_utils::read_lines;

struct Fold {
    axis: char,
    position: i32,
}

pub fn day_13() -> io::Result<i32> {
    fold_first("./inputs/day-13-input.txt")
}
pub fn day_13_part_2() -> io::Result<String> {
    calc_plot("./inputs/day-13-input.txt")
}

type FoldingPlot = (HashSet<(i32, i32)>, Vec<Fold>);

fn load_plot(filename: &str) -> io::Result<FoldingPlot> {
    let mut plot = HashSet::new();
    let mut folds = Vec::new();

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        if line.contains(',') {
            let (x, y) = line
                .split(',')
                .map(|p| p.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();
            plot.insert((x, y));
        } else {
            let (axis, position) = sscanf::sscanf!(line, "fold along {}={}", char, i32).unwrap();
            folds.push(Fold { axis, position });
        }
    }
    Ok((plot, folds))
}

fn perform_fold(plot: HashSet<(i32, i32)>, fold: &Fold) -> HashSet<(i32, i32)> {
    let mut new_plot = HashSet::new();

    for (x, y) in plot {
        if fold.axis == 'x' {
            if x > fold.position {
                new_plot.insert((2 * fold.position - x, y));
                continue;
            }
        } else if fold.axis == 'y' {
            if y > fold.position {
                new_plot.insert((x, 2 * fold.position - y));
                continue;
            }
        } else {
            panic!("Unknown fold axis.");
        }
        new_plot.insert((x, y));
    }
    new_plot
}

fn fold_first(filepath: &str) -> io::Result<i32> {
    let (plot, folds) = load_plot(filepath)?;
    let new_plot = perform_fold(plot, folds.get(0).unwrap());
    Ok(new_plot.len() as i32)
}

fn performs_folds(filepath: &str) -> io::Result<HashSet<(i32, i32)>> {
    let (plot, folds) = load_plot(filepath)?;
    let mut new_plot = plot;
    for fold in folds {
        new_plot = perform_fold(new_plot, &fold);
    }
    Ok(new_plot)
}

fn print_plot(plot: &HashSet<(i32, i32)>) -> String {
    let width = plot.iter().map(|(x, _)| x).max().unwrap();
    let height = plot.iter().map(|(_, y)| y).max().unwrap();

    let mut output: String = String::new();

    for y in 0..=*height {
        for x in 0..=*width {
            if plot.contains(&(x, y)) {
                output.push('#');
            } else {
                output.push('.');
            }
        }
        output.push('\n');
    }
    output
}

fn calc_plot(filepath: &str) -> io::Result<String> {
    let plot = performs_folds(filepath)?;
    Ok(print_plot(&plot))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(fold_first("./inputs/day-13-input-test.txt").unwrap(), 17);
    }

    #[test]
    fn test() {
        assert_eq!(fold_first("./inputs/day-13-input.txt").unwrap(), 814);
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            calc_plot("./inputs/day-13-input-test.txt").unwrap(),
            r#"#####
#...#
#...#
#...#
#####
"#
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            calc_plot("./inputs/day-13-input.txt").unwrap(),
            r#"###..####.####.#..#.###...##..####.###.
#..#....#.#....#..#.#..#.#..#.#....#..#
#..#...#..###..####.#..#.#..#.###..#..#
###...#...#....#..#.###..####.#....###.
#....#....#....#..#.#.#..#..#.#....#.#.
#....####.####.#..#.#..#.#..#.####.#..#
"#
        );
    }
}
