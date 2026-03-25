use super::sizes::{ALIGN_MASK, NULL_TERMINATOR};

pub trait MessageBuilder {
    fn write_u32(&mut self, val: u32);
    fn write_string(&mut self, val: &str);
}

impl MessageBuilder for Vec<u8> {
    fn write_u32(&mut self, val: u32) {
        self.extend_from_slice(&val.to_ne_bytes());
    }

    fn write_string(&mut self, val: &str) {
        let bytes = val.as_bytes();
        let str_len = bytes.len() + NULL_TERMINATOR;
        let str_padded = (str_len + ALIGN_MASK) & !ALIGN_MASK;
        self.write_u32(str_len as u32);
        self.extend_from_slice(bytes);
        let padding = str_padded - bytes.len();
        for _ in 0..padding {
            self.push(0u8);
        }
    }
}
