use std::io::Write;
use std::os::unix::net::UnixStream;

use crate::pal::platform::objects::{
    WlCallback, WlCompositor, WlDisplay, WlRegistry, WlShm, XdgWmBase, ZwpLinuxDmabufV1,
    ZxdgDecorationManager,
};
use crate::pal::platform::windowing::bind::Bind;

use super::super::encoding::{encode_bind, encode_op};
use super::super::types::WaylandGlobal;
use super::event_loop::event_loop;
use super::protocol::{base_ids, interfaces};

pub fn setup_globals(
    stream: &mut UnixStream,
) -> Result<
    (
        WlCompositor,
        WlShm,
        XdgWmBase,
        ZxdgDecorationManager,
        ZwpLinuxDmabufV1,
    ),
    std::io::Error,
> {
    stream.write_all(&encode_op(
        base_ids::WL_DISPLAY,
        base_ids::REGISTRY,
        WlDisplay::GET_REGISTRY,
    ))?;
    stream.write_all(&encode_op(
        base_ids::WL_DISPLAY,
        base_ids::SYNC,
        WlDisplay::SYNC,
    ))?;
    let globals = read_until_sync(stream)?;
    create_window_bindings(stream, globals)
}

fn read_until_sync(stream: &mut UnixStream) -> Result<Vec<WaylandGlobal>, std::io::Error> {
    let mut globals: Vec<WaylandGlobal> = Vec::new();
    let handlers = [
        (
            base_ids::REGISTRY,
            WlRegistry::EVENT_GLOBAL,
            WlRegistry::handle_global as _,
        ),
        (
            base_ids::SYNC,
            WlCallback::EVENT_DONE,
            WlCallback::handle_sync_done as _,
        ),
    ];
    event_loop(stream, &mut globals, &handlers)?;
    Ok(globals)
}

fn create_window_bindings(
    stream: &mut UnixStream,
    globals: Vec<WaylandGlobal>,
) -> Result<
    (
        WlCompositor,
        WlShm,
        XdgWmBase,
        ZxdgDecorationManager,
        ZwpLinuxDmabufV1,
    ),
    std::io::Error,
> {
    Ok((
        bind_global::<WlCompositor>(stream, &globals)?,
        bind_global::<WlShm>(stream, &globals)?,
        bind_global::<XdgWmBase>(stream, &globals)?,
        bind_global::<ZxdgDecorationManager>(stream, &globals)?,
        bind_global::<ZwpLinuxDmabufV1>(stream, &globals)?,
    ))
}

pub fn bind_global<T: Bind>(
    stream: &mut UnixStream,
    globals: &[WaylandGlobal],
) -> Result<T, std::io::Error> {
    let chosen = globals
        .iter()
        .filter(|g| g.interface == T::INTERFACE)
        .min_by_key(|g| g.name)
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("compositor missing required global: {}", T::INTERFACE),
            )
        })?;

    let version = std::cmp::min(chosen.version, supported_version(T::INTERFACE));
    stream.write_all(&encode_bind(chosen.name, T::INTERFACE, version, T::BIND_ID))?;
    Ok(T::new())
}

fn supported_version(interface: &str) -> u32 {
    match interface {
        interfaces::WL_COMPOSITOR => 4,
        interfaces::WL_SHM => 1,
        interfaces::XDG_WM_BASE => 2,
        interfaces::ZXDG_DECORATION_MANAGER => 1,
        _ => 1,
    }
}
