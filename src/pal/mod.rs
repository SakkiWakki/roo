// TODO: x11 handling later perhaps
#[cfg_attr(target_os = "linux", path = "wayland/mod.rs")]
#[cfg_attr(target_os = "macos", path = "macos/mod.rs")]
#[cfg_attr(target_os = "windows", path = "windows/mod.rs")]
mod platform;

pub(crate) use platform::windowing::connect;
