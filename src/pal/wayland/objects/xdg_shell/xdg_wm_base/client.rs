use crate::write_msg;
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
        let msg = write_msg!(self.id, Self::GET_XDG_SURFACE, new_id, surface_id);
        stream.write_all(&msg)?;
        Ok(())
    }

    pub fn pong(&self, stream: &mut UnixStream, serial: u32) -> Result<(), std::io::Error> {
        let msg = write_msg!(self.id, Self::PONG, serial);
        stream.write_all(&msg)?;
        Ok(())
    }
}
