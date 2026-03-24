use super::super::super::{HEADER_SIZE, U32_SIZE};
use super::super::super::encoding::{build_msg, MessageBuilder};
use std::io::Write;
use std::os::unix::net::UnixStream;

pub struct WlSurface {
    pub id: u32,
}

impl WlSurface {
    const ATTACH: u32 = 1;
    const DAMAGE: u32 = 2;
    const COMMIT: u32 = 6;

    pub fn attach(&self, stream: &mut UnixStream, buffer_id: u32) -> Result<(), std::io::Error> {
        let msg_size: u32 = (HEADER_SIZE + U32_SIZE * 3) as u32;
        let mut msg = build_msg(self.id, msg_size, Self::ATTACH);
        msg.write_u32(buffer_id);
        msg.write_u32(0); // x offset
        msg.write_u32(0); // y offset
        stream.write_all(&msg)?;
        Ok(())
    }

    pub fn damage(
        &self,
        stream: &mut UnixStream,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), std::io::Error> {
        let msg_size: u32 = (HEADER_SIZE + U32_SIZE * 4) as u32;
        let mut msg = build_msg(self.id, msg_size, Self::DAMAGE);
        msg.write_u32(x as u32);
        msg.write_u32(y as u32);
        msg.write_u32(width as u32);
        msg.write_u32(height as u32);
        stream.write_all(&msg)?;
        Ok(())
    }

    pub fn commit(&self, stream: &mut UnixStream) -> Result<(), std::io::Error> {
        let msg = build_msg(self.id, HEADER_SIZE as u32, Self::COMMIT);
        stream.write_all(&msg)?;
        Ok(())
    }
}
