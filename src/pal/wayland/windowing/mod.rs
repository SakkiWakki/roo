//! See https://wayland-book.com/protocol-design/wire-protocol.html
pub(crate) mod connect;
pub mod event_loop;
mod window;

pub use connect::connect;
pub use window::Window;
