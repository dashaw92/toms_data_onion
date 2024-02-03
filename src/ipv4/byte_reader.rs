use std::io::{Cursor, Read, Seek, SeekFrom};

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
    fn seek(&mut self, offset: i64) -> Result<(), std::io::Error>;
    fn read_u8(&mut self) -> Option<u8>;
    fn read_u16(&mut self) -> Option<u16>;
    fn read_u32(&mut self) -> Option<u32>;
    fn read_many_u8(&mut self, how_many: usize) -> Option<Vec<u8>>;
}

impl<'a> BudgetByteorder for ByteReader<'a> {
    fn seek(&mut self, offset: i64) -> Result<(), std::io::Error> {
        self.cursor.seek(SeekFrom::Current(offset)).map(|_| ())
    }

    fn read_u8(&mut self) -> Option<u8> {
        let mut buf = [0; 1];
        self.cursor.read_exact(&mut buf).ok()?;
        Some(buf[0])
    }

    fn read_u16(&mut self) -> Option<u16> {
        let mut buf = [0; 2];
        self.cursor.read_exact(&mut buf).ok()?;
        Some(u16::from_be_bytes(buf))
    }

    fn read_u32(&mut self) -> Option<u32> {
        let mut buf = [0; 4];
        self.cursor.read_exact(&mut buf).ok()?;
        Some(u32::from_be_bytes(buf))
    }

    fn read_many_u8(&mut self, how_many: usize) -> Option<Vec<u8>> {
        let mut buf = vec![0; how_many];
        self.cursor.read_exact(&mut buf).ok()?;
        Some(buf)
    }
}