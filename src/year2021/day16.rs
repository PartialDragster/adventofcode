use crate::utils::utils;

trait Packet : std::fmt::Debug { 
    fn sum_version_numbers(&self) -> u32;

    fn evaluate(&self) -> u64;
}

#[derive(Debug)]
struct LiteralValue {
    version: u32,
    packet_type_id: u32,
    literal_value: u64,
}

#[derive(Debug)]
struct SumPacket {
    version: u32,
    packet_type_id: u32,
    sub_packets: Vec<Box<dyn Packet>>,
}

#[derive(Debug)]
struct ProductPacket {
    version: u32,
    packet_type_id: u32,
    sub_packets: Vec<Box<dyn Packet>>,
}

#[derive(Debug)]
struct MinimumPacket {
    version: u32,
    packet_type_id: u32,
    sub_packets: Vec<Box<dyn Packet>>,
}

#[derive(Debug)]
struct MaximumPacket {
    version: u32,
    packet_type_id: u32,
    sub_packets: Vec<Box<dyn Packet>>,
}

#[derive(Debug)]
struct GreaterThanPacket {
    version: u32,
    packet_type_id: u32,
    sub_packets: Vec<Box<dyn Packet>>,
}

#[derive(Debug)]
struct LessThanPacket {
    version: u32,
    packet_type_id: u32,
    sub_packets: Vec<Box<dyn Packet>>,
}

#[derive(Debug)]
struct EqualToPacket {
    version: u32,
    packet_type_id: u32,
    sub_packets: Vec<Box<dyn Packet>>,
}

impl LiteralValue {
    fn new(version: u32, packet_type_id: u32, literal_value: u64) -> Self {
        LiteralValue { version, packet_type_id, literal_value }
    }
}

impl SumPacket {
    fn new(version: u32, packet_type_id: u32, sub_packets: Vec<Box<dyn Packet>>) -> Self {
        SumPacket { version, packet_type_id, sub_packets }
    }
}

impl ProductPacket {
    fn new(version: u32, packet_type_id: u32, sub_packets: Vec<Box<dyn Packet>>) -> Self {
        ProductPacket { version, packet_type_id, sub_packets }
    }
}

impl MinimumPacket {
    fn new(version: u32, packet_type_id: u32, sub_packets: Vec<Box<dyn Packet>>) -> Self {
        MinimumPacket { version, packet_type_id, sub_packets }
    }
}

impl MaximumPacket {
    fn new(version: u32, packet_type_id: u32, sub_packets: Vec<Box<dyn Packet>>) -> Self {
        MaximumPacket { version, packet_type_id, sub_packets }
    }
}

impl GreaterThanPacket {
    fn new(version: u32, packet_type_id: u32, sub_packets: Vec<Box<dyn Packet>>) -> Self {
        GreaterThanPacket { version, packet_type_id, sub_packets }
    }
}

impl LessThanPacket {
    fn new(version: u32, packet_type_id: u32, sub_packets: Vec<Box<dyn Packet>>) -> Self {
        LessThanPacket { version, packet_type_id, sub_packets }
    }
}

impl EqualToPacket {
    fn new(version: u32, packet_type_id: u32, sub_packets: Vec<Box<dyn Packet>>) -> Self {
        EqualToPacket { version, packet_type_id, sub_packets }
    }
}

impl Packet for LiteralValue {
    fn sum_version_numbers(&self) -> u32 {
        self.version
    }

    fn evaluate(&self) -> u64 {
        self.literal_value
    }
}

impl Packet for SumPacket {
    fn sum_version_numbers(&self) -> u32 {
        let mut sum = self.version;
        for packet in &self.sub_packets {
            sum += packet.sum_version_numbers();
        }
        sum
    }

    fn evaluate(&self) -> u64 {
        self.sub_packets
            .iter()
            .map(|sub_packet| sub_packet.evaluate())
            .sum()
    }
}

impl Packet for ProductPacket {
    fn sum_version_numbers(&self) -> u32 {
        let mut sum = self.version;
        for packet in &self.sub_packets {
            sum += packet.sum_version_numbers();
        }
        sum
    }

    fn evaluate(&self) -> u64 {
        self.sub_packets
            .iter()
            .map(|sub_packet| sub_packet.evaluate())
            .product()
    }
}

impl Packet for MinimumPacket {
    fn sum_version_numbers(&self) -> u32 {
        let mut sum = self.version;
        for packet in &self.sub_packets {
            sum += packet.sum_version_numbers();
        }
        sum
    }

    fn evaluate(&self) -> u64 {
        self.sub_packets
            .iter()
            .map(|sub_packet| sub_packet.evaluate())
            .min()
            .unwrap()
    }
}

impl Packet for MaximumPacket {
    fn sum_version_numbers(&self) -> u32 {
        let mut sum = self.version;
        for packet in &self.sub_packets {
            sum += packet.sum_version_numbers();
        }
        sum
    }

    fn evaluate(&self) -> u64 {
        self.sub_packets
            .iter()
            .map(|sub_packet| sub_packet.evaluate())
            .max()
            .unwrap()
    }
}

impl Packet for GreaterThanPacket {
    fn sum_version_numbers(&self) -> u32 {
        let mut sum = self.version;
        for packet in &self.sub_packets {
            sum += packet.sum_version_numbers();
        }
        sum
    }

    fn evaluate(&self) -> u64 {
        if self.sub_packets[0].evaluate() > self.sub_packets[1].evaluate() {
            1
        } else { 
            0
        }
    }
}

impl Packet for LessThanPacket {
    fn sum_version_numbers(&self) -> u32 {
        let mut sum = self.version;
        for packet in &self.sub_packets {
            sum += packet.sum_version_numbers();
        }
        sum
    }

    fn evaluate(&self) -> u64 {
        if self.sub_packets[0].evaluate() < self.sub_packets[1].evaluate() {
            1
        } else { 
            0
        }
    }
}

impl Packet for EqualToPacket {
    fn sum_version_numbers(&self) -> u32 {
        let mut sum = self.version;
        for packet in &self.sub_packets {
            sum += packet.sum_version_numbers();
        }
        sum
    }

    fn evaluate(&self) -> u64 {
        if self.sub_packets[0].evaluate() == self.sub_packets[1].evaluate() {
            1
        } else { 
            0
        }
    }
}


fn convert_hex_to_binary(hex_string: &str) -> Vec<u8> {
    hex_string.chars()
        .map(|ch| match ch {
            '0' => vec![0, 0, 0, 0],
            '1' => vec![0, 0, 0, 1],
            '2' => vec![0, 0, 1, 0],
            '3' => vec![0, 0, 1, 1],
            '4' => vec![0, 1, 0, 0],
            '5' => vec![0, 1, 0, 1],
            '6' => vec![0, 1, 1, 0],
            '7' => vec![0, 1, 1, 1],
            '8' => vec![1, 0, 0, 0],
            '9' => vec![1, 0, 0, 1],
            'A' => vec![1, 0, 1, 0],
            'B' => vec![1, 0, 1, 1],
            'C' => vec![1, 1, 0, 0],
            'D' => vec![1, 1, 0, 1],
            'E' => vec![1, 1, 1, 0],
            'F' => vec![1, 1, 1, 1],
            _ => panic!("Unrecognised letter: {}", ch),
        })
        .flatten()
        .collect()
}

fn read_n_bytes_as_number<'a, I: 'a>(iter: &mut I, n: usize) -> Option<u32> where 
    I: Iterator<Item=&'a u8> 
{
    let mut num = 0;
    for _ in 0..n {
        num <<= 1;
        num += *iter.next()? as u32;
    }
    Some(num)
}

fn parse_literal_value<'a, I: 'a>(iter: &mut I) -> Option<u64> where
    I: Iterator<Item=&'a u8>
{
    let mut number = 0;
    let mut read_group = 1;
    while read_group != 0 {
        read_group = read_n_bytes_as_number(iter, 1)?;
        number <<= 4;
        number += read_n_bytes_as_number(iter, 4)? as u64;
    }
    Some(number)
}

fn parse_operator<'a, I: 'a>(iter: &mut I) -> Option<Vec<Box<dyn Packet>>> where
    I: Iterator<Item=&'a u8>
{
    // parse length type id
    let length_type_id = read_n_bytes_as_number(iter, 1)?;

    match length_type_id {
        // if the length type ID is 0 the next 15 bits are a number that
        // represents the total length in bits of the sub_packets contained
        // by this packet
        0 => {
            let mut packets = vec![];
            let sub_packets_bit_length = read_n_bytes_as_number(iter, 15)?;
            let sub_packets: Vec<u8> = iter.take(sub_packets_bit_length as usize)
                .map(|n| *n)
                .collect();
            let mut sub_packets_iter = sub_packets.iter();
            while let Some(packet) = parse_recurse(&mut sub_packets_iter) {
                packets.push(packet);
            }
            Some(packets)
        },
        1 => {
            let mut packets = vec![];
            let sub_packets_count = read_n_bytes_as_number(iter, 11)?;
            for _ in 0..sub_packets_count {
                let packet = parse_recurse(iter)?;
                packets.push(packet);
            }
            Some(packets)
        },
        _ => panic!("unrecognised length_type_id {}", length_type_id),
    }
}

fn parse(binary: &Vec<u8>) -> Option<Box<dyn Packet>> {
    parse_recurse(&mut binary.iter())
}

fn parse_recurse<'a, I: 'a>(iter: &mut I) -> Option<Box<dyn Packet>> where
    I: Iterator<Item=&'a u8>
{
    // first 3 bytes are always the packet version
    let version = read_n_bytes_as_number(iter, 3)?;

    // next 3 bytes are packet type ID
    let packet_type_id = read_n_bytes_as_number(iter, 3)?; 

    match packet_type_id {
        0 => Some(Box::new(SumPacket::new(version, packet_type_id, parse_operator(iter)?))),
        1 => Some(Box::new(ProductPacket::new(version, packet_type_id, parse_operator(iter)?))),
        2 => Some(Box::new(MinimumPacket::new(version, packet_type_id, parse_operator(iter)?))),
        3 => Some(Box::new(MaximumPacket::new(version, packet_type_id, parse_operator(iter)?))),
        4 => Some(Box::new(LiteralValue::new(version, packet_type_id, parse_literal_value(iter)?))),
        5 => Some(Box::new(GreaterThanPacket::new(version, packet_type_id, parse_operator(iter)?))),
        6 => Some(Box::new(LessThanPacket::new(version, packet_type_id, parse_operator(iter)?))),
        7 => Some(Box::new(EqualToPacket::new(version, packet_type_id, parse_operator(iter)?))),
        _ => panic!("unrecognised packet_type_id: {}", packet_type_id),
    }
}

pub fn run() {
    let hex_string = utils::read_file_to_string("data/year2021/day16").trim_end().to_string();
    let binary = convert_hex_to_binary(&hex_string);
    let packet = parse(&binary).unwrap();
    println!("{}", packet.sum_version_numbers());
    println!("{}", packet.evaluate());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2021_day16() {
        let hex_string = "8A004A801A8002F478";
        let binary = convert_hex_to_binary(hex_string);
        let packet = parse(&binary).unwrap();
        assert_eq!(packet.sum_version_numbers(), 16);

        let hex_string = "620080001611562C8802118E34";
        let binary = convert_hex_to_binary(hex_string);
        let packet = parse(&binary).unwrap();
        assert_eq!(packet.sum_version_numbers(), 12);

        let hex_string = "C0015000016115A2E0802F182340";
        let binary = convert_hex_to_binary(hex_string);
        let packet = parse(&binary).unwrap();
        assert_eq!(packet.sum_version_numbers(), 23);

        let hex_string = "A0016C880162017C3686B18A3D4780";
        let binary = convert_hex_to_binary(hex_string);
        let packet = parse(&binary).unwrap();
        assert_eq!(packet.sum_version_numbers(), 31);
    }
}
