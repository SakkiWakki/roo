use super::sizes::{HEADER_SIZE, OPCODE_MASK, SIZE_SHIFT, U32_SIZE};
use crate::pal::platform::objects::helper::recv_with_fd;
use crate::pal::platform::WaylandEvent;
use std::os::unix::net::UnixStream;

/// Reads a single Wayland event from the stream
pub fn read_event(stream: &mut UnixStream) -> Result<WaylandEvent, std::io::Error> {
    let mut buf = [0u8; U32_SIZE];
    recv_with_fd(stream, &mut buf)?;
    let obj_id = u32::from_ne_bytes(buf);
    recv_with_fd(stream, &mut buf)?;
    let size_opcode = u32::from_ne_bytes(buf);
    let size = (size_opcode >> SIZE_SHIFT) as usize;
    let opcode = (size_opcode & OPCODE_MASK) as u16;
    let mut data = vec![0u8; size - HEADER_SIZE];
    let fd = recv_with_fd(stream, &mut data)?;
    Ok(WaylandEvent {
        obj_id,
        opcode,
        data,
        fd,
    })
}
