use std::fs::File;
use std::os::unix::net::UnixStream;

use super::surfaces::next_id;
use crate::pal::platform::objects::{
    WlDisplay, XdgSurface, XdgToplevel, XdgWmBase, ZwpLinuxBufferParamsV1, ZwpLinuxDmabufV1,
};
use crate::pal::platform::types::Fd;
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
    pub(super) dmabuf: ZwpLinuxDmabufV1,
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

    fn import_dmabuf(&mut self, fd: i32, format: u32, modifier: u64) -> Result<(), std::io::Error> {
        let (width, height) = self.resolution();
        let params_id = next_id(&mut self.ctx.id_counter);
        let buffer_id = next_id(&mut self.ctx.id_counter);
        let modifier_hi = (modifier >> 32) as u32;
        let modifier_lo = modifier as u32;
        let stride = width * 4;

        self.dmabuf.create_params(&mut self.stream, params_id)?;
        let params = ZwpLinuxBufferParamsV1 { id: params_id };
        params.add(
            &mut self.stream,
            Fd(fd),
            0,
            0,
            stride,
            modifier_hi,
            modifier_lo,
        )?;
        params.create_immed(
            &mut self.stream,
            buffer_id,
            width as i32,
            height as i32,
            format,
            0,
        )?;
        self.ctx.wl_surface.attach(&mut self.stream, buffer_id)?;
        self.ctx.wl_surface.commit(&mut self.stream)
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
