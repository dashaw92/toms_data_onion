use std::iter;

use crate::ipv4::{V4Packet, byte_reader::ByteReader};

pub fn process(bytes: String) -> String {
    let bytes: Vec<u8> = bytes.chars().map(|ch| ch as u8).collect();
    let mut buf = ByteReader::new(&bytes);

    let packets: Vec<_> = iter::from_fn(|| V4Packet::read_from_stream(&mut buf))
        .filter(|packet| is_valid(&packet))
        .map(|packet| format!("{packet:?}"))
        // .map(|packet| packet.data)
        // .map(|data| String::from_utf8_lossy(&data).to_string())
        .collect();
    
    format!("{packets:#?}")
}

fn is_valid(_: &V4Packet) -> bool {
    true
}