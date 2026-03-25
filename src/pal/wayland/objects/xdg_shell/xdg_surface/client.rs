use super::super::super::super::encoding::encode_op;
use std::io::Write;
use std::os::unix::net::UnixStream;

pub struct XdgSurface {
    pub id: u32,
}

impl XdgSurface {
    const GET_TOPLEVEL: u32 = 1;
    const ACK_CONFIGURE: u32 = 4;

    pub fn get_toplevel(&self, stream: &mut UnixStream, new_id: u32) -> Result<(), std::io::Error> {
        stream.write_all(&encode_op(self.id, new_id, Self::GET_TOPLEVEL))?;
        Ok(())
    }

    pub fn ack_configure(
        &self,
        stream: &mut UnixStream,
        serial: u32,
    ) -> Result<(), std::io::Error> {
        stream.write_all(&encode_op(self.id, serial, Self::ACK_CONFIGURE))?;
        Ok(())
    }
}
