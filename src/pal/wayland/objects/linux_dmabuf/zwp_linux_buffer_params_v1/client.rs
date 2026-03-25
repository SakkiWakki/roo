use crate::write_msg;
use std::io::Write;
use std::os::unix::net::UnixStream;
pub struct ZwpLinuxBufferParamsV1 {
    pub id: u32,
}

impl ZwpLinuxBufferParamsV1 {
    const DESTROY: u32 = 0;
    const ADD: u32 = 1;
    const CREATE: u32 = 2;
    const CREATE_IMMED: u32 = 3;

    pub fn add(
        stream: &mut UnixStream,
        fd: u32,
        plane_idx: u32,
        offset: u32,
        stride: u32,
        modifier_hi: u32,
        modifier_lo: u32,
    ) {
        let msg = write_msg!(fd, plane_idx, offset, stride, modifier_hi, modifier_lo);
        stream.write_all(&msg);
    }

    pub fn create(
        &self,
        stream: &mut UnixStream,
        width: i32,
        height: i32,
        format: u32,
        flags: u32,
    ) -> Result<(), std::io::Error> {
        let msg = write_msg!(
            self.id,
            Self::CREATE,
            width as u32,
            height as u32,
            format,
            flags
        );
        stream.write_all(&msg)
    }

    pub fn create_immed(
        &self,
        stream: &mut UnixStream,
        buffer_id: u32,
        width: i32,
        height: i32,
        format: u32,
        flags: u32,
    ) -> Result<(), std::io::Error> {
        let msg = write_msg!(
            self.id,
            Self::CREATE_IMMED,
            buffer_id,
            width as u32,
            height as u32,
            format,
            flags
        );
        stream.write_all(&msg)
    }
}
