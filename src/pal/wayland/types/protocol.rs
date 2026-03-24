pub mod base_ids {
    pub const WL_DISPLAY: u32 = 1;
    pub const REGISTRY: u32 = 2;
    pub const SYNC: u32 = 3;
    pub const WL_COMPOSITOR: u32 = 4;
    pub const WL_SHM: u32 = 5;
    pub const XDG_WM_BASE: u32 = 6;
    pub const ZXDG_DECORATION_MANAGER: u32 = 7;
}

pub mod wl_display {
    pub const SYNC: u32 = 0;
    pub const GET_REGISTRY: u32 = 1;

    pub mod event {
        pub const ERROR: u16 = 0;
    }
}

pub mod wl_registry_opcode {
    pub const BIND: u32 = 0;
}

pub mod wl_registry_event {
    pub const GLOBAL: u16 = 0;
    pub const GLOBAL_REMOVE: u16 = 1;
}

pub mod wl_callback_event {
    pub const DONE: u16 = 0;
}

pub mod xdg_toplevel {
    pub mod event {
        pub const CONFIGURE: u16 = 0;
        pub const CLOSE: u16 = 1;
    }
}

pub mod zxdg_decoration_manager_v1 {
    pub const GET_TOPLEVEL_DECORATION: u32 = 1;
}

pub mod zxdg_toplevel_decoration_v1 {
    pub const SET_MODE: u32 = 1;
    pub const MODE_SERVER_SIDE: u32 = 2;
}

pub mod interfaces {
    pub const WL_COMPOSITOR: &str = "wl_compositor";
    pub const WL_SHM: &str = "wl_shm";
    pub const XDG_WM_BASE: &str = "xdg_wm_base";
    pub const ZXDG_DECORATION_MANAGER: &str = "zxdg_decoration_manager_v1";

    pub const REQUIRED: &[&str] = &[WL_COMPOSITOR, WL_SHM, XDG_WM_BASE];

    pub enum RequiredInterface {
        WlCompositor = 0,
        WlShm = 1,
        XdgWmBase = 2,
    }
}
