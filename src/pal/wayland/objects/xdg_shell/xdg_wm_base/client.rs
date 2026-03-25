use super::super::super::super::encoding::{build_msg, encode_op, MessageBuilder};
use super::super::super::super::{HEADER_SIZE, U32_SIZE};
use std::io::Write;
use std::os::unix::net::UnixStream;

pub struct XdgWmBase {
    pub id: u32,
}

impl XdgWmBase {
    const GET_XDG_SURFACE: u32 = 2;
    const PONG: u32 = 3;

    pub fn get_xdg_surface(
        &self,
        stream: &mut UnixStream,
        surface_id: u32,
        new_id: u32,
    ) -> Result<(), std::io::Error> {
        let msg_size: u32 = (HEADER_SIZE + U32_SIZE * 2) as u32;
        let mut msg = build_msg(self.id, msg_size, Self::GET_XDG_SURFACE);
        msg.write_u32(new_id);
        msg.write_u32(surface_id);
        stream.write_all(&msg)?;
        Ok(())
    }

    pub fn pong(&self, stream: &mut UnixStream, serial: u32) -> Result<(), std::io::Error> {
        stream.write_all(&encode_op(self.id, serial, Self::PONG))?;
        Ok(())
    }
}
