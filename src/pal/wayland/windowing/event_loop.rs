use std::os::unix::net::UnixStream;

use super::super::encoding::read_event;
use crate::pal::SupportedFormat;
use super::super::objects::{ToplevelConfigure, WlBuffer, WlShm, WlSurface, XdgSurface, XdgWmBase};
use super::super::types::Fd;

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
    pub formats: Vec<SupportedFormat>,
}

pub fn event_loop<Ctx>(
    stream: &mut UnixStream,
    ctx: &mut Ctx,
    handlers: &[(
        u32,
        u16,
        fn(&[u8], Option<Fd>, &mut Ctx, &mut UnixStream) -> Result<LoopAction, std::io::Error>,
    )],
) -> Result<(), std::io::Error> {
    loop {
        let event = read_event(stream)?;
        let fd = event.fd;
        for &(obj_id, opcode, handler) in handlers {
            if event.obj_id == obj_id && event.opcode == opcode {
                match handler(&event.data, fd, ctx, stream)? {
                    LoopAction::Continue => {}
                    LoopAction::Break => return Ok(()),
                }
            }
        }
    }
}
