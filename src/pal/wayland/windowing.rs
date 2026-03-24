//! See https://wayland-book.com/protocol-design/wire-protocol.html
use std::env;
use std::io::Cursor;
use std::io::Write;
use std::os::unix::net::UnixStream;

use crate::pal::platform::objects::{
    ToplevelConfigure, WlBuffer, WlCallback, WlCompositor, WlDisplay, WlRegistry, WlShm, WlSurface,
    XdgSurface, XdgToplevel, XdgWmBase, ZxdgDecorationManager, ZxdgToplevelDecoration,
};

use super::encoding::{encode_bind, encode_op, read_event, MessageReader};
use super::{base_ids, interfaces, WaylandGlobal};

pub struct Window {
    stream: UnixStream,
    xdg_wm_base: XdgWmBase,
    xdg_surface: XdgSurface,
    xdg_surface_id: u32,
    toplevel_id: u32,
    wl_surface: WlSurface,
    wl_shm: WlShm,
    wl_buffer: WlBuffer,
    id_counter: u32,
}

impl Window {
    pub fn run(&mut self) -> Result<(), std::io::Error> {
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
            if event.obj_id == self.xdg_surface_id && event.opcode == XdgSurface::EVENT_CONFIGURE {
                let serial = u32::from_ne_bytes(event.data[0..4].try_into().unwrap());
                if let Some(top_config) = top_config_tmp.take() {
                    self.resize(top_config.width as u32, top_config.height as u32)?;
                    println!("test")
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

pub fn connect() -> Result<Window, std::io::Error> {
    let runtime_dir = env::var("XDG_RUNTIME_DIR")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e))?;
    let wayland_display = env::var("WAYLAND_DISPLAY").unwrap_or("wayland-0".to_string());
    let socket_path: String = format!("{}/{}", runtime_dir, wayland_display);

    let mut stream = UnixStream::connect(&socket_path)?;
    let mut id_counter: u32 = base_ids::ZXDG_DECORATION_MANAGER + 1;

    let (compositor, wl_shm, xdg_wm_base, zxdg_deco_manager) = setup_globals(&mut stream)?;

    let wl_surface = create_wl_surface(&mut stream, &mut id_counter, &compositor)?;
    let (xdg_surface, xdg_surface_id) =
        create_xdg_surface(&mut stream, &mut id_counter, &xdg_wm_base, &wl_surface)?;
    let xdg_toplevel_id = create_xdg_toplevel(&mut stream, &mut id_counter, &xdg_surface)?;
    setup_decoration(
        &mut stream,
        &mut id_counter,
        &zxdg_deco_manager,
        xdg_toplevel_id,
    )?;

    wl_surface.commit(&mut stream)?;
    let serial = wait_for_configure(&mut stream, xdg_surface_id)?;

    let wl_buffer = setup_buffer(&mut stream, &mut id_counter, &wl_shm, 800, 600)?;
    wl_surface.attach(&mut stream, wl_buffer.id)?;
    xdg_surface.ack_configure(&mut stream, serial)?;
    wl_surface.commit(&mut stream)?;

    Ok(Window {
        stream,
        xdg_wm_base,
        xdg_surface,
        xdg_surface_id,
        toplevel_id: xdg_toplevel_id,
        wl_surface,
        wl_shm,
        wl_buffer,
        id_counter,
    })
}

fn next_id(id_counter: &mut u32) -> u32 {
    let id = *id_counter;
    *id_counter += 1;
    id
}

fn setup_globals(
    stream: &mut UnixStream,
) -> Result<(WlCompositor, WlShm, XdgWmBase, ZxdgDecorationManager), std::io::Error> {
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
) -> Result<(XdgSurface, u32), std::io::Error> {
    let xdg_surface_id = next_id(id_counter);
    xdg_wm_base.get_xdg_surface(stream, wl_surface.id, xdg_surface_id)?;
    Ok((XdgSurface { id: xdg_surface_id }, xdg_surface_id))
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

fn wait_for_configure(stream: &mut UnixStream, xdg_surface_id: u32) -> Result<u32, std::io::Error> {
    loop {
        let event = read_event(stream)?;
        if event.obj_id == xdg_surface_id && event.opcode == XdgSurface::EVENT_CONFIGURE {
            return Ok(u32::from_ne_bytes(event.data[0..4].try_into().unwrap()));
        }
    }
}

fn setup_buffer(
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

    loop {
        let event = read_event(stream)?;

        if event.obj_id == base_ids::REGISTRY && event.opcode == WlRegistry::EVENT_GLOBAL {
            let mut cursor = Cursor::new(event.data.as_slice());
            let name = cursor.read_u32();
            let interface = cursor.read_string();
            let version = cursor.read_u32();
            globals.push(WaylandGlobal {
                name,
                interface,
                version,
            });
        }
        if event.obj_id == base_ids::SYNC && event.opcode == WlCallback::EVENT_DONE {
            return Ok(globals);
        }
    }
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
) -> Result<(WlCompositor, WlShm, XdgWmBase, ZxdgDecorationManager), std::io::Error> {
    let bind_map: &[(&str, u32)] = &[
        (interfaces::WL_COMPOSITOR, base_ids::WL_COMPOSITOR),
        (interfaces::WL_SHM, base_ids::WL_SHM),
        (interfaces::XDG_WM_BASE, base_ids::XDG_WM_BASE),
        (
            interfaces::ZXDG_DECORATION_MANAGER,
            base_ids::ZXDG_DECORATION_MANAGER,
        ),
    ];

    let mut to_bind: Vec<(&str, u32, u32, u32)> = bind_map
        .iter()
        .map(|&(iface, bind_id)| {
            let chosen = globals
                .iter()
                .filter(|g| g.interface == iface)
                .min_by_key(|g| g.name)
                .ok_or_else(|| {
                    std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        format!("compositor missing required global: {}", iface),
                    )
                })?;

            Ok((
                iface,
                chosen.name,
                std::cmp::min(chosen.version, supported_version(iface)),
                bind_id,
            ))
        })
        .collect::<Result<_, std::io::Error>>()?;

    to_bind.sort_by_key(|(_, _, _, bind_id)| *bind_id);

    for (iface, global_name, bind_version, bind_id) in to_bind {
        println!("binding id: {}", bind_id);
        stream.write_all(&encode_bind(global_name, iface, bind_version, bind_id))?;
    }

    Ok((
        WlCompositor {
            id: base_ids::WL_COMPOSITOR,
        },
        WlShm {
            id: base_ids::WL_SHM,
        },
        XdgWmBase {
            id: base_ids::XDG_WM_BASE,
        },
        ZxdgDecorationManager {
            id: base_ids::ZXDG_DECORATION_MANAGER,
        },
    ))
}
