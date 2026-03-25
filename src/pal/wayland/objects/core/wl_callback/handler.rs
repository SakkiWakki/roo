use std::os::unix::net::UnixStream;

use super::super::super::super::types::Fd;
use super::super::super::super::windowing::event_loop::LoopAction;
use super::super::super::super::WaylandGlobal;
use super::server::WlCallback;

impl WlCallback {
    pub fn handle_sync_done(
        _data: &[u8],
        _fd: Option<Fd>,
        _ctx: &mut Vec<WaylandGlobal>,
        _stream: &mut UnixStream,
    ) -> Result<LoopAction, std::io::Error> {
        Ok(LoopAction::Break)
    }
}
