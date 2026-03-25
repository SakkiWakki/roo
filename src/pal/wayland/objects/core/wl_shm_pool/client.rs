use super::super::super::super::encoding::{build_msg, MessageBuilder};
use super::super::super::super::{HEADER_SIZE, U32_SIZE};
use std::io::Write;
use std::os::unix::net::UnixStream;

pub struct WlShmPool {
    pub id: u32,
}

impl WlShmPool {
    pub const CREATE_BUFFER: u32 = 0;
    pub const DESTROY: u32 = 1;

    pub fn create_buffer(
        &self,
        stream: &mut UnixStream,
        new_id: u32,
        offset: i32,
        width: i32,
        height: i32,
        stride: i32,
        format: u32,
    ) -> Result<(), std::io::Error> {
        let msg_size: u32 = (HEADER_SIZE + U32_SIZE * 6) as u32;
        let mut msg = build_msg(self.id, msg_size, Self::CREATE_BUFFER);
        msg.write_u32(new_id);
        msg.write_u32(offset as u32);
        msg.write_u32(width as u32);
        msg.write_u32(height as u32);
        msg.write_u32(stride as u32);
        msg.write_u32(format);
        stream.write_all(&msg)?;
        Ok(())
    }
}
