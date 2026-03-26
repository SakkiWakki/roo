use std::env;
use std::os::unix::net::UnixStream;

use crate::pal::platform::objects::XdgSurface;
use crate::pal::platform::types::TRANCHE_FLAG_SCANOUT;
use crate::pal::platform::windowing::feedback::open_drm_device;
use crate::pal::SupportedFormat;

use super::buffer::setup_buffer;
use super::event_loop::{event_loop, EventContext};
use super::feedback::dmabuf_feedback;
use super::globals::setup_globals;
use super::protocol::base_ids;
use super::surfaces::{
    create_wl_surface, create_xdg_surface, create_xdg_toplevel, setup_decoration,
};
use super::window::Window;
use super::window::{DEFAULT_HEIGHT, DEFAULT_WIDTH};

pub fn connect() -> Result<Window, std::io::Error> {
    let runtime_dir = env::var("XDG_RUNTIME_DIR")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e))?;
    let wayland_display = env::var("WAYLAND_DISPLAY").unwrap_or("wayland-0".to_string());
    let socket_path = format!("{}/{}", runtime_dir, wayland_display);

    let mut stream = UnixStream::connect(&socket_path)?;
    let mut id_counter: u32 = base_ids::ZWP_LINUX_DMABUF + 1;

    let (compositor, wl_shm, xdg_wm_base, zxdg_deco_manager, dmabuf) = setup_globals(&mut stream)?;
    let feedback = dmabuf_feedback(&mut stream, &mut id_counter, &dmabuf)?;
    let drm_device = open_drm_device(feedback.main_device)?;
    let scanout: Vec<SupportedFormat> = feedback
        .tranches
        .iter()
        .filter(|t| t.flags & TRANCHE_FLAG_SCANOUT != 0)
        .flat_map(|t| t.formats.iter())
        .map(|&(drm_format, modifier)| SupportedFormat {
            drm_format,
            modifier,
        })
        .collect();
    let formats: Vec<SupportedFormat> = if !scanout.is_empty() {
        scanout
    } else {
        feedback
            .tranches
            .iter()
            .flat_map(|t| t.formats.iter())
            .map(|&(drm_format, modifier)| SupportedFormat {
                drm_format,
                modifier,
            })
            .collect()
    };
    let wl_surface = create_wl_surface(&mut stream, &mut id_counter, &compositor)?;
    let xdg_surface = create_xdg_surface(&mut stream, &mut id_counter, &xdg_wm_base, &wl_surface)?;
    let xdg_toplevel_id = create_xdg_toplevel(&mut stream, &mut id_counter, &xdg_surface)?;
    setup_decoration(
        &mut stream,
        &mut id_counter,
        &zxdg_deco_manager,
        xdg_toplevel_id,
    )?;

    wl_surface.commit(&mut stream)?;
    let serial = wait_for_configure(&mut stream, xdg_surface.id)?;

    let wl_buffer = setup_buffer(
        &mut stream,
        &mut id_counter,
        &wl_shm,
        DEFAULT_WIDTH as i32,
        DEFAULT_HEIGHT as i32,
    )?;
    wl_surface.attach(&mut stream, wl_buffer.id)?;
    xdg_surface.ack_configure(&mut stream, serial)?;
    wl_surface.commit(&mut stream)?;

    Ok(Window {
        stream,
        toplevel_id: xdg_toplevel_id,
        drm_device: Some(drm_device),
        dmabuf,
        ctx: EventContext {
            xdg_wm_base,
            xdg_surface,
            wl_surface,
            wl_shm,
            wl_buffer,
            id_counter,
            top_config_tmp: None,
            formats,
        },
    })
}

fn wait_for_configure(stream: &mut UnixStream, xdg_surface_id: u32) -> Result<u32, std::io::Error> {
    let mut serial: Option<u32> = None;
    let handlers = [(
        xdg_surface_id,
        XdgSurface::EVENT_CONFIGURE,
        XdgSurface::handle_configure_serial as _,
    )];
    event_loop(stream, &mut serial, &handlers)?;
    serial.ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::Other, "configure event missing serial")
    })
}
