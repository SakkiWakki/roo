use crate::read_msg;
use std::io::Cursor;
use std::os::unix::net::UnixStream;

use super::super::super::super::types::Fd;
use super::super::super::super::windowing::event_loop::LoopAction;
use super::super::super::super::WaylandGlobal;
use super::client::WlRegistry;

impl WlRegistry {
    pub fn handle_global(
        data: &[u8],
        _fd: Option<Fd>,
        ctx: &mut Vec<WaylandGlobal>,
        _stream: &mut UnixStream,
    ) -> Result<LoopAction, std::io::Error> {
        let (name, interface, version) = read_msg!(Cursor::new(data), u32, String, u32);
        ctx.push(WaylandGlobal {
            name,
            interface,
            version,
        });
        Ok(LoopAction::Continue)
    }
}
