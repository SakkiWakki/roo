use super::super::super::super::encoding::encode_op;
use std::io::Write;
use std::os::unix::net::UnixStream;

pub struct ZxdgToplevelDecoration {
    pub id: u32,
}

impl ZxdgToplevelDecoration {
    const SET_MODE: u32 = 1;
    const CLIENT_SIDE_MODE: u32 = 1;
    const SERVER_SIDE_MODE: u32 = 2;

    fn set_mode(&self, stream: &mut UnixStream, mode: u32) -> Result<(), std::io::Error> {
        stream.write_all(&encode_op(self.id, mode, Self::SET_MODE))?;
        Ok(())
    }

    pub fn set_server_side_mode(&self, stream: &mut UnixStream) -> Result<(), std::io::Error> {
        self.set_mode(stream, Self::SERVER_SIDE_MODE)
    }

    pub fn set_client_side_mode(&self, stream: &mut UnixStream) -> Result<(), std::io::Error> {
        self.set_mode(stream, Self::CLIENT_SIDE_MODE)
    }
}
