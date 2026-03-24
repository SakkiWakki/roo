use super::super::{ALIGN_MASK, NULL_TERMINATOR, U32_SIZE};
use std::io::{Cursor, Read};
use std::os::unix::net::UnixStream;

pub trait MessageReader {
    fn read_u32(&mut self) -> u32;
    fn read_string(&mut self) -> String;
}

fn read_string_impl(r: &mut impl Read) -> String {
    let mut len_buf = [0u8; U32_SIZE];
    r.read_exact(&mut len_buf).unwrap();
    let str_len = u32::from_ne_bytes(len_buf) as usize;
    let padded = (str_len + ALIGN_MASK) & !ALIGN_MASK;
    let mut buf = vec![0u8; padded];
    r.read_exact(&mut buf).unwrap();
    String::from_utf8(buf[..str_len - NULL_TERMINATOR].to_vec()).unwrap()
}

impl MessageReader for UnixStream {
    fn read_u32(&mut self) -> u32 {
        let mut buf = [0u8; U32_SIZE];
        self.read_exact(&mut buf).unwrap();
        u32::from_ne_bytes(buf)
    }

    fn read_string(&mut self) -> String {
        read_string_impl(self)
    }
}

impl MessageReader for Cursor<Vec<u8>> {
    fn read_u32(&mut self) -> u32 {
        let mut buf = [0u8; U32_SIZE];
        self.read_exact(&mut buf).unwrap();
        u32::from_ne_bytes(buf)
    }

    fn read_string(&mut self) -> String {
        read_string_impl(self)
    }
}
