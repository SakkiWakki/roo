use crate::write_msg;
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
        let msg = write_msg!(self.id, Self::ATTACH, buffer_id, 0u32, 0u32);
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
        let msg = write_msg!(self.id, Self::DAMAGE, x as u32, y as u32, width as u32, height as u32);
        stream.write_all(&msg)?;
        Ok(())
    }

    pub fn commit(&self, stream: &mut UnixStream) -> Result<(), std::io::Error> {
        let msg = write_msg!(self.id, Self::COMMIT);
        stream.write_all(&msg)?;
        Ok(())
    }
}
