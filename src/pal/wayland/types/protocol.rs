pub mod base_ids {
    pub const WL_DISPLAY: u32 = 1;
    pub const REGISTRY: u32 = 2;
    pub const SYNC: u32 = 3;
    pub const WL_COMPOSITOR: u32 = 4;
    pub const WL_SHM: u32 = 5;
    pub const XDG_WM_BASE: u32 = 6;
    pub const ZXDG_DECORATION_MANAGER: u32 = 7;
}

pub mod interfaces {
    pub const WL_COMPOSITOR: &str = "wl_compositor";
    pub const WL_SHM: &str = "wl_shm";
    pub const XDG_WM_BASE: &str = "xdg_wm_base";
    pub const ZXDG_DECORATION_MANAGER: &str = "zxdg_decoration_manager_v1";

    pub const REQUIRED: &[&str] = &[WL_COMPOSITOR, WL_SHM, XDG_WM_BASE, ZXDG_DECORATION_MANAGER];
}
