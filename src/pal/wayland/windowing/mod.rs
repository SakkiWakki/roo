//! See https://wayland-book.com/protocol-design/wire-protocol.html
mod connect;
mod window;

pub use connect::connect;
pub use window::Window;
