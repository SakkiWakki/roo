//! See https://wayland-book.com/protocol-design/wire-protocol.html
pub(crate) mod connect;
pub mod event_loop;
pub mod protocol;
mod window;

pub use connect::connect;
pub use protocol::*;
pub use window::Window;
