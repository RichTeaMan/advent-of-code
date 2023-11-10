use std::{
    collections::{HashMap, HashSet},
    io::{self},
};

use itertools::Itertools;

use utils::file_utils::read_lines;

/**
 *  aaaa
 * b    c
 * b    c
 *  dddd
 * e    f
 * e    f
 *  gggg
 */
struct SegmentDisplay {
    a: char,
    b: char,
    c: char,
    d: char,
    e: char,
    f: char,
    g: char,
}

impl SegmentDisplay {
    fn calc_number(&self, digits: &str) -> i32 {
        let chars: String = digits.chars().sorted().collect();

        // 0
        if chars.len() == 6
            && chars.contains(self.a)
            && chars.contains(self.b)
            && chars.contains(self.c)
            && chars.contains(self.e)
            && chars.contains(self.f)
            && chars.contains(self.g)
        {
            return 0;
        }

        // 1
        if chars.len() == 2 && chars.contains(self.c) && chars.contains(self.f) {
            return 1;
        }

        // 2
        if chars.len() == 5
            && chars.contains(self.a)
            && chars.contains(self.c)
            && chars.contains(self.d)
            && chars.contains(self.e)
            && chars.contains(self.g)
        {
            return 2;
        }

        // 3
        if chars.len() == 5
            && chars.contains(self.a)
            && chars.contains(self.c)
            && chars.contains(self.d)
            && chars.contains(self.f)
            && chars.contains(self.g)
        {
            return 3;
        }

        // 4
        if chars.len() == 4
            && chars.contains(self.b)
            && chars.contains(self.c)
            && chars.contains(self.d)
            && chars.contains(self.f)
        {
            return 4;
        }

        // 5
        if chars.len() == 5
            && chars.contains(self.a)
            && chars.contains(self.b)
            && chars.contains(self.d)
            && chars.contains(self.f)
            && chars.contains(self.g)
        {
            return 5;
        }

        // 6
        if chars.len() == 6
            && chars.contains(self.a)
            && chars.contains(self.b)
            && chars.contains(self.d)
            && chars.contains(self.e)
            && chars.contains(self.f)
            && chars.contains(self.g)
        {
            return 6;
        }

        // 7
        if chars.len() == 3
            && chars.contains(self.a)
            && chars.contains(self.c)
            && chars.contains(self.f)
        {
            return 7;
        }

        // 8
        if chars.len() == 7
            && chars.contains(self.a)
            && chars.contains(self.b)
            && chars.contains(self.c)
            && chars.contains(self.d)
            && chars.contains(self.e)
            && chars.contains(self.f)
            && chars.contains(self.g)
        {
            return 8;
        }

        // 9
        if chars.len() == 6
            && chars.contains(self.a)
            && chars.contains(self.b)
            && chars.contains(self.c)
            && chars.contains(self.d)
            && chars.contains(self.f)
            && chars.contains(self.g)
        {
            return 9;
        }

        panic!("Invalid combination: '{chars}'")
    }
}

struct DisplayEntry {
    signal_patterns: Vec<String>,
    output_values: Vec<String>,
}

pub fn day_8() -> io::Result<i32> {
    load_find_unique_numbers("./inputs/day-8-input.txt")
}
pub fn day_8_part_2() -> io::Result<i32> {
    solve_displays("./inputs/day-8-input.txt")
}

fn load_displays(filename: &str) -> io::Result<Vec<DisplayEntry>> {
    let mut displays = Vec::new();

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        let (signal_pattern_str, output_str) = line.split('|').collect_tuple().unwrap();
        displays.push(DisplayEntry {
            signal_patterns: signal_pattern_str
                .split(' ')
                .map(|s| s.to_owned())
                .filter(|s| !s.is_empty())
                .collect_vec(),
            output_values: output_str
                .split(' ')
                .map(|s| s.to_owned())
                .filter(|s| !s.is_empty())
                .collect_vec(),
        });
    }
    Ok(displays)
}

/**
 * Find display entries where the output value contains a 1, 4, 7, or 8. These digits have a unique number of segments in a 7 segment display.
 */
fn find_unique_numbers(displays: &Vec<DisplayEntry>) -> i32 {
    let unique_segments = HashSet::from([
        2_usize, // 1 digit
        4_usize, // 4 digit
        3_usize, // 7 digit
        7_usize, // 8 digit
    ]);

    let mut count = 0;
    for display in displays {
        count += display
            .output_values
            .iter()
            .filter(|d| unique_segments.contains(&d.len()))
            .count();
    }
    count as i32
}

fn load_find_unique_numbers(filename: &str) -> io::Result<i32> {
    let displays = load_displays(filename)?;
    Ok(find_unique_numbers(&displays))
}

fn solve_pattern(display_entry: &DisplayEntry) -> SegmentDisplay {
    let segments_by_count = collect_segments_by_count(display_entry);

    let one_pattern = display_entry
        .signal_patterns
        .iter()
        .find(|p| p.len() == 2)
        .unwrap()
        .chars();
    let four_pattern = display_entry
        .signal_patterns
        .iter()
        .find(|p| p.len() == 4)
        .unwrap()
        .chars();
    let seven_pattern = display_entry
        .signal_patterns
        .iter()
        .find(|p| p.len() == 3)
        .unwrap()
        .chars();

    // a segment is always in 7 but not in 1.
    let a_map = seven_pattern
        .clone()
        .find(|c| !one_pattern.clone().contains(c))
        .unwrap();

    // b segment appears exactly 6 times across all patterns.
    let b_map = segments_by_count
        .get(&6)
        .unwrap()
        .iter()
        .exactly_one()
        .unwrap()
        .to_owned();

    // e segment appears exactly 4 times across all patterns.
    let e_map = segments_by_count
        .get(&4)
        .unwrap()
        .iter()
        .exactly_one()
        .unwrap()
        .to_owned();

    // f segment appears exactly 9 times across all patterns.
    let f_map = segments_by_count
        .get(&9)
        .unwrap()
        .iter()
        .exactly_one()
        .unwrap()
        .to_owned();

    // c segment is always in 7 but isn't a or f.
    let c_map = seven_pattern
        .clone()
        .find(|c| c != &a_map && c != &f_map)
        .unwrap();

    // d segment is always in 4 but isn't b, c, or f.
    let d_map = four_pattern
        .clone()
        .find(|c| c != &b_map && c != &c_map && c != &f_map)
        .unwrap();

    let g_map = segments_by_count
        .get(&7)
        .unwrap()
        .iter()
        .find(|c| c != &&d_map)
        .unwrap()
        .to_owned();

    SegmentDisplay {
        a: a_map,
        b: b_map,
        c: c_map,
        d: d_map,
        e: e_map,
        f: f_map,
        g: g_map,
    }
}

fn collect_segments_by_count(display_entry: &DisplayEntry) -> HashMap<usize, Vec<char>> {
    let letter_count_vec = display_entry
        .signal_patterns
        .join("")
        .chars()
        .sorted()
        .group_by(|&c| c)
        .into_iter()
        .map(|(k, v)| (v.count(), k))
        .collect_vec();

    let mut letter_count_map: HashMap<usize, Vec<char>> = HashMap::new();
    for (count, letter) in letter_count_vec {
        if let Some(letters) = letter_count_map.get_mut(&count) {
            letters.push(letter);
        } else {
            letter_count_map.insert(count, vec![letter]);
        }
    }
    letter_count_map
}

fn solve_displays(filename: &str) -> io::Result<i32> {
    let mut result = 0;

    let displays = load_displays(filename)?;

    for display in displays {
        let segment = solve_pattern(&display);
        let mut answer = 0;
        for output in &display.output_values {
            answer *= 10;
            let digit = segment.calc_number(output);
            answer += digit;
        }
        result += answer;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_test() {
        assert_eq!(
            load_find_unique_numbers("./inputs/day-8-input-test.txt").unwrap(),
            26
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            load_find_unique_numbers("./inputs/day-8-input.txt").unwrap(),
            534
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            solve_displays("./inputs/day-8-input-test.txt").unwrap(),
            61229
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(solve_displays("./inputs/day-8-input.txt").unwrap(), 1070188);
    }
}
