#![allow(dead_code)]

pub mod byte_reader;

use byte_reader::BudgetByteorder;

use crate::ipv4::byte_reader::ByteReader;

#[derive(Clone, Debug)]
pub struct V4Packet {
    pub version: u8,
    pub ihl: u8,
    pub length: u16,
    pub computed_checksum: u16,
    pub hdr_checksum: u16,
    pub source: u32,
    pub dest: u32,
    pub udp: UDP,
}

impl V4Packet {
    pub fn read_from_stream(buf: &mut impl BudgetByteorder) -> Option<Self> {
        let mut header: Vec<u16> = buf.read_many_u8(20)?.chunks(2)
            .map(|bytes| (bytes[0] as u16) << 8 | bytes[1] as u16)
            .collect();
        header[5] = 0; //don't consider the checksum in the packet when computing the checksum
        let computed_checksum: u16 = !header.into_iter().fold(0, |acc, next| {
            let (sum, carry) = acc.overflowing_add(next);
            if carry {
                sum + 1
            } else {
                sum
            }
        });
        buf.seek(-20).ok()?;

        let byte = buf.read_u8()?;

        let version = (byte & 0xF0) >> 4;
        let ihl = byte & 0x0F;

        buf.read_u8()?;

        let length = buf.read_u16()?;

        buf.read_u16()?;
        buf.read_u16()?;
        buf.read_u8()?;
        buf.read_u8()?;
        
        let hdr_checksum = buf.read_u16()?;
        let source = buf.read_u32()?;
        let dest = buf.read_u32()?;

        let data_len = (length - (ihl as u16 * 32 / 8)) as usize;
        let udp = UDP::parse(buf.read_many_u8(data_len)?)?;


        Some(Self {
            version,
            ihl,
            length,
            computed_checksum,
            hdr_checksum,
            source,
            dest,
            udp,
        })
    }
}

pub fn dotted_to_decimal(ip: [u8; 4]) -> u32 {
    ip.iter()
        .enumerate()
        .map(|(idx, &octet)| octet as u32 * 256u32.pow(3 - idx as u32))
        .sum()
}

#[derive(Clone, Debug)]
pub struct UDP {
    pub source_port: u16,
    pub dest_port: u16,
    pub length: u16,
    pub checksum: u16,
    pub data: Vec<u8>,
}

impl UDP {
    fn parse(bytes: Vec<u8>) -> Option<Self> {
        let mut reader = ByteReader::new(&bytes);

        let source_port = reader.read_u16()?;
        let dest_port = reader.read_u16()?;
        let length = reader.read_u16()?;
        let checksum = reader.read_u16()?;

        let data = reader.read_many_u8(length as usize - 8)?;
        
        Some(UDP {
            source_port,
            dest_port,
            length,
            checksum,
            data,
        })
    }
}