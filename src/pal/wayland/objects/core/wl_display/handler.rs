use crate::read_msg;
use std::io::Cursor;
use std::os::unix::net::UnixStream;

use super::super::super::super::windowing::event_loop::{EventContext, LoopAction};
use super::client::WlDisplay;

impl WlDisplay {
    pub fn handle_error(
        data: &[u8],
        _ctx: &mut EventContext,
        _stream: &mut UnixStream,
    ) -> Result<LoopAction, std::io::Error> {
        let (failed_id, code, msg) = read_msg!(Cursor::new(data), u32, u32, String);
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "wayland error: object={} code={} msg={}",
                failed_id, code, msg
            ),
        ))
    }
}
