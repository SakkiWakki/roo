use std::env;
use std::io::Write;
use std::os::unix::net::UnixStream;

use crate::pal::platform::objects::{
    FeedbackState, WlBuffer, WlCallback, WlCompositor, WlDisplay, WlRegistry, WlShm, WlSurface,
    XdgSurface, XdgWmBase, ZwpLinuxDmabufFeedbackV1, ZwpLinuxDmabufV1, ZxdgDecorationManager,
    ZxdgToplevelDecoration,
};
use crate::pal::platform::windowing::bind::Bind;
use crate::pal::platform::DmabufFeedback;

use super::super::encoding::{encode_bind, encode_op};
use super::super::types::WaylandGlobal;
use super::event_loop::{event_loop, EventContext};
use super::protocol::{base_ids, interfaces};
use super::window::Window;

pub fn connect() -> Result<Window, std::io::Error> {
    let runtime_dir = env::var("XDG_RUNTIME_DIR")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e))?;
    let wayland_display = env::var("WAYLAND_DISPLAY").unwrap_or("wayland-0".to_string());
    let socket_path: String = format!("{}/{}", runtime_dir, wayland_display);

    let mut stream = UnixStream::connect(&socket_path)?;
    let mut id_counter: u32 = base_ids::ZWP_LINUX_DMABUF + 1;

    let (compositor, wl_shm, xdg_wm_base, zxdg_deco_manager, dmabuf) = setup_globals(&mut stream)?;
    let feedback = dmabuf_feedback(&mut stream, &mut id_counter, &dmabuf)?;
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

    let wl_buffer = setup_buffer(&mut stream, &mut id_counter, &wl_shm, 800, 600)?;
    wl_surface.attach(&mut stream, wl_buffer.id)?;
    xdg_surface.ack_configure(&mut stream, serial)?;
    wl_surface.commit(&mut stream)?;

    Ok(Window {
        stream,
        toplevel_id: xdg_toplevel_id,
        ctx: EventContext {
            xdg_wm_base,
            xdg_surface,
            wl_surface,
            wl_shm,
            wl_buffer,
            id_counter,
            top_config_tmp: None,
        },
    })
}

fn next_id(id_counter: &mut u32) -> u32 {
    let id = *id_counter;
    *id_counter += 1;
    id
}

fn setup_globals(
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

fn create_wl_surface(
    stream: &mut UnixStream,
    id_counter: &mut u32,
    compositor: &WlCompositor,
) -> Result<WlSurface, std::io::Error> {
    let surface_id = next_id(id_counter);
    compositor.create_surface(stream, surface_id)?;
    Ok(WlSurface { id: surface_id })
}

fn create_xdg_surface(
    stream: &mut UnixStream,
    id_counter: &mut u32,
    xdg_wm_base: &XdgWmBase,
    wl_surface: &WlSurface,
) -> Result<XdgSurface, std::io::Error> {
    let xdg_surface_id = next_id(id_counter);
    xdg_wm_base.get_xdg_surface(stream, wl_surface.id, xdg_surface_id)?;
    Ok(XdgSurface { id: xdg_surface_id })
}

fn create_xdg_toplevel(
    stream: &mut UnixStream,
    id_counter: &mut u32,
    xdg_surface: &XdgSurface,
) -> Result<u32, std::io::Error> {
    let xdg_toplevel_id = next_id(id_counter);
    xdg_surface.get_toplevel(stream, xdg_toplevel_id)?;
    Ok(xdg_toplevel_id)
}

fn setup_decoration(
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

fn dmabuf_feedback(
    stream: &mut UnixStream,
    id_counter: &mut u32,
    dmabuf: &ZwpLinuxDmabufV1,
) -> Result<DmabufFeedback, std::io::Error> {
    let dmabuf_feedback_id = next_id(id_counter);
    dmabuf.get_default_feedback(stream, dmabuf_feedback_id)?;
    loop_until_feedback(stream, dmabuf_feedback_id)
}

fn loop_until_feedback(
    stream: &mut UnixStream,
    feedback_id: u32,
) -> Result<DmabufFeedback, std::io::Error> {
    let mut state = FeedbackState::default();
    let handlers = [
        (
            feedback_id,
            ZwpLinuxDmabufFeedbackV1::EVENT_MAIN_DEVICE,
            ZwpLinuxDmabufFeedbackV1::handle_main_device as _,
        ),
        (
            feedback_id,
            ZwpLinuxDmabufFeedbackV1::EVENT_FORMAT_TABLE,
            ZwpLinuxDmabufFeedbackV1::handle_format_table as _,
        ),
        (
            feedback_id,
            ZwpLinuxDmabufFeedbackV1::EVENT_TRANCHE_FORMATS,
            ZwpLinuxDmabufFeedbackV1::handle_tranche_formats as _,
        ),
        (
            feedback_id,
            ZwpLinuxDmabufFeedbackV1::EVENT_DONE,
            ZwpLinuxDmabufFeedbackV1::handle_done as _,
        ),
    ];
    event_loop(stream, &mut state, &handlers)?;
    Ok(DmabufFeedback {
        main_device: state.main_device,
        formats: state.formats,
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

pub fn setup_buffer(
    stream: &mut UnixStream,
    id_counter: &mut u32,
    wl_shm: &WlShm,
    width: i32,
    height: i32,
) -> Result<WlBuffer, std::io::Error> {
    let pool_id = next_id(id_counter);
    let wl_buffer_id = next_id(id_counter);
    let mut wl_buffer = wl_shm.alloc_buffer(stream, pool_id, wl_buffer_id, width, height)?;
    wl_buffer.fill(0xFF000000);
    Ok(wl_buffer)
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

fn supported_version(interface: &str) -> u32 {
    // TODO: Decide this later
    match interface {
        interfaces::WL_COMPOSITOR => 4,
        interfaces::WL_SHM => 1,
        interfaces::XDG_WM_BASE => 2,
        interfaces::ZXDG_DECORATION_MANAGER => 1,
        _ => 1,
    }
}

/// Bind in monotonically increasing id order for the backends that need it to be like this
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

fn bind_global<T: Bind>(
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
