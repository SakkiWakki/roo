//! See https://wayland-book.com/protocol-design/wire-protocol.html
pub mod bind;
pub(crate) mod buffer;
pub(crate) mod connect;
pub mod event_loop;
pub(crate) mod feedback;
pub(crate) mod globals;
pub mod protocol;
pub(crate) mod surfaces;
mod window;

pub use connect::connect;
pub use protocol::*;
pub use window::Window;
