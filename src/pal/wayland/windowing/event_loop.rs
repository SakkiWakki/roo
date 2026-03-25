use std::os::unix::net::UnixStream;

use super::super::encoding::read_event;
use super::super::objects::{ToplevelConfigure, WlBuffer, WlShm, WlSurface, XdgSurface, XdgWmBase};

pub enum LoopAction {
    Continue,
    Break,
}

pub struct EventContext {
    pub xdg_wm_base: XdgWmBase,
    pub xdg_surface: XdgSurface,
    pub wl_surface: WlSurface,
    pub wl_shm: WlShm,
    pub wl_buffer: WlBuffer,
    pub id_counter: u32,
    pub top_config_tmp: Option<ToplevelConfigure>,
}

pub fn event_loop<Ctx>(
    stream: &mut UnixStream,
    ctx: &mut Ctx,
    handlers: &[(
        u32,
        u16,
        fn(&[u8], &mut Ctx, &mut UnixStream) -> Result<LoopAction, std::io::Error>,
    )],
) -> Result<(), std::io::Error> {
    loop {
        let event = read_event(stream)?;
        for &(obj_id, opcode, handler) in handlers {
            if event.obj_id == obj_id && event.opcode == opcode {
                match handler(&event.data, ctx, stream)? {
                    LoopAction::Continue => {}
                    LoopAction::Break => return Ok(()),
                }
            }
        }
    }
}
