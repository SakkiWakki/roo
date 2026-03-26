use std::fs::File;
use std::os::unix::net::UnixStream;

use crate::pal::platform::objects::{WlDisplay, XdgSurface, XdgToplevel, XdgWmBase};
use crate::pal::{GpuInfo, PlatformWindow, SupportedFormat};

use super::event_loop::{event_loop, EventContext};
use super::protocol::base_ids;

pub const DEFAULT_WIDTH: u32 = 800;
pub const DEFAULT_HEIGHT: u32 = 600;
pub struct Window {
    pub(super) stream: UnixStream,
    pub(super) ctx: EventContext,
    pub(super) drm_device: Option<File>,
    pub(super) toplevel_id: u32,
}

impl PlatformWindow for Window {
    fn resolution(&self) -> (u32, u32) {
        let c = self.ctx.top_config_tmp.as_ref();
        let w = c.map(|c| c.width).unwrap_or(0);
        let h = c.map(|c| c.height).unwrap_or(0);
        let w = if w > 0 { w as u32 } else { DEFAULT_WIDTH };
        let h = if h > 0 { h as u32 } else { DEFAULT_HEIGHT };
        (w, h)
    }

    fn gpu_info(&mut self) -> Option<GpuInfo> {
        self.drm_device.take().map(|f| GpuInfo { device_node: f })
    }

    fn formats(&self) -> &[SupportedFormat] {
        &self.ctx.formats
    }

    fn run(&mut self) -> Result<(), std::io::Error> {
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
