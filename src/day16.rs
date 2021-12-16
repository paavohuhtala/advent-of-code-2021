use std::io::Cursor;

use anyhow::Context;
use bitstream_io::{BigEndian, BitRead, BitReader};

const INPUT: &str = include_str!("./day16.txt");

type Literal = u64;

fn read_input() -> Vec<u8> {
    hex::decode(INPUT).unwrap()
}

#[derive(Debug)]
struct Packet {
    version: u8,
    data: PacketData,
}

impl Packet {
    fn get_version_number_sum(&self) -> usize {
        let inner = match &self.data {
            PacketData::Literal(_) => 0,
            PacketData::Operator(_, inner) => {
                inner.iter().map(|x| x.get_version_number_sum()).sum()
            }
        };

        inner + self.version as usize
    }

    fn eval(&self) -> Literal {
        match &self.data {
            PacketData::Literal(i) => *i,
            PacketData::Operator(op, packets) => {
                let op_fn: fn(Literal, Literal) -> Literal = match op {
                    Operator::Sum => |a, b| a + b,
                    Operator::Product => |a, b| a * b,
                    Operator::Minimum => |a, b| a.min(b),
                    Operator::Maximum => |a, b| a.max(b),
                    Operator::GreaterThan => |a, b| (a > b) as Literal,
                    Operator::LessThan => |a, b| (a < b) as Literal,
                    Operator::EqualTo => |a, b| (a == b) as Literal,
                };

                packets
                    .iter()
                    .map(|a| a.eval())
                    .reduce(|a, b| op_fn(a, b))
                    .unwrap()
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug)]
enum PacketData {
    Literal(Literal),
    Operator(Operator, Vec<Packet>),
}

fn read_variable_int<R: std::io::Read>(
    reader: &mut BitReader<R, BigEndian>,
) -> anyhow::Result<(Literal, usize)> {
    let mut parts = Vec::new();

    loop {
        let has_more_groups = reader.read_bit()?;
        let part: u8 = reader.read(4)?;
        parts.push(part);

        if !has_more_groups {
            break;
        }
    }

    let total_bit_length = parts.len() * 5;

    let mut result: Literal = 0;
    let mut shift = 0;

    for part in parts.into_iter().rev() {
        result |= (part as Literal) << shift;
        shift += 4;
    }

    Ok((result, total_bit_length))
}

fn parse_operator(op: u8) -> anyhow::Result<Operator> {
    match op {
        0 => Ok(Operator::Sum),
        1 => Ok(Operator::Product),
        2 => Ok(Operator::Minimum),
        3 => Ok(Operator::Maximum),
        5 => Ok(Operator::GreaterThan),
        6 => Ok(Operator::LessThan),
        7 => Ok(Operator::EqualTo),
        _ => Err(anyhow::anyhow!("Unknown operator: {}", op)),
    }
}

fn read_packet<R: std::io::Read>(
    reader: &mut BitReader<R, BigEndian>,
) -> anyhow::Result<(Packet, usize)> {
    let mut length = 0;
    let version: u8 = reader.read(3).context("reading version")?;
    length += 3;

    let type_id: u8 = reader.read(3).context("reading type id")?;
    length += 3;

    let packet_data = match type_id {
        4 => {
            let (literal, literal_length) =
                read_variable_int(reader).context("reading variable integer")?;
            length += literal_length;
            PacketData::Literal(literal)
        }
        otherwise => {
            let op = parse_operator(otherwise)?;
            let has_number_of_sub_packets = reader.read_bit()?;
            length += 1;
            let mut sub_packets = Vec::new();

            if has_number_of_sub_packets {
                let number_of_sub_packets: u16 = reader.read(11)?;
                length += 11;

                for _ in 0..number_of_sub_packets {
                    let (packet, packet_length) = read_packet(reader)?;
                    sub_packets.push(packet);
                    length += packet_length;
                }
            } else {
                let total_length_of_sub_packets: u16 = reader.read(15)?;
                length += 15;
                let mut total_read_sub = 0;

                while total_read_sub < (total_length_of_sub_packets as usize) {
                    let (packet, packet_length) = read_packet(reader)?;
                    sub_packets.push(packet);
                    total_read_sub += packet_length;
                }

                length += total_length_of_sub_packets as usize;
            }

            PacketData::Operator(op, sub_packets)
        }
    };

    Ok((
        Packet {
            version,
            data: packet_data,
        },
        length,
    ))
}

pub fn ab() {
    let data = read_input();
    let mut cursor = Cursor::new(data);
    let mut reader = BitReader::endian(&mut cursor, BigEndian);
    let (packet, _) = read_packet(&mut reader).unwrap();
    println!("Day16a: {}", packet.get_version_number_sum());
    println!("Day16b: {}", packet.eval());
}
