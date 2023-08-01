use std::io::{Cursor, Read};

pub struct ByteReader<'a> {
    cursor: Cursor<&'a [u8]>,
}

impl<'a> ByteReader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(buf),
        }
    }
}

pub trait BudgetByteorder {
    fn read_u8(&mut self) -> u8;
    fn read_u16(&mut self) -> u16;
    fn read_u32(&mut self) -> u32;
    fn read_many_u8(&mut self, how_many: usize) -> Vec<u8>;
}

impl<'a> BudgetByteorder for ByteReader<'a> {
    fn read_u8(&mut self) -> u8 {
        let mut buf = [0; 1];
        let _ = self.cursor.read_exact(&mut buf);
        buf[0]
    }

    fn read_u16(&mut self) -> u16 {
        let mut buf = [0; 2];
        let _ = self.cursor.read_exact(&mut buf);
        u16::from_be_bytes(buf)
    }

    fn read_u32(&mut self) -> u32 {
        let mut buf = [0; 4];
        let _ = self.cursor.read_exact(&mut buf);
        u32::from_be_bytes(buf)
    }

    fn read_many_u8(&mut self, how_many: usize) -> Vec<u8> {
        let mut buf = vec![0; how_many];
        let _ = self.cursor.read_exact(&mut buf);
        buf
    }
}