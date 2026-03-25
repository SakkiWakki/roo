use super::sizes::{ALIGN_MASK, NULL_TERMINATOR};

pub trait WlWrite {
    fn wl_write(&self, buf: &mut Vec<u8>);
}

impl WlWrite for u32 {
    fn wl_write(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.to_ne_bytes());
    }
}

impl WlWrite for &str {
    fn wl_write(&self, buf: &mut Vec<u8>) {
        let bytes = self.as_bytes();
        let str_len = bytes.len() + NULL_TERMINATOR;
        let padded = (str_len + ALIGN_MASK) & !ALIGN_MASK;
        (str_len as u32).wl_write(buf);
        buf.extend_from_slice(bytes);
        buf.resize(buf.len() + padded - bytes.len(), 0u8);
    }
}


#[macro_export]
macro_rules! write_msg {
    ($obj_id:expr, $opcode:expr $(, $val:expr)*) => {{
        use $crate::pal::platform::encoding::{WlWrite, sizes::{U32_SIZE, SIZE_SHIFT}};
        let mut buf = Vec::new();
        ($obj_id as u32).wl_write(&mut buf);
        buf.extend_from_slice(&[0u8; U32_SIZE]);
        $(($val).wl_write(&mut buf);)*
        let msg_size = buf.len() as u32;
        buf[4..8].copy_from_slice(
            &((msg_size << SIZE_SHIFT) | $opcode as u32).to_ne_bytes(),
        );
        buf
    }};
}
