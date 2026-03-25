use std::os::unix::net::UnixStream;

use crate::pal::platform::objects::{WlDisplay, XdgSurface, XdgToplevel, XdgWmBase};

use super::event_loop::{event_loop, EventContext};
use super::protocol::base_ids;

pub struct Window {
    pub(super) stream: UnixStream,
    pub(super) ctx: EventContext,
    pub(super) toplevel_id: u32,
}

impl Window {
    pub fn run(&mut self) -> Result<(), std::io::Error> {
        // I'm not sure if the compiler can inline these because it is prby possible
        // But I'm assuming that a compiler does not inline these
        // But also socket reads are quite expensive so I don't think it matters
        // TODO: Vulkan stuff
        let handlers = [
            (
                base_ids::WL_DISPLAY,
                WlDisplay::EVENT_ERROR,
                WlDisplay::handle_error as _,
            ),
            (
                self.ctx.xdg_wm_base.id,
                XdgWmBase::EVENT_PING,
                XdgWmBase::handle_ping as _,
            ),
            (
                self.toplevel_id,
                XdgToplevel::EVENT_CLOSE,
                XdgToplevel::handle_close as _,
            ),
            (
                self.toplevel_id,
                XdgToplevel::EVENT_CONFIGURE,
                XdgToplevel::handle_configure as _,
            ),
            (
                self.ctx.xdg_surface.id,
                XdgSurface::EVENT_CONFIGURE,
                XdgSurface::handle_configure as _,
            ),
        ];
        event_loop(&mut self.stream, &mut self.ctx, &handlers)
    }
}
