use core::fmt;
use std::{cmp::Ordering, io};

use itertools::Itertools;

use crate::file_utils::read_lines;

struct Packet {
    pub value: Option<i32>,
    pub list: Vec<Packet>,
}

impl Packet {
    pub fn fetch_str(&self) -> String {
        if let Some(v) = self.value {
            format!("{v}")
        } else {
            let strs = self.list.iter().map(|p| p.fetch_str()).collect_vec();
            format!("[{s}]", s = strs.join(","))
        }
    }

    pub fn from_str(value: &str) -> Packet {
        let mut char_buffer = Vec::new();

        for c in value.chars() {
            char_buffer.push(c);
        }
        let mut packets = Packet::from_vec(char_buffer);
        assert_eq!(1, packets.len());
        let p = packets.swap_remove(0);
        return p;
    }

    pub fn from_vec(char_buffer: Vec<char>) -> Vec<Packet> {
        let mut list = Vec::new();

        let mut number_buffer = "".to_string();
        let mut char_i = 0;
        while char_i < char_buffer.len() {
            let c = char_buffer[char_i];

            match c {
                '[' => {
                    // find ]
                    let mut pair = 0;
                    let mut slice = Vec::new();
                    loop {
                        char_i += 1;
                        let f_char = char_buffer[char_i];
                        if f_char == '[' {
                            pair += 1;
                        } else if f_char == ']' {
                            if pair == 0 {
                                break;
                            }
                            pair -= 1;
                        }
                        slice.push(f_char);
                    }
                    let inner_packets = Packet::from_vec(slice);

                    list.push(Packet {
                        value: None,
                        list: inner_packets,
                    });
                    char_i += 1;
                }
                ',' => {
                    if let Ok(num_value) = number_buffer.parse::<i32>() {
                        list.push(Packet::from_number(num_value));
                        number_buffer.clear();
                    } else {
                        // probably fine, moving on from array termination.
                        //panic!("Cannot parse number: '{number_buffer}'.");
                    }
                }
                other => {
                    if other.is_ascii_digit() {
                        number_buffer.push_str(&other.to_string());
                    } else {
                        panic!("Unexpected value: '{other}'.");
                    }
                }
            }
            char_i += 1;
        }
        if !number_buffer.is_empty() {
            if let Ok(num_value) = number_buffer.parse::<i32>() {
                list.push(Packet::from_number(num_value));
                number_buffer.clear();
            } else {
                panic!("Cannot parse number: '{number_buffer}'.");
            }
        }
        list
    }

    pub fn from_number(value: i32) -> Packet {
        Packet {
            value: Some(value),
            list: Vec::new(),
        }
    }
}

impl PartialEq for Packet {
    // needed for partial ord. I have no plans to implement this.
    fn eq(&self, _other: &Self) -> bool {
        todo!();
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.value.is_some() && other.value.is_some() {
            return Some(self.value.unwrap().cmp(&other.value.unwrap()));
        } else if let Some(v) = self.value {
            let list_a = Packet {
                value: None,
                list: vec![Packet::from_number(v)],
            };
            return Some(cmp_array(&list_a, other));
        } else if let Some(v) = other.value {
            let list_b = Packet {
                value: None,
                list: vec![Packet::from_number(v)],
            };
            return Some(cmp_array(self, &list_b));
        }

        Some(cmp_array(self, other))
    }
}

fn cmp_array(a: &Packet, b: &Packet) -> Ordering {
    let max_index = a.list.len().min(b.list.len());
    for i in 0..max_index {
        if let Some(order) = a.list[i].partial_cmp(&b.list[i]) {
            if order.is_ne() {
                return order;
            }
        }
    }
    if a.list.len() < b.list.len() {
        return Ordering::Less;
    }
    if a.list.len() > b.list.len() {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{d}", d = self.fetch_str())
    }
}

fn compare_sets_from_file(filename: &str) -> io::Result<i32> {
    let lines = read_lines(filename)?;

    let mut result = 0;

    let mut a = None;
    let mut b = None;
    let mut index = 1;
    for line in lines.flatten() {
        if line.is_empty() {
            let aw = a.unwrap();
            let bw = b.unwrap();

            if aw < bw {
                result += index;
            }

            a = None;
            b = None;
            index += 1;

            continue;
        }

        if a.is_none() {
            let p = Packet::from_str(line.as_str());
            assert!(p.fetch_str() == line.as_str());
            a = Some(p);
        } else {
            let p = Packet::from_str(line.as_str());
            assert!(p.fetch_str() == line.as_str());
            b = Some(p);
        }
    }
    Ok(result)
}

pub fn day_13() -> io::Result<i32> {
    compare_sets_from_file("./inputs/day-13-input.txt")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn packet_test() {
        let a = Packet::from_str("[9]");

        let b = Packet::from_str("[1,[2,3,4]]");

        let c = Packet::from_str("[17,512]");

        assert_eq!(1, a.list.len());
        assert_eq!(Some(9), a.list[0].value);

        assert_eq!(2, b.list.len());
        assert_eq!(0, b.list[0].list.len());
        assert_eq!(3, b.list[1].list.len());
        assert_eq!(Some(1), b.list[0].value);
        assert_eq!(None, b.list[1].value);
        assert_eq!(3, b.list[1].list.len());
        assert_eq!(Some(2), b.list[1].list[0].value);
        assert_eq!(0, b.list[1].list[0].list.len());
        assert_eq!(Some(3), b.list[1].list[1].value);
        assert_eq!(0, b.list[1].list[1].list.len());
        assert_eq!(Some(4), b.list[1].list[2].value);
        assert_eq!(0, b.list[1].list[2].list.len());

        assert_eq!(2, c.list.len());
        assert_eq!(Some(17), c.list[0].value);
        assert_eq!(Some(512), c.list[1].value);
    }

    #[test]
    fn compare_test() {
        let res = Packet::from_str("[1,1,3,1,1]").partial_cmp(&Packet::from_str("[1,1,5,1,1]"));

        if let Some(r) = res {
            println!("{r:?}");
        } else {
            panic!("no result");
        }
        assert!(Packet::from_str("[1,1,3,1,1]") < Packet::from_str("[1,1,5,1,1]"));
        assert!(Packet::from_str("[[1],[2,3,4]]") < Packet::from_str("[[1],4]"));
        assert!(Packet::from_str("[9]") > Packet::from_str("[[8,7,6]]"));
        assert!(Packet::from_str("[[4,4],4,4]") < Packet::from_str("[[4,4],4,4,4]"));
        assert!(Packet::from_str("[7,7,7,7]") > Packet::from_str("[7,7,7]"));
        assert!(Packet::from_str("[]") < Packet::from_str("[3]"));
        assert!(Packet::from_str("[[[]]]") > Packet::from_str("[[]]"));
        assert!(
            Packet::from_str("[1,[2,[3,[4,[5,6,7]]]],8,9]")
                > Packet::from_str("[1,[2,[3,[4,[5,6,0]]]],8,9]")
        );
        assert!(Packet::from_str("[5,6,7]") > Packet::from_str("[5,6,0]"));
        assert!(Packet::from_str("[[4,[4,[5],[7,2],[4,0,3],[]],[[10,8,1]]],[5,[[1,1,7,3],[],[8,10,6,5,4],0,9],[[3,5,6,10],[10,0,2,10],5,[1,6,7]],2],[],[8,4,0,5]]") <
Packet::from_str("[[[5,[8,8,6,4,4],[2,9,0,9]],6,[],[[4,0],[],0]],[],[],[8,3]]"));
        assert!(
            Packet::from_str("[4,[4,[5],[7,2],[4,0,3],[]],[[10,8,1]]]")
                < Packet::from_str("[[5,[8,8,6,4,4],[2,9,0,9]],6,[],[[4,0],[],0]]")
        );

        assert!(Packet::from_str("[4]",) < Packet::from_str("[5,[8,8,6,4,4],[2,9,0,9]]"));
    }

    #[test]
    fn small_test() {
        assert_eq!(
            compare_sets_from_file("./inputs/day-13-input-test.txt").unwrap(),
            13
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            compare_sets_from_file("./inputs/day-13-input.txt").unwrap(),
            6046
        );
    }
}
