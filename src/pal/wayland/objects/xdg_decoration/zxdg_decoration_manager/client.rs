use crate::write_msg;
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
        let msg = write_msg!(self.id, Self::GET_TOPLEVEL_DECORATION, new_id, xdg_toplevel_id);
        stream.write_all(&msg)?;
        Ok(())
    }
}
