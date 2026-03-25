use crate::write_msg;
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
        let msg = write_msg!(
            self.id, Self::CREATE_BUFFER,
            new_id, offset as u32, width as u32, height as u32, stride as u32, format
        );
        stream.write_all(&msg)?;
        Ok(())
    }
}
