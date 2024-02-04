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

fn u8stou16s(bytes: &[u8]) -> Vec<u16> {
    bytes.chunks(2)
        .map(|bytes| {
            let msb = bytes[0];
            let lsb;
            if bytes.len() != 2 {
                lsb = 0;
            } else {
                lsb = bytes[1];
            }

            (msb as u16) << 8 | lsb as u16
        })
        .collect()
}

fn calc_checksum(bytes: impl Iterator<Item = u16>) -> u16 {
    !bytes.fold(0, |acc, next| {
        let (sum, carry) = acc.overflowing_add(next);
        if carry {
            sum + 1
        } else {
            sum
        }
    })
}

impl V4Packet {
    pub fn read_from_stream(buf: &mut impl BudgetByteorder) -> Option<Self> {
        let computed_checksum: u16 = {
            let mut header = u8stou16s(&buf.read_many_u8(20)?);
            header[5] = 0; //don't consider the checksum from the packet
            calc_checksum(header.into_iter())
        };
        buf.seek(-20).ok()?;

        let version_ihl = buf.read_u8()?;

        let version = (version_ihl & 0xF0) >> 4;
        let ihl = version_ihl & 0xF;

        //Not needed
        buf.read_u8()?;

        let length = buf.read_u16()?;

        //Not needed
        buf.read_u16()?;
        buf.read_u16()?;
        buf.read_u8()?;
        let protocol = buf.read_u8()?;
        
        let hdr_checksum = buf.read_u16()?;
        let source = buf.read_u32()?;
        let dest = buf.read_u32()?;

        //Used to calculate the UDP checksum
        let udp_header = PsuedoHeader {
            src_ip: source,
            dst_ip: dest,
            reserved: 0,
            protocol,
            length
        };

        let data_len = (length - (ihl as u16 * 32 / 8)) as usize;
        let udp = UDP::parse(&udp_header, buf.read_many_u8(data_len)?)?;


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
struct PsuedoHeader {
    src_ip: u32,
    dst_ip: u32,
    reserved: u8,
    protocol: u8,
    length: u16,
}

#[derive(Clone, Debug)]
pub struct UDP {
    pub source_port: u16,
    pub dest_port: u16,
    pub length: u16,
    pub computed_checksum: u16,
    pub checksum: u16,
    pub data: Vec<u8>,
}

impl UDP {
    fn parse(header: &PsuedoHeader, bytes: Vec<u8>) -> Option<Self> {
        let mut reader = ByteReader::new(&bytes);

        let source_port = reader.read_u16()?;
        let dest_port = reader.read_u16()?;
        let length = reader.read_u16()?;
        let checksum = reader.read_u16()?;

        let data = reader.read_many_u8(length as usize - 8)?;

        let computed_checksum = {
            let mut u16bytes = vec![
                (header.src_ip & 0xFF00) as u16, (header.src_ip & 0xFF) as u16,
                (header.dst_ip & 0xFF00) as u16, (header.dst_ip & 0xFF) as u16,
                ((header.reserved as u16) << 8) as u16 | header.protocol as u16,
                header.length,
                source_port, dest_port,
                length,
            ];

            let data = u8stou16s(&data);
            u16bytes.extend(data.into_iter());
            calc_checksum(u16bytes.into_iter())
        };

        println!("{checksum:04X} vs computed {computed_checksum:04X}");
        
        Some(UDP {
            source_port,
            dest_port,
            length,
            computed_checksum,
            checksum,
            data,
        })
    }
}