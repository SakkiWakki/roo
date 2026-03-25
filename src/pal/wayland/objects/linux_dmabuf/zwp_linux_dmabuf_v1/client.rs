use std::io::Write;
use std::os::unix::net::UnixStream;

use crate::write_msg;

pub struct ZwpLinuxDmabufV1 {
    pub id: u32,
}

impl ZwpLinuxDmabufV1 {
    const DESTROY: u32 = 0;
    const CREATE_PARAMS: u32 = 1;
    const GET_DEFAULT_FEEDBACK: u32 = 2;
    const GET_SURFACE_FEEDBACK: u32 = 3;

    pub fn create_params(
        &self,
        stream: &mut UnixStream,
        params_id: u32,
    ) -> Result<(), std::io::Error> {
        let msg = write_msg!(self.id, Self::CREATE_PARAMS, params_id);
        stream.write_all(&msg)
    }

    pub fn get_default_feedback(&self, stream: &mut UnixStream, new_id: u32) {
        let msg = write_msg!(self.id, Self::GET_DEFAULT_FEEDBACK, new_id);
        stream.write_all(&msg);
    }

    pub fn get_surface_feedback(&self, stream: &mut UnixStream, new_id: u32, surface_id: u32) {
        let msg = write_msg!(self.id, Self::GET_SURFACE_FEEDBACK, new_id, surface_id);
        stream.write_all(&msg);
    }
}
