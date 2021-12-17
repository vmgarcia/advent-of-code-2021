use std::collections::VecDeque;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::mem;
use std::path::Path;

fn convert_char_to_bits(ch: char) -> VecDeque<u8> {
    match ch {
        '0' => vec![0, 0, 0, 0].into_iter().collect(),
        '1' => vec![0, 0, 0, 1].into_iter().collect(),
        '2' => vec![0, 0, 1, 0].into_iter().collect(),
        '3' => vec![0, 0, 1, 1].into_iter().collect(),
        '4' => vec![0, 1, 0, 0].into_iter().collect(),
        '5' => vec![0, 1, 0, 1].into_iter().collect(),
        '6' => vec![0, 1, 1, 0].into_iter().collect(),
        '7' => vec![0, 1, 1, 1].into_iter().collect(),
        '8' => vec![1, 0, 0, 0].into_iter().collect(),
        '9' => vec![1, 0, 0, 1].into_iter().collect(),
        'A' => vec![1, 0, 1, 0].into_iter().collect(),
        'B' => vec![1, 0, 1, 1].into_iter().collect(),
        'C' => vec![1, 1, 0, 0].into_iter().collect(),
        'D' => vec![1, 1, 0, 1].into_iter().collect(),
        'E' => vec![1, 1, 1, 0].into_iter().collect(),
        'F' => vec![1, 1, 1, 1].into_iter().collect(),
        _ => vec![].into_iter().collect(),
    }
}

fn convert_bit_array_to_number(bits: VecDeque<u8>) -> u64 {
    let mut num = 0;
    for bit in bits {
        num = num * 2 + bit as u64;
    }
    num
}

fn parse_input(input: String) -> VecDeque<u8> {
    let mut parsed_input = VecDeque::new();
    for ch in input.chars() {
        parsed_input.append(&mut convert_char_to_bits(ch));
    }
    parsed_input
}

struct PacketStreamer {
    bits: VecDeque<u8>,
}

impl PacketStreamer {
    fn new(input: String) -> Self {
        Self {
            bits: parse_input(input),
        }
    }

    fn get_n_bits(&mut self, n: usize) -> VecDeque<u8> {
        let mut tmp = self.bits.split_off(n);
        mem::swap(&mut tmp, &mut self.bits);
        tmp
    }

    fn parse_packet(&mut self) -> (PacketType, u64) {
        let version = convert_bit_array_to_number(self.get_n_bits(3));
        let type_id = convert_bit_array_to_number(self.get_n_bits(3));
        let mut bits_used = 6;
        match type_id {
            4 => {
                let mut value_bits = VecDeque::new();
                loop {
                    let mut segment = self.get_n_bits(5);
                    let is_last_segment = segment.pop_front().unwrap_or(0) == 0;
                    value_bits.append(&mut segment);
                    bits_used += 5;
                    if is_last_segment {
                        break;
                    }
                }
                let value = convert_bit_array_to_number(value_bits);
                (PacketType::Literal(type_id, version, value), bits_used)
            }
            _ => {
                let length_type = self.get_n_bits(1).pop_front().unwrap_or(0);
                bits_used += 1;
                if length_type == 0 {
                    let subpacket_bit_count = convert_bit_array_to_number(self.get_n_bits(15));
                    bits_used += 15 + subpacket_bit_count;
                    let subpackets = self.parse_subpackets_using_bit_count(subpacket_bit_count);
                    (
                        PacketType::Operator(type_id, version, subpackets),
                        bits_used,
                    )
                } else {
                    let subpacket_count = convert_bit_array_to_number(self.get_n_bits(11));
                    let (subpackets, subpacket_bits) =
                        self.parse_subpackets_using_packet_count(subpacket_count);
                    bits_used += subpacket_bits + 11;
                    (
                        PacketType::Operator(type_id, version, subpackets),
                        bits_used,
                    )
                }
            }
        }
    }

    fn parse_subpackets_using_bit_count(&mut self, no_of_bits: u64) -> Vec<PacketType> {
        let mut subpackets = Vec::new();
        let mut remaining_bits = no_of_bits;
        while remaining_bits > 0 {
            let (subpacket, bits_used) = self.parse_packet();
            remaining_bits -= bits_used;
            subpackets.push(subpacket);
        }
        subpackets
    }

    fn parse_subpackets_using_packet_count(
        &mut self,
        no_of_packets: u64,
    ) -> (Vec<PacketType>, u64) {
        let mut subpackets = Vec::new();
        let mut total_bits_used = 0;
        for _ in 0..no_of_packets {
            let (subpacket, bits_used) = self.parse_packet();
            total_bits_used += bits_used;
            subpackets.push(subpacket);
        }
        (subpackets, total_bits_used)
    }

    fn flush_extra_bits(&mut self, bits_in_packet: usize) {
        let mut bits_to_flush = 0;
        while (bits_in_packet + bits_to_flush) % 4 != 0 {
            bits_to_flush += 1;
        }
        self.get_n_bits(bits_to_flush);
    }

    fn get_next_packet(&mut self) -> Option<PacketType> {
        if self.bits.len() < 24 {
            return None;
        }
        let version = convert_bit_array_to_number(self.get_n_bits(3));
        let type_id = convert_bit_array_to_number(self.get_n_bits(3));
        let mut bits_taken = 6;
        match type_id {
            4 => {
                let mut value_bits = VecDeque::new();
                loop {
                    let mut segment = self.get_n_bits(5);
                    bits_taken += 5;
                    let is_last_segment = segment.pop_front().unwrap_or(0) == 0;
                    value_bits.append(&mut segment);
                    if is_last_segment {
                        break;
                    }
                }
                let value = convert_bit_array_to_number(value_bits);
                // flush extra bits
                self.flush_extra_bits(bits_taken);

                Some(PacketType::Literal(type_id, version, value))
            }
            _ => {
                let length_type = self.get_n_bits(1).pop_front().unwrap_or(0);
                bits_taken += 1;
                if length_type == 0 {
                    let subpacket_bit_length = convert_bit_array_to_number(self.get_n_bits(15));
                    bits_taken += subpacket_bit_length as usize + 15;
                    let subpackets = self.parse_subpackets_using_bit_count(subpacket_bit_length);

                    // flush extra bits
                    self.flush_extra_bits(bits_taken);

                    Some(PacketType::Operator(type_id, version, subpackets))
                } else {
                    let no_of_packets = convert_bit_array_to_number(self.get_n_bits(11));
                    let (subpackets, subpacket_bits_used) =
                        self.parse_subpackets_using_packet_count(no_of_packets);
                    bits_taken += subpacket_bits_used as usize + 11;
                    // flush extra bits
                    self.flush_extra_bits(bits_taken);

                    Some(PacketType::Operator(type_id, version, subpackets))
                }
            }
        }
    }
}

type Version = u64;
type Value = u64;
type TypeId = u64;

#[derive(Debug)]
enum PacketType {
    Literal(TypeId, Version, Value),
    Operator(TypeId, Version, Vec<PacketType>),
}

impl Iterator for PacketStreamer {
    type Item = PacketType;

    fn next(&mut self) -> Option<PacketType> {
        if self.bits.len() > 0 {
            self.get_next_packet()
        } else {
            None
        }
    }
}

fn get_result(packet: PacketType) -> u64 {
    match packet {
        PacketType::Literal(_, _, value) => value,
        PacketType::Operator(type_id, _, subpackets) => match type_id {
            0 => subpackets
                .into_iter()
                .map(|subpacket| get_result(subpacket))
                .sum(),
            1 => subpackets
                .into_iter()
                .map(|subpacket| get_result(subpacket))
                .product(),
            2 => subpackets
                .into_iter()
                .map(|subpacket| get_result(subpacket))
                .min()
                .unwrap_or(0),
            3 => subpackets
                .into_iter()
                .map(|subpacket| get_result(subpacket))
                .max()
                .unwrap_or(0),
            5 => {
                let values = subpackets
                    .into_iter()
                    .map(|subpacket| get_result(subpacket))
                    .collect::<Vec<u64>>();
                if let (Some(a), Some(b)) = (values.get(0), values.get(1)) {
                    if *a > *b {
                        1
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            6 => {
                let values = subpackets
                    .into_iter()
                    .map(|subpacket| get_result(subpacket))
                    .collect::<Vec<u64>>();
                if let (Some(a), Some(b)) = (values.get(0), values.get(1)) {
                    if *a < *b {
                        1
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            7 => {
                let values = subpackets
                    .into_iter()
                    .map(|subpacket| get_result(subpacket))
                    .collect::<Vec<u64>>();
                if let (Some(a), Some(b)) = (values.get(0), values.get(1)) {
                    if *a == *b {
                        1
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            _ => 0,
        },
    }
}

pub fn day_16_problem_2() -> io::Result<u64> {
    let path_to_read = Path::new("./src/day-16-input.txt");
    let mut file = fs::File::open(&path_to_read)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    // file_contents = String::from("EE00D40C823060");

    let packet_streamer = PacketStreamer::new(file_contents);
    let mut total = 0;
    for packet in packet_streamer {
        return Ok(get_result(packet));
    }
    Ok(total)
}
