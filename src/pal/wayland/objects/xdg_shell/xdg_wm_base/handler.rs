use crate::read_msg;
use std::io::Cursor;
use std::os::unix::net::UnixStream;

use super::super::super::super::types::Fd;
use super::super::super::super::windowing::event_loop::{EventContext, LoopAction};
use super::client::XdgWmBase;

impl XdgWmBase {
    pub fn handle_ping(
        data: &[u8],
        _fd: Option<Fd>,
        ctx: &mut EventContext,
        stream: &mut UnixStream,
    ) -> Result<LoopAction, std::io::Error> {
        let (serial,) = read_msg!(Cursor::new(data), u32);
        ctx.xdg_wm_base.pong(stream, serial)?;
        Ok(LoopAction::Continue)
    }
}
