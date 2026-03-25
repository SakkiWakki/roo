use std::io::Cursor;
use std::os::unix::net::UnixStream;

use super::super::super::super::encoding::MessageReader;
use super::super::super::super::windowing::event_loop::{EventContext, LoopAction};
use super::client::WlDisplay;

impl WlDisplay {
    pub fn handle_error(
        data: &[u8],
        _ctx: &mut EventContext,
        _stream: &mut UnixStream,
    ) -> Result<LoopAction, std::io::Error> {
        let mut cursor = Cursor::new(data);
        let failed_id = cursor.read_u32();
        let code = cursor.read_u32();
        let msg = cursor.read_string();
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "wayland error: object={} code={} msg={}",
                failed_id, code, msg
            ),
        ))
    }
}
