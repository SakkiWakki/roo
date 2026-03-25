use std::os::unix::net::UnixStream;

use super::super::super::windowing::event_loop::{EventContext, LoopAction};
use super::client::XdgWmBase;

impl XdgWmBase {
    pub fn handle_ping(
        data: &[u8],
        ctx: &mut EventContext,
        stream: &mut UnixStream,
    ) -> Result<LoopAction, std::io::Error> {
        let serial = u32::from_ne_bytes(data[0..4].try_into().unwrap());
        ctx.xdg_wm_base.pong(stream, serial)?;
        Ok(LoopAction::Continue)
    }
}
