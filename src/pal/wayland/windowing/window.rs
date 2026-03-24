use std::io::Cursor;
use std::os::unix::net::UnixStream;

use crate::pal::platform::objects::{
    ToplevelConfigure, WlBuffer, WlDisplay, WlShm, WlSurface, XdgSurface, XdgToplevel, XdgWmBase,
};

use super::super::encoding::{read_event, MessageReader};
use super::super::base_ids;
use super::connect::setup_buffer;

pub struct Window {
    pub(super) stream: UnixStream,
    pub(super) xdg_wm_base: XdgWmBase,
    pub(super) xdg_surface: XdgSurface,
    pub(super) toplevel_id: u32,
    pub(super) wl_surface: WlSurface,
    pub(super) wl_shm: WlShm,
    pub(super) wl_buffer: WlBuffer,
    pub(super) id_counter: u32,
}

impl Window {
    pub fn run(&mut self) -> Result<(), std::io::Error> {
        // TODO: This needs a loop helper
        let mut top_config_tmp: Option<ToplevelConfigure> = None;
        loop {
            let event = read_event(&mut self.stream)?;
            if event.obj_id == base_ids::WL_DISPLAY && event.opcode == WlDisplay::EVENT_ERROR {
                let mut cursor = Cursor::new(event.data.as_slice());
                let failed_id = cursor.read_u32();
                let code = cursor.read_u32();
                let msg = cursor.read_string();
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "wayland error: object={} code={} msg={}",
                        failed_id, code, msg
                    ),
                ));
            }
            if event.obj_id == self.xdg_wm_base.id && event.opcode == XdgWmBase::EVENT_PING {
                let serial = u32::from_ne_bytes(event.data[0..4].try_into().unwrap());
                self.xdg_wm_base.pong(&mut self.stream, serial)?;
            }
            if event.obj_id == self.toplevel_id && event.opcode == XdgToplevel::EVENT_CLOSE {
                break;
            }
            if event.obj_id == self.toplevel_id && event.opcode == XdgToplevel::EVENT_CONFIGURE {
                top_config_tmp = Some(ToplevelConfigure::parse(&event.data))
            }
            if event.obj_id == self.xdg_surface.id && event.opcode == XdgSurface::EVENT_CONFIGURE {
                let serial = u32::from_ne_bytes(event.data[0..4].try_into().unwrap());
                if let Some(top_config) = top_config_tmp.take() {
                    self.resize(top_config.width as u32, top_config.height as u32)?;
                }
                self.xdg_surface.ack_configure(&mut self.stream, serial)?;
                self.wl_surface.commit(&mut self.stream)?;
            }
        }
        Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), std::io::Error> {
        self.wl_buffer = setup_buffer(
            &mut self.stream,
            &mut self.id_counter,
            &self.wl_shm,
            width as i32,
            height as i32,
        )?;
        self.wl_surface.attach(&mut self.stream, self.wl_buffer.id)?;
        Ok(())
    }
}
