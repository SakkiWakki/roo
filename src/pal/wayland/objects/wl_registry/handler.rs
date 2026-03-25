use std::io::Cursor;
use std::os::unix::net::UnixStream;

use super::super::super::encoding::MessageReader;
use super::super::super::windowing::event_loop::LoopAction;
use super::super::super::WaylandGlobal;
use super::client::WlRegistry;

impl WlRegistry {
    pub fn handle_global(
        data: &[u8],
        ctx: &mut Vec<WaylandGlobal>,
        _stream: &mut UnixStream,
    ) -> Result<LoopAction, std::io::Error> {
        let mut cursor = Cursor::new(data);
        let name = cursor.read_u32();
        let interface = cursor.read_string();
        let version = cursor.read_u32();
        ctx.push(WaylandGlobal {
            name,
            interface,
            version,
        });
        Ok(LoopAction::Continue)
    }
}
