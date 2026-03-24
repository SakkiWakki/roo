//! See https://wayland-book.com/protocol-design/wire-protocol.html
use std::env;
use std::io::Cursor;
use std::io::Write;
use std::os::unix::net::UnixStream;

use crate::pal::platform::objects::ZxdgDecorationManager;
use crate::pal::platform::objects::ZxdgToplevelDecoration;
use crate::pal::platform::zxdg_decoration_manager;
use crate::pal::platform::zxdg_toplevel_decoration;

use super::encoding::{encode_bind, encode_op, read_event, MessageReader};
use super::{
    base_ids, interfaces, wl_callback_event, wl_display, wl_registry_event, xdg_toplevel,
    WaylandGlobal, WlCompositor, WlShm, WlSurface, XdgSurface, XdgWmBase,
};

pub struct Window {
    stream: UnixStream,
    xdg_wm_base: XdgWmBase,
    xdg_surface: XdgSurface,
    xdg_surface_id: u32,
    toplevel_id: u32,
    wl_surface: WlSurface,
}

impl Window {
    pub fn run(&mut self) -> Result<(), std::io::Error> {
        loop {
            let event = read_event(&mut self.stream)?;
            if event.obj_id == base_ids::WL_DISPLAY && event.opcode == wl_display::event::ERROR {
                let mut cursor = Cursor::new(event.data);
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
            if event.obj_id == self.xdg_surface_id && event.opcode == XdgSurface::EVENT_CONFIGURE {
                let serial = u32::from_ne_bytes(event.data[0..4].try_into().unwrap());
                self.xdg_surface.ack_configure(&mut self.stream, serial)?;
                self.wl_surface.commit(&mut self.stream)?;
            }
            if event.obj_id == self.toplevel_id && event.opcode == xdg_toplevel::event::CLOSE {
                break;
            }
        }
        Ok(())
    }
}

pub fn connect() -> Result<Window, std::io::Error> {
    let runtime_dir = env::var("XDG_RUNTIME_DIR")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e))?;
    let wayland_display = env::var("WAYLAND_DISPLAY").unwrap_or("wayland-0".to_string());
    let socket_path: String = format!("{}/{}", runtime_dir, wayland_display);
    let mut id_counter: u32 = base_ids::ZXDG_DECORATION_MANAGER + 1;

    let mut stream = UnixStream::connect(&socket_path)?;

    println!("Setting wl display id: {}", base_ids::WL_DISPLAY);
    println!("Setting registry id: {}", base_ids::REGISTRY);
    stream.write_all(&encode_op(
        base_ids::WL_DISPLAY,
        base_ids::REGISTRY,
        wl_display::GET_REGISTRY,
    ))?;

    println!("Setting sync id: {}", base_ids::SYNC);
    stream.write_all(&encode_op(
        base_ids::WL_DISPLAY,
        base_ids::SYNC,
        wl_display::SYNC,
    ))?;

    let globals = read_until_sync(&mut stream)?;

    let (compositor, shm, xdg_wm_base, zxdg_deco_manager) = create_window_bindings(&mut stream, globals)?;
    let mut next_id = || {
        let id = id_counter;
        id_counter += 1;
        id
    };

    let surface_id = next_id();
    compositor.create_surface(&mut stream, surface_id)?;
    let wl_surface = WlSurface { id: surface_id };

    let xdg_surface_id = next_id();
    xdg_wm_base.get_xdg_surface(&mut stream, surface_id, xdg_surface_id)?;
    let xdg_surface = XdgSurface { id: xdg_surface_id };

    let xdg_toplevel_id = next_id();
    xdg_surface.get_toplevel(&mut stream, xdg_toplevel_id)?;

    let zxdg_deco_id = next_id();
    zxdg_deco_manager.get_toplevel_decoration(&mut stream, zxdg_deco_id, xdg_toplevel_id)?;
    let zxdg_top_deco = ZxdgToplevelDecoration { id: zxdg_deco_id };
    zxdg_top_deco.set_mode(&mut stream, 2)?; // Serverside

    wl_surface.commit(&mut stream)?;
    let serial = wait_for_configure(&mut stream, xdg_surface_id)?;
    xdg_surface.ack_configure(&mut stream, serial)?;

    let pool_id = next_id();
    let buffer_id = next_id();
    let mut buffer = shm.alloc_buffer(&mut stream, pool_id, buffer_id, 800, 600)?;

    buffer.fill(0xFF000000);
    wl_surface.attach(&mut stream, buffer_id)?;
    wl_surface.commit(&mut stream)?;

    Ok(Window {
        stream,
        xdg_wm_base,
        xdg_surface,
        xdg_surface_id,
        toplevel_id: xdg_toplevel_id,
        wl_surface,
    })
}

fn read_until_sync(stream: &mut UnixStream) -> Result<Vec<WaylandGlobal>, std::io::Error> {
    let mut globals: Vec<WaylandGlobal> = Vec::new();

    loop {
        let event = read_event(stream)?;

        if event.obj_id == base_ids::REGISTRY && event.opcode == wl_registry_event::GLOBAL {
            let mut cursor = Cursor::new(event.data);
            let name = cursor.read_u32();
            let interface = cursor.read_string();
            let version = cursor.read_u32();
            globals.push(WaylandGlobal {
                name,
                interface,
                version,
            });
        }
        if event.obj_id == base_ids::SYNC && event.opcode == wl_callback_event::DONE {
            return Ok(globals);
        }
    }
}

fn wait_for_configure(stream: &mut UnixStream, xdg_surface_id: u32) -> Result<u32, std::io::Error> {
    loop {
        let event = read_event(stream)?;
        if event.obj_id == xdg_surface_id && event.opcode == XdgSurface::EVENT_CONFIGURE {
            return Ok(u32::from_ne_bytes(event.data[0..4].try_into().unwrap()));
        }
    }
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

/// Bind in monotonically increasing id order for the backends that need it to be like this
fn create_window_bindings(
    stream: &mut UnixStream,
    globals: Vec<WaylandGlobal>,
) -> Result<(WlCompositor, WlShm, XdgWmBase, ZxdgDecorationManager), std::io::Error> {
    for &required in interfaces::REQUIRED {
        if !globals.iter().any(|g| g.interface == required) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("compositor missing required global: {}", required),
            ));
        }
    }

    let bind_map: &[(&str, u32)] = &[
        (interfaces::WL_COMPOSITOR, base_ids::WL_COMPOSITOR),
        (interfaces::WL_SHM, base_ids::WL_SHM),
        (interfaces::XDG_WM_BASE, base_ids::XDG_WM_BASE),
        (interfaces::ZXDG_DECORATION_MANAGER, base_ids::ZXDG_DECORATION_MANAGER),
    ];

    let mut to_bind: Vec<(&str, u32, u32, u32)> = Vec::new();

    for &(iface, bind_id) in bind_map {
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

        let bind_version = std::cmp::min(chosen.version, supported_version(iface));
        to_bind.push((iface, chosen.name, bind_version, bind_id));
    }

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