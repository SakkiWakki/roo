// TODO: x11 handling later perhaps
#[cfg_attr(target_os = "linux", path = "wayland/mod.rs")]
#[cfg_attr(target_os = "macos", path = "macos/mod.rs")]
#[cfg_attr(target_os = "windows", path = "windows/mod.rs")]
mod platform;

pub(crate) use platform::windowing::connect;
pub use platform::windowing::Window;

use std::ffi::c_void;

// Rendering stuff
pub struct GpuInfo {
    pub device_node: std::fs::File,
}
