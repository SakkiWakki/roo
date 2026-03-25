use super::super::super::encoding::encode_op;
use std::io::Write;
use std::os::unix::net::UnixStream;

pub struct WlCompositor {
    pub id: u32,
}

impl WlCompositor {
    pub const CREATE_SURFACE: u32 = 0;

    pub fn create_surface(
        &self,
        stream: &mut UnixStream,
        new_id: u32,
    ) -> Result<(), std::io::Error> {
        stream.write_all(&encode_op(self.id, new_id, Self::CREATE_SURFACE))?;
        Ok(())
    }
}
