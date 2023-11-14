use std::{collections::VecDeque, io};

use utils::file_utils::read_lines;

pub fn day_18() -> io::Result<i64> {
    add_numbers_from_file("./inputs/day-18-input.txt")
}
pub fn day_18_part_2() -> io::Result<i64> {
    find_largest_magnitude_from_file("./inputs/day-18-input.txt")
}

fn find_largest_magnitude_from_file(filename: &str) -> io::Result<i64> {
    let numbers = load_snail_numbers(filename)?;

    let mut magnitudes = Vec::new();

    for (a_i, a) in numbers.iter().enumerate() {
        for (b_i, b) in numbers.iter().enumerate() {
            if a_i == b_i {
                continue;
            }
            let added = add(a.clone(), b.clone());
            let mag = added.magnitude();
            magnitudes.push(mag);
        }
    }
    Ok(*magnitudes.iter().max().unwrap())
}

fn add_numbers_from_file(filename: &str) -> io::Result<i64> {
    let numbers = load_snail_numbers(filename)?;
    Ok(add_numbers_from_vec(numbers))
}

fn add_numbers_from_vec(numbers: Vec<SnailNumber>) -> i64 {
    if let Some((first, elements)) = numbers.split_first() {
        let mut accumulator: SnailNumber = first.clone();
        for num in elements {
            accumulator = add(accumulator, num.clone());
        }
        return accumulator.magnitude();
    }
    panic!("Could not load numbers")
}

#[derive(Clone)]
struct SnailNumber {
    a: SnailValue,
    b: SnailValue,
}

impl SnailNumber {
    fn from_str(str: &str) -> Self {
        let mut chars = VecDeque::new();
        chars.extend(str.chars());

        let result = Self::from_chars(&mut chars);
        assert_eq!(chars.len(), 0, "Failed to consume all chars.");
        result
    }

    fn from_chars(chars: &mut VecDeque<char>) -> Self {
        if let Some(first) = chars.pop_front() {
            if first != '[' {
                panic!("Expected '[' but got '{}'", first);
            }
        }
        let value_a = if let Some(c) = chars.front() {
            match c {
                '[' => SnailValue::from_pair(SnailNumber::from_chars(chars)),
                token if token.is_ascii_digit() => SnailValue::from_chars(chars),
                _ => panic!("Expected '0'..'9' or '[' but got '{}'", c),
            }
        } else {
            panic!("Unexpected end of input.");
        };

        if let Some(comma) = chars.pop_front() {
            if comma != ',' {
                panic!("Expected comma but got'{}'", comma);
            }
        }

        let value_b = if let Some(c) = chars.front() {
            match c {
                '[' => SnailValue::from_pair(SnailNumber::from_chars(chars)),
                token if token.is_ascii_digit() => SnailValue::from_chars(chars),
                _ => panic!("Expected '0'..'9' or '[' but got '{}'", c),
            }
        } else {
            panic!("Unexpected end of input.");
        };

        if let Some(last) = chars.pop_front() {
            if last != ']' {
                panic!("Expected ']' but got '{}'", last);
            }
        }

        SnailNumber {
            a: value_a,
            b: value_b,
        }
    }

    fn magnitude(&self) -> i64 {
        (3 * self.a.magnitude()) + (2 * self.b.magnitude())
    }

    fn to_str(&self) -> String {
        format!("[{a},{b}]", a = self.a.to_str(), b = self.b.to_str())
    }

    fn explode(&self) -> Option<SnailNumber> {
        let org_str = self.to_str();

        let mut chars = VecDeque::with_capacity(org_str.len());
        chars.extend(org_str.chars());

        let mut depth = 0;

        let mut index = 0;
        let mut explode_str = None;
        let mut explode_index_opt = None;

        let mut left_num_opt = None;
        let mut right_num_opt = None;

        let mut left_num_index_opt = None;
        let mut right_num_index_opt = None;

        let mut old_left_num_length_opt = None;
        let mut old_right_num_length_opt = None;

        while !chars.is_empty() {
            let c = chars.pop_front().unwrap();
            index += 1;

            if c == '[' {
                depth += 1;
                if depth >= 5 && chars.front().unwrap() != &'[' && explode_str.is_none() {
                    // check if we're in a regular number pair
                    let num_1 = read_number_from_chars(&chars, 0);
                    let c2 = chars.get(num_1.1).unwrap();
                    let num_2 = read_number_from_chars(&chars, num_1.1 + 1);

                    if num_1.1 > 0 && c2 == &',' && num_2.1 > 0 {
                        explode_str = Some(format!("[{a},{b}]", a = num_1.0, b = num_2.0));
                        explode_index_opt = Some(index - 1);

                        for _ in 0..(num_1.1 + num_2.1 + 1) {
                            chars.pop_front();
                        }
                        index += num_1.1 + num_2.1 + 1;
                        left_num_opt = Some(num_1.0);
                        right_num_opt = Some(num_2.0);
                    }
                }
            }
            if c == ']' {
                depth -= 1;
            }

            if c.is_ascii_digit() {
                // find right digit and index
                if let Some(right_num) = right_num_opt {
                    chars.push_front(c);
                    let (num, right_num_length) = read_number_from_chars(&chars, 0);

                    if right_num_length > 0 {
                        right_num_index_opt = Some(index - 1);
                        right_num_opt = Some(right_num + num);
                        old_right_num_length_opt = Some(right_num_length);
                        break;
                    }
                    chars.pop_front();
                }
            }
        }

        explode_index_opt?;

        let mut left_num = left_num_opt.unwrap();
        let right_num = right_num_opt.unwrap();

        let explode_index = explode_index_opt.unwrap();

        let mut last_left_num = 0;

        let mut left_haystack = VecDeque::with_capacity(org_str.len());
        left_haystack.extend(org_str.chars());

        // find left index
        let mut skip_indexes = 0;
        for (index, c) in org_str.chars().enumerate() {
            if index >= explode_index {
                break;
            }
            if skip_indexes > 0 {
                skip_indexes -= 1;
                continue;
            }

            if c.is_ascii_digit() {
                left_num_index_opt = Some(index);
                let (num, indexes) = read_number_from_chars(&left_haystack, index);
                last_left_num = num;
                old_left_num_length_opt = Some(indexes);
                skip_indexes = indexes;
            }
        }

        left_num += last_left_num;

        let mut res = org_str;

        // and now to rebuild the whole thing
        if let Some(right_num_index) = right_num_index_opt {
            let (a, b) = res.split_at_mut(right_num_index + old_right_num_length_opt.unwrap());
            let mut first = a.to_string();
            for _ in 0..old_right_num_length_opt.unwrap() {
                first.pop();
            }
            res = format!("{first}{right_num}{b}");
        }

        // and replace explosion
        let (a, b) = res.split_at_mut(explode_index);
        let replace = b.replacen(explode_str.unwrap().as_str(), "0", 1);
        res = format!("{a}{replace}");

        if let Some(left_num_index) = left_num_index_opt {
            let (a, b) = res.split_at_mut(left_num_index + old_left_num_length_opt.unwrap());
            let mut first = a.to_string();
            for _ in 0..old_left_num_length_opt.unwrap() {
                first.pop();
            }
            res = format!("{first}{left_num}{b}");
        }

        Some(SnailNumber::from_str(res.as_str()))
    }

    fn split(&mut self) -> bool {
        if let Some(a) = split_number(self.a.number) {
            self.a = a;
            return true;
        }
        if let Some(a_pair) = self.a.pair.as_mut() {
            if a_pair.split() {
                return true;
            }
        }

        if let Some(b) = split_number(self.b.number) {
            self.b = b;
            return true;
        }
        if let Some(b_pair) = self.b.pair.as_mut() {
            if b_pair.split() {
                return true;
            }
        }

        false
    }
}

fn split_number(num_opt: Option<i32>) -> Option<SnailValue> {
    if let Some(num) = num_opt {
        if num >= 10 {
            let new_a = num / 2;
            let mut new_b = new_a;

            if num % 2 == 1 {
                new_b += 1;
            }
            debug_assert_eq!(new_a + new_b, num);
            debug_assert!(new_a <= new_b);
            let number = SnailNumber {
                a: SnailValue {
                    number: Some(new_a),
                    pair: None,
                },
                b: SnailValue {
                    number: Some(new_b),
                    pair: None,
                },
            };
            return Some(SnailValue {
                number: None,
                pair: Some(Box::new(number)),
            });
        }
    }
    None
}

fn reduce_number(mut snail_number: SnailNumber) -> SnailNumber {
    loop {
        if let Some(exploded_number) = snail_number.explode() {
            snail_number = exploded_number;
            continue;
        }

        if snail_number.split() {
            continue;
        }
        break;
    }
    snail_number
}

#[derive(Clone)]
struct SnailValue {
    number: Option<i32>,
    pair: Option<Box<SnailNumber>>,
}

fn read_number_from_chars(chars: &VecDeque<char>, skip: usize) -> (i32, usize) {
    let mut index = 0;
    let mut value = 0;

    while let Some(c) = chars.get(index + skip) {
        if let Ok(v) = c.to_string().parse::<i32>() {
            value = (value * 10) + v;
        } else {
            break;
        }
        index += 1;
    }
    (value, index)
}

impl SnailValue {
    fn from_pair(pair: SnailNumber) -> Self {
        let pair = Box::new(Some(Box::new(pair)));
        SnailValue {
            number: None,
            pair: *pair,
        }
    }

    fn from_chars(chars: &mut VecDeque<char>) -> Self {
        let mut value = 0;
        while let Some(c) = chars.pop_front() {
            if c.is_ascii_digit() {
                value = (value * 10) + c.to_string().parse::<i32>().unwrap();
            } else {
                chars.push_front(c);
                break;
            }
        }
        SnailValue {
            number: Some(value),
            pair: None,
        }
    }

    fn to_str(&self) -> String {
        if let Some(value) = self.number {
            return value.to_string();
        }
        if let Some(value) = &self.pair {
            return value.to_str();
        }
        unreachable!()
    }

    fn magnitude(&self) -> i64 {
        if let Some(num) = self.number {
            return num as i64;
        }
        if let Some(pair) = &self.pair {
            return pair.magnitude();
        }
        unreachable!()
    }
}

fn load_snail_numbers(filename: &str) -> io::Result<Vec<SnailNumber>> {
    let mut snail_numbers = Vec::new();
    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        let snail_number = SnailNumber::from_str(line.as_str());
        snail_numbers.push(snail_number);
    }
    Ok(snail_numbers)
}

fn add(a: SnailNumber, b: SnailNumber) -> SnailNumber {
    let result = SnailNumber {
        a: SnailValue::from_pair(a),
        b: SnailValue::from_pair(b),
    };

    reduce_number(result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn read_str_test() {
        let a = SnailNumber::from_str("[1,2]");
        assert_eq!(a.a.number, Some(1));
        assert_eq!(a.b.number, Some(2));
        assert!(a.a.pair.is_none());
        assert!(a.b.pair.is_none());

        let b = SnailNumber::from_str("[[1,2],3]");
        assert_eq!(b.a.number, None);
        assert_eq!(b.b.number, Some(3));
        assert!(b.a.pair.is_some());
        assert_eq!(b.a.pair.as_ref().unwrap().a.number, Some(1));
        assert_eq!(b.a.pair.as_ref().unwrap().b.number, Some(2));
        assert!(b.b.pair.is_none());

        let _c =
            SnailNumber::from_str("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
    }

    #[test]
    fn to_str_test() {
        let numbers = vec![
            "[1,2]",
            "[[1,2],3]",
            "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]",
            "[[[[[9,80],1],2],3],4]",
        ];

        for number in numbers {
            let snail_number = SnailNumber::from_str(number);
            let res_number = snail_number.to_str();
            assert_eq!(res_number, number);
        }
    }

    #[test]
    fn explode_test() {
        assert_eq!(
            SnailNumber::from_str("[[[[[9,8],1],2],3],4]")
                .explode()
                .unwrap()
                .to_str(),
            "[[[[0,9],2],3],4]"
        );
        assert_eq!(
            SnailNumber::from_str("[7,[6,[5,[4,[3,2]]]]]")
                .explode()
                .unwrap()
                .to_str(),
            "[7,[6,[5,[7,0]]]]"
        );
        assert_eq!(
            SnailNumber::from_str("[[6,[5,[4,[3,2]]]],1]")
                .explode()
                .unwrap()
                .to_str(),
            "[[6,[5,[7,0]]],3]"
        );
        assert_eq!(
            SnailNumber::from_str("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")
                .explode()
                .unwrap()
                .to_str(),
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
        );
        assert_eq!(
            SnailNumber::from_str("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
                .explode()
                .unwrap()
                .to_str(),
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
        );
    }

    #[test]
    fn mixed_width_explode() {
        assert_eq!(
            SnailNumber::from_str("[[[[[9,80],1],2],3],4]")
                .explode()
                .unwrap()
                .to_str(),
            "[[[[0,81],2],3],4]"
        );
        assert_eq!(
            SnailNumber::from_str("[[3,[2,[8,0]]],[9,[5,[412,[3,2]]]]]")
                .explode()
                .unwrap()
                .to_str(),
            "[[3,[2,[8,0]]],[9,[5,[415,0]]]]"
        );
    }

    #[test]
    fn split_test() {
        let mut a = SnailNumber::from_str("[[[[0,7],4],[15,[0,13]]],[1,1]]");
        let res = a.split();
        assert!(res);
        assert_eq!(a.to_str(), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
    }

    #[test]
    fn reduce_test() {
        let init = SnailNumber::from_str("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        let reduce = reduce_number(init);
        assert_eq!(reduce.to_str(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn magnitude_test() {
        assert_eq!(
            SnailNumber::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
                .magnitude(),
            3488
        );
    }

    #[test]
    fn small_test() {
        assert_eq!(
            add_numbers_from_file("./inputs/day-18-input-test.txt").unwrap(),
            4140
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            add_numbers_from_file("./inputs/day-18-input.txt").unwrap(),
            4124
        );
    }

    #[test]
    fn part_2_small_test() {
        assert_eq!(
            find_largest_magnitude_from_file("./inputs/day-18-input-test.txt").unwrap(),
            3993
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            find_largest_magnitude_from_file("./inputs/day-18-input.txt").unwrap(),
            4673
        );
    }
}
