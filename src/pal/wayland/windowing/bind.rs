use super::protocol::{base_ids, interfaces};
use crate::pal::platform::objects::{
    WlCompositor, WlShm, XdgWmBase, ZwpLinuxDmabufV1, ZxdgDecorationManager,
};

pub trait Bind {
    const INTERFACE: &'static str;
    const BIND_ID: u32;

    fn new() -> Self;
}

impl Bind for WlCompositor {
    const INTERFACE: &'static str = interfaces::WL_COMPOSITOR;
    const BIND_ID: u32 = base_ids::WL_COMPOSITOR;

    fn new() -> Self {
        Self { id: Self::BIND_ID }
    }
}

impl Bind for WlShm {
    const INTERFACE: &'static str = interfaces::WL_SHM;
    const BIND_ID: u32 = base_ids::WL_SHM;

    fn new() -> Self {
        Self { id: Self::BIND_ID }
    }
}

impl Bind for XdgWmBase {
    const INTERFACE: &'static str = interfaces::XDG_WM_BASE;
    const BIND_ID: u32 = base_ids::XDG_WM_BASE;

    fn new() -> Self {
        Self { id: Self::BIND_ID }
    }
}

impl Bind for ZxdgDecorationManager {
    const INTERFACE: &'static str = interfaces::ZXDG_DECORATION_MANAGER;
    const BIND_ID: u32 = base_ids::ZXDG_DECORATION_MANAGER;

    fn new() -> Self {
        Self { id: Self::BIND_ID }
    }
}

impl Bind for ZwpLinuxDmabufV1 {
    const INTERFACE: &'static str = interfaces::ZWP_LINUX_DMABUF;
    const BIND_ID: u32 = base_ids::ZWP_LINUX_DMABUF;

    fn new() -> Self {
        Self { id: Self::BIND_ID }
    }
}

#[macro_export]
macro_rules! wl_bind {
    ($($ty:ty => ($iface:expr, $id:expr)),* $(,)?) => {
        $(impl Bind for $ty {
            const INTERFACE: &'static str = $iface;
            const BIND_ID: u32 = $id;
            fn new() -> Self { Self { id: Self::BIND_ID } }
        })*
    };
}
