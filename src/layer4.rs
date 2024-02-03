use std::iter;

use crate::ipv4::{byte_reader::ByteReader, dotted_to_decimal, V4Packet};

struct Criteria {
    ip_from: u32,
    ip_to: u32,
    port_to: u16,
}

pub fn process(bytes: String) -> String {
    let bytes: Vec<u8> = bytes.chars().map(|ch| ch as u8).collect();
    let mut buf = ByteReader::new(&bytes);

    let criteria = Criteria {
        ip_from: dotted_to_decimal([10, 1, 1, 10]),
        ip_to: dotted_to_decimal([10, 1, 1, 200]),
        port_to: 42069,
    };

    let packets: Vec<_> = iter::from_fn(|| V4Packet::read_from_stream(&mut buf))
        .filter(|packet| is_valid(&packet, &criteria))
        // .map(|packet| format!("{packet:?}"))
        .map(|packet| packet.udp.data)
        .map(|data| String::from_utf8_lossy(&data).to_string())
        .collect();
    
    format!("{packets:#?}")
}

fn is_valid(packet: &V4Packet, criteria: &Criteria) -> bool {
    packet.source == criteria.ip_from 
    && packet.udp.dest_port == criteria.port_to 
    && packet.dest == criteria.ip_to
}