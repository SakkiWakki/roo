use super::super::super::super::encoding::{build_msg, MessageBuilder};
use super::super::super::super::{HEADER_SIZE, U32_SIZE};
use std::io::Write;
use std::os::unix::net::UnixStream;

pub struct ZxdgDecorationManager {
    pub id: u32,
}

impl ZxdgDecorationManager {
    const GET_TOPLEVEL_DECORATION: u32 = 1;

    pub fn get_toplevel_decoration(
        &self,
        stream: &mut UnixStream,
        new_id: u32,
        xdg_toplevel_id: u32,
    ) -> Result<(), std::io::Error> {
        let msg_size = (HEADER_SIZE + U32_SIZE * 2) as u32;
        let mut msg = build_msg(self.id, msg_size, Self::GET_TOPLEVEL_DECORATION);
        msg.write_u32(new_id);
        msg.write_u32(xdg_toplevel_id);
        stream.write_all(&msg)?;
        Ok(())
    }
}
