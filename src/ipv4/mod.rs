#![allow(dead_code)]

pub mod byte_reader;

use byte_reader::BudgetByteorder;

#[derive(Clone, Debug)]
pub struct V4Packet {
    version: u8,
    ihl: u8,
    length: u16,
    ttl: u8,
    proto: u8,
    hdr_checksum: u16,
    source: u32,
    dest: u32,
    pub data: Vec<u8>,
}

impl V4Packet {
    pub fn read_from_stream(buf: &mut impl BudgetByteorder) -> Option<Self> {
        let byte = buf.read_u8()?;

        let version = (byte & 0xF0) >> 4;
        let ihl = byte & 0x0F;

        buf.read_u8()?;

        let length = buf.read_u16()?;
        buf.read_u16()?;

        buf.read_u16()?;

        let ttl = buf.read_u8()?;
        let proto = buf.read_u8()?;
        let hdr_checksum = buf.read_u16()?;
        let source = buf.read_u32()?;
        let dest = buf.read_u32()?;

        let data_len = (length - (ihl as u16 * 32 / 8)) as usize;
        let data = buf.read_many_u8(data_len)?;

        Some(Self {
            version,
            ihl,
            length,
            ttl,
            proto,
            hdr_checksum,
            source,
            dest,
            data,
        })
    }
}

#[derive(Clone, Debug)]
struct UDP {
    source_port: u16,
    dest_port: u16,
    length: u16,
    checksum: u16,
    data: Vec<u8>,
}