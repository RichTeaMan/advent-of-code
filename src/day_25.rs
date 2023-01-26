use std::io;

use crate::file_utils::read_lines;

fn convert_snafu(snafu: &str) -> i64 {
    let mut decimal = 0;
    for (i, c) in snafu.chars().rev().enumerate() {
        let place = i as u32;

        let multiplier = 5_i64.pow(place);

        let place_value = match c {
            '=' => -2,
            '-' => -1,
            '0' | '1' | '2' => c.to_string().parse::<i64>().unwrap(),
            _ => panic!("Illegal digit '{c}'"),
        } * multiplier;
        decimal += place_value;
    }
    decimal
}

fn max_value_from_place(place: usize) -> i64 {
    (0..=place).map(|n| 5_i64.pow(n as u32)).sum::<i64>() * 2
}

fn convert_to_snafu(decimal: i64) -> String {
    let snafu = snafu_target(decimal, 20_usize);
    let snafu = snafu.trim_start_matches(&['0']).to_string();

    let dec = convert_snafu(snafu.as_str());
    if dec != decimal {
        panic!("Snafu for {decimal} is wrong. Calculated {snafu} but that value is actually {dec}.")
    }

    snafu
}

fn snafu_target(target: i64, max_position: usize) -> String {
    // find left most number closest
    // and we're doing it the dumb way

    let mut closest = Vec::new();

    for place in 0..=max_position {
        let multiplier = 5_i64.pow(place as u32);
        let max_value = if place == 0 {
            0
        } else {
            max_value_from_place(place - 1)
        };
        for digit in -2..=2 {
            if digit == 0 {
                continue;
            }
            let diff = target - (multiplier * digit);
            if diff >= -max_value && diff <= max_value {
                closest.push((diff, digit, place, diff.abs()));
            }
        }
    }

    closest.sort_by(|a, b| a.3.cmp(&b.3));

    // consider attempting all of these in order - many combinations do seem possible when trying the "optimal" route

    let (diff, digit, place, _) = closest[0];
    let snafu_char = match digit {
        2 => "2",
        1 => "1",
        0 => "0",
        -1 => "-",
        -2 => "=",
        _ => panic!("Bad digit {digit}"),
    }
    .to_string();
    let mut snafu = "0".repeat(max_position - (place));
    snafu.push_str(&snafu_char);
    snafu.push_str(&"0".repeat(place));

    if diff != 0 {
        let sub_snafu = snafu_target(diff, place - 1);
        snafu.replace_range((snafu.len() - sub_snafu.len())..snafu.len(), &sub_snafu);
    }
    snafu
}

fn sum_snafu_file(filename: &str) -> io::Result<String> {
    let mut sum_dec = 0;
    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }
        sum_dec += convert_snafu(line.as_str());
    }
    let sum_snafu = convert_to_snafu(sum_dec);
    Ok(sum_snafu)
}

pub fn day_25() -> io::Result<String> {
    let sum = sum_snafu_file("./inputs/day-25-input.txt")?;
    Ok(sum)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn convert_snafu_test() {
        assert_eq!(1747, convert_snafu("1=-0-2"));
        assert_eq!(906, convert_snafu("12111"));
        assert_eq!(198, convert_snafu("2=0="));
        assert_eq!(11, convert_snafu("21"));
        assert_eq!(201, convert_snafu("2=01"));
        assert_eq!(31, convert_snafu("111"));
        assert_eq!(1257, convert_snafu("20012"));
        assert_eq!(32, convert_snafu("112"));
        assert_eq!(353, convert_snafu("1=-1="));
        assert_eq!(107, convert_snafu("1-12"));
        assert_eq!(7, convert_snafu("12"));
        assert_eq!(3, convert_snafu("1="));
        assert_eq!(37, convert_snafu("122"));
    }

    #[test]
    fn convert_to_snafu_test() {
        assert_eq!("-2", convert_to_snafu(-3));
        assert_eq!("1=-0-2", convert_to_snafu(1747));
        assert_eq!("12111", convert_to_snafu(906));
        assert_eq!("2=0=", convert_to_snafu(198));
        assert_eq!("21", convert_to_snafu(11));
        assert_eq!("2=01", convert_to_snafu(201));
        assert_eq!("111", convert_to_snafu(31));
        assert_eq!("20012", convert_to_snafu(1257));
        assert_eq!("112", convert_to_snafu(32));
        assert_eq!("1=-1=", convert_to_snafu(353));
        assert_eq!("1-12", convert_to_snafu(107));
        assert_eq!("12", convert_to_snafu(7));
        assert_eq!("1=", convert_to_snafu(3));
        assert_eq!("122", convert_to_snafu(37));
    }

    #[test]
    fn small_test() {
        let sum = sum_snafu_file("./inputs/day-25-input-test.txt").unwrap();
        assert_eq!("2=-1=0", sum);
    }

    #[test]
    fn test() {
        let sum = sum_snafu_file("./inputs/day-25-input.txt").unwrap();
        assert_eq!("2-2--02=1---1200=0-1", sum);
    }
}
