use std::os::unix::net::UnixStream;

use super::super::super::super::windowing::event_loop::{EventContext, LoopAction};
use super::server::{ToplevelConfigure, XdgToplevel};

impl XdgToplevel {
    pub fn handle_close(
        _data: &[u8],
        _ctx: &mut EventContext,
        _stream: &mut UnixStream,
    ) -> Result<LoopAction, std::io::Error> {
        Ok(LoopAction::Break)
    }

    pub fn handle_configure(
        data: &[u8],
        ctx: &mut EventContext,
        _stream: &mut UnixStream,
    ) -> Result<LoopAction, std::io::Error> {
        ctx.top_config_tmp = Some(ToplevelConfigure::parse(data));
        Ok(LoopAction::Continue)
    }
}
