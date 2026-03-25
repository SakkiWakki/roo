use std::os::unix::net::UnixStream;

use super::super::super::windowing::connect::setup_buffer;
use super::super::super::windowing::event_loop::{EventContext, LoopAction};
use super::client::XdgSurface;

impl XdgSurface {
    pub fn handle_configure(
        data: &[u8],
        ctx: &mut EventContext,
        stream: &mut UnixStream,
    ) -> Result<LoopAction, std::io::Error> {
        let serial = u32::from_ne_bytes(data[0..4].try_into().unwrap());
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
        ctx: &mut Option<u32>,
        _stream: &mut UnixStream,
    ) -> Result<LoopAction, std::io::Error> {
        let serial = u32::from_ne_bytes(data[0..4].try_into().unwrap());
        *ctx = Some(serial);
        Ok(LoopAction::Break)
    }
}
