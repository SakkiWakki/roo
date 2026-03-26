// TODO: x11 handling later perhaps
#[cfg_attr(target_os = "linux", path = "wayland/mod.rs")]
#[cfg_attr(target_os = "macos", path = "macos/mod.rs")]
#[cfg_attr(target_os = "windows", path = "windows/mod.rs")]
mod platform;

pub use platform::windowing::connect;
pub use platform::windowing::Window;

pub struct SupportedFormat {
    pub drm_format: u32,
    pub modifier: u64,
}

pub trait PlatformWindow {
    fn resolution(&self) -> (u32, u32);
    fn gpu_info(&mut self) -> Option<GpuInfo>;
    fn formats(&self) -> &[SupportedFormat];
    fn import_dmabuf(&mut self, fd: i32, format: u32, modifier: u64) -> Result<(), std::io::Error>;
    fn run(&mut self) -> Result<(), std::io::Error>;
}

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::{LinuxLoader, Symbol};

// Rendering stuff
pub struct GpuInfo {
    pub device_node: std::fs::File,
}
