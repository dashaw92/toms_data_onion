#![allow(dead_code)]

pub mod byte_reader;

use byte_reader::BudgetByteorder;

#[derive(Clone, Debug)]
pub struct V4Packet {
    version: u8,
    ihl: u8,
    type_of_svc: u8,
    length: u16,
    ident: u16,
    flags: u8,
    frag_offset: u16,
    ttl: u8,
    proto: u8,
    hdr_checksum: u16,
    source: u32,
    dest: u32,
    //options: ???,
    data: Vec<u8>,
}

impl V4Packet {
    pub fn read_from_stream(buf: &mut impl BudgetByteorder) -> Self {
        let byte = buf.read_u8();

        let version = byte & 0xF0 >> 3;
        let ihl = byte & 0x0F;
        let type_of_svc = buf.read_u8();
        let length = buf.read_u16();
        let ident = buf.read_u16();
        let flags = buf.read_u8();
        let frag_offset = buf.read_u16();
        let ttl = buf.read_u8();
        let proto = buf.read_u8();
        let hdr_checksum = buf.read_u16();
        let source = buf.read_u32();
        let dest = buf.read_u32();
        let data = buf.read_many_u8(length as usize * 8);

        Self {
            version,
            ihl,
            type_of_svc,
            length,
            ident,
            flags,
            frag_offset,
            ttl,
            proto,
            hdr_checksum,
            source,
            dest,
            data,
        }
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