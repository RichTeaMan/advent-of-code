use std::{
    collections::VecDeque,
    io::{self},
};

use utils::file_utils::read_lines;

struct BitTransmission {
    version: i32,
    type_id: i32,

    literal: Option<i64>,
    sub_packets: Vec<BitTransmission>,
}

impl BitTransmission {
    fn sum_version(&self) -> i32 {
        self.version
            + self
                .sub_packets
                .iter()
                .map(|p| p.sum_version())
                .sum::<i32>()
    }

    fn evaluate(&self) -> i64 {
        match self.type_id {
            // sum
            0 => self.sub_packets.iter().map(|b| b.evaluate()).sum(),
            // product
            1 => self.sub_packets.iter().map(|b| b.evaluate()).product(),
            // minimum
            2 => self.sub_packets.iter().map(|b| b.evaluate()).min().unwrap(),
            // maximum
            3 => self.sub_packets.iter().map(|b| b.evaluate()).max().unwrap(),
            // literal
            4 => self.literal.unwrap(),
            // greater than
            5 => {
                debug_assert_eq!(
                    2,
                    self.sub_packets.len(),
                    "Greater than operator must have exactly 2 subs. Found {}.",
                    self.sub_packets.len()
                );
                if self.sub_packets.get(0).unwrap().evaluate()
                    > self.sub_packets.get(1).unwrap().evaluate()
                {
                    1
                } else {
                    0
                }
            }
            // less than
            6 => {
                debug_assert_eq!(
                    2,
                    self.sub_packets.len(),
                    "Less than operator must have exactly 2 subs. Found {}.",
                    self.sub_packets.len()
                );
                if self.sub_packets.get(0).unwrap().evaluate()
                    < self.sub_packets.get(1).unwrap().evaluate()
                {
                    1
                } else {
                    0
                }
            }
            // equality
            7 => {
                debug_assert_eq!(
                    2,
                    self.sub_packets.len(),
                    "Less than operator must have exactly 2 subs. Found {}.",
                    self.sub_packets.len()
                );
                if self.sub_packets.get(0).unwrap().evaluate()
                    == self.sub_packets.get(1).unwrap().evaluate()
                {
                    1
                } else {
                    0
                }
            }
            x => panic!("Unknown packet type: {}", x),
        }
    }
}

pub fn day_16() -> io::Result<i32> {
    sum_transmissions_version("./inputs/day-16-input.txt")
}
pub fn day_16_part_2() -> io::Result<i64> {
    evalutate_transmission("./inputs/day-16-input.txt")
}

fn sum_transmissions_version(filename: &str) -> io::Result<i32> {
    let transmission = load_transmissions(filename)?;

    Ok(transmission.sum_version())
}

fn evalutate_transmission(filename: &str) -> io::Result<i64> {
    let transmission = load_transmissions(filename)?;

    Ok(transmission.evaluate())
}

fn load_transmissions(filename: &str) -> io::Result<BitTransmission> {
    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }
        return Ok(bit_transmission_from_string(line.trim().to_string()));
    }
    unreachable!()
}

fn bit_transmission_from_string(s: String) -> BitTransmission {
    let mut bits = extract_bits_from_hex(s.as_str());
    bit_transmission_from_bits(&mut bits)
}

fn bit_transmission_from_bits(bits: &mut VecDeque<bool>) -> BitTransmission {
    let mut literal = None;
    let mut sub_packets = Vec::new();
    let version = read_number_from_bits(bits, 3);
    let type_id = read_number_from_bits(bits, 3);

    if type_id == 4 {

        let mut value = 0_i64;
        loop {
            let end = !read_bool_from_bits(bits);
            value = value << 4 | read_number_from_bits(bits, 4) as i64;
            if end {
                break;
            }
        }
        literal = Some(value);
    } else {
        let length_bit = read_bool_from_bits(bits);
        let digit_length = if length_bit { 11 } else { 15 };

        let length_value = read_number_from_bits(bits, digit_length);

        // length bit 0 means size of all sub packets
        if !length_bit {
            let mut sub_packet_bits = VecDeque::new();

            for _ in 0..length_value {
                sub_packet_bits.push_back(bits.pop_front().unwrap());
            }

            // expect subpacket(s) to exactly consume sub packet bits.
            while !sub_packet_bits.is_empty() {
                sub_packets.push(bit_transmission_from_bits(&mut sub_packet_bits));
            }
        }
        // digit length 1 means multiple packets, seemingly of unknown length
        else {
            for _ in 0..length_value {
                sub_packets.push(bit_transmission_from_bits(bits));
            }
        }
    }
    BitTransmission {
        version: version as i32,
        type_id: type_id as i32,
        literal,
        sub_packets,
    }
}

fn extract_bits_from_hex(s: &str) -> VecDeque<bool> {
    let mut bits = VecDeque::new();
    for i in s.chars() {
        let byte = u8::from_str_radix(&i.to_string(), 16).unwrap();
        bits.push_back(byte & 0b00001000 > 0);
        bits.push_back(byte & 0b00000100 > 0);
        bits.push_back(byte & 0b00000010 > 0);
        bits.push_back(byte & 0b00000001 > 0);
    }
    bits
}

fn read_number_from_bits(bits: &mut VecDeque<bool>, length: usize) -> usize {
    let mut value = 0;
    for _ in 0..length {
        let bit = bits.pop_front().unwrap();
        value <<= 1;
        if bit {
            value += 1;
        }
    }
    value
}

fn read_bool_from_bits(bits: &mut VecDeque<bool>) -> bool {
    bits.pop_front().unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn literal_test() {
        assert_eq!(
            bit_transmission_from_string("D2FE28".to_string())
                .literal
                .unwrap(),
            2021
        );

        assert_eq!(
            bit_transmission_from_string("38006F45291200".to_string()).version,
            1
        );

        let c = bit_transmission_from_string("EE00D40C823060".to_string());
        assert_eq!(c.sub_packets.len(), 3);
        let c_1 = c.sub_packets.get(0).unwrap();
        let c_2 = c.sub_packets.get(1).unwrap();
        let c_3 = c.sub_packets.get(2).unwrap();
        assert_eq!(c.version, 7);
        assert_eq!(c.type_id, 3);

        assert_eq!(c_1.literal, Some(1));
        assert_eq!(c_2.literal, Some(2));
        assert_eq!(c_3.literal, Some(3));

        let d = bit_transmission_from_string("8A004A801A8002F478".to_string());
        let d_sum = d.sum_version();
        assert_eq!(16, d_sum);

        let e = bit_transmission_from_string("620080001611562C8802118E34".to_string());
        let e_sum = e.sum_version();
        assert_eq!(12, e_sum);

        let f = bit_transmission_from_string("C0015000016115A2E0802F182340".to_string());
        let f_sum = f.sum_version();
        assert_eq!(23, f_sum);

        let g = bit_transmission_from_string("A0016C880162017C3686B18A3D4780".to_string());
        let g_sum = g.sum_version();
        assert_eq!(31, g_sum);
    }

    #[test]
    fn small_test() {
        assert_eq!(
            sum_transmissions_version("./inputs/day-16-input-test.txt").unwrap(),
            31
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            sum_transmissions_version("./inputs/day-16-input.txt").unwrap(),
            986
        );
    }

    #[test]
    fn expression_test() {
        assert_eq!(
            bit_transmission_from_string("C200B40A82".to_string()).evaluate(),
            3
        );
        assert_eq!(
            bit_transmission_from_string("04005AC33890".to_string()).evaluate(),
            54
        );
        assert_eq!(
            bit_transmission_from_string("880086C3E88112".to_string()).evaluate(),
            7
        );
        assert_eq!(
            bit_transmission_from_string("CE00C43D881120".to_string()).evaluate(),
            9
        );
        assert_eq!(
            bit_transmission_from_string("D8005AC2A8F0".to_string()).evaluate(),
            1
        );
        assert_eq!(
            bit_transmission_from_string("F600BC2D8F".to_string()).evaluate(),
            0
        );
        assert_eq!(
            bit_transmission_from_string("9C005AC2F8F0".to_string()).evaluate(),
            0
        );
        assert_eq!(
            bit_transmission_from_string("9C0141080250320F1802104A08".to_string()).evaluate(),
            1
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            evalutate_transmission("./inputs/day-16-input.txt").unwrap(),
            18234816469452
        );
    }
}
