use super::sizes::{ALIGN_MASK, NULL_TERMINATOR, U32_SIZE};
use std::io::Read;

pub trait WlRead: Sized {
    fn wl_read(r: &mut impl Read) -> Self;
}

impl WlRead for u32 {
    fn wl_read(r: &mut impl Read) -> Self {
        let mut buf = [0u8; U32_SIZE];
        r.read_exact(&mut buf).unwrap();
        u32::from_ne_bytes(buf)
    }
}

impl WlRead for String {
    fn wl_read(r: &mut impl Read) -> Self {
        let mut len_buf = [0u8; U32_SIZE];
        r.read_exact(&mut len_buf).unwrap();
        let str_len = u32::from_ne_bytes(len_buf) as usize;
        let padded = (str_len + ALIGN_MASK) & !ALIGN_MASK;
        let mut buf = vec![0u8; padded];
        r.read_exact(&mut buf).unwrap();
        String::from_utf8(buf[..str_len - NULL_TERMINATOR].to_vec()).unwrap()
    }
}

#[macro_export]
macro_rules! read_msg {
    ($reader:expr, $($ty:ty),+) => {{
        use $crate::pal::platform::encoding::WlRead;
        let r = &mut $reader;
        ($(<$ty>::wl_read(r),)+)
    }};
}
