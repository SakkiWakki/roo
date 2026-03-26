use std::os::unix::net::UnixStream;

use crate::pal::platform::objects::{
    WlCompositor, WlSurface, XdgSurface, XdgWmBase, ZxdgDecorationManager, ZxdgToplevelDecoration,
};

pub fn create_wl_surface(
    stream: &mut UnixStream,
    id_counter: &mut u32,
    compositor: &WlCompositor,
) -> Result<WlSurface, std::io::Error> {
    let surface_id = next_id(id_counter);
    compositor.create_surface(stream, surface_id)?;
    Ok(WlSurface { id: surface_id })
}

pub fn create_xdg_surface(
    stream: &mut UnixStream,
    id_counter: &mut u32,
    xdg_wm_base: &XdgWmBase,
    wl_surface: &WlSurface,
) -> Result<XdgSurface, std::io::Error> {
    let xdg_surface_id = next_id(id_counter);
    xdg_wm_base.get_xdg_surface(stream, wl_surface.id, xdg_surface_id)?;
    Ok(XdgSurface { id: xdg_surface_id })
}

pub fn create_xdg_toplevel(
    stream: &mut UnixStream,
    id_counter: &mut u32,
    xdg_surface: &XdgSurface,
) -> Result<u32, std::io::Error> {
    let xdg_toplevel_id = next_id(id_counter);
    xdg_surface.get_toplevel(stream, xdg_toplevel_id)?;
    Ok(xdg_toplevel_id)
}

pub fn setup_decoration(
    stream: &mut UnixStream,
    id_counter: &mut u32,
    zxdg_deco_manager: &ZxdgDecorationManager,
    xdg_toplevel_id: u32,
) -> Result<(), std::io::Error> {
    let zxdg_deco_id = next_id(id_counter);
    zxdg_deco_manager.get_toplevel_decoration(stream, zxdg_deco_id, xdg_toplevel_id)?;
    let zxdg_top_deco = ZxdgToplevelDecoration { id: zxdg_deco_id };
    zxdg_top_deco.set_server_side_mode(stream)
}

pub fn next_id(id_counter: &mut u32) -> u32 {
    let id = *id_counter;
    *id_counter += 1;
    id
}
