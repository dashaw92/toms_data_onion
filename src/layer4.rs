use crate::ipv4::{V4Packet, byte_reader::ByteReader};

pub fn process(input: String) -> String {
    let bytes: Vec<u8> = input.chars().map(|ch| ch as u8).collect();
    let mut buf = ByteReader::new(&bytes);

    let first = V4Packet::read_from_stream(&mut buf);
    let next = V4Packet::read_from_stream(&mut buf);
    dbg!(first);
    dbg!(next);
    input
}