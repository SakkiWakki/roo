use super::super::super::super::types::Fd;
use super::super::super::super::windowing::buffer::setup_buffer;
use super::super::super::super::windowing::event_loop::{EventContext, LoopAction};
use super::client::XdgSurface;
use crate::read_at;
use std::os::unix::net::UnixStream;

impl XdgSurface {
    pub fn handle_configure(
        data: &[u8],
        _fd: Option<Fd>,
        ctx: &mut EventContext,
        stream: &mut UnixStream,
    ) -> Result<LoopAction, std::io::Error> {
        let serial = read_at!(data, 0, u32);
        if let Some(top_config) = ctx.top_config_tmp.take() {
            ctx.wl_buffer = setup_buffer(
                stream,
                &mut ctx.id_counter,
                &ctx.wl_shm,
                top_config.width,
                top_config.height,
            )?;
            ctx.wl_surface.attach(stream, ctx.wl_buffer.id)?;
        }
        ctx.xdg_surface.ack_configure(stream, serial)?;
        ctx.wl_surface.commit(stream)?;
        Ok(LoopAction::Continue)
    }

    pub fn handle_configure_serial(
        data: &[u8],
        _fd: Option<Fd>,
        ctx: &mut Option<u32>,
        _stream: &mut UnixStream,
    ) -> Result<LoopAction, std::io::Error> {
        let serial = read_at!(data, 0, u32);
        *ctx = Some(serial);
        Ok(LoopAction::Break)
    }
}
