use super::sizes::{HEADER_SIZE, OPCODE_MASK, SIZE_SHIFT, U32_SIZE};
use crate::pal::platform::objects::helper::recv_with_fd;
use crate::pal::platform::WaylandEvent;
use std::os::unix::net::UnixStream;

/// Reads a single Wayland event from the stream
pub fn read_event(stream: &mut UnixStream) -> Result<WaylandEvent, std::io::Error> {
    let mut header = [0u8; HEADER_SIZE];
    let fd_from_header = recv_with_fd(stream, &mut header)?;

    let obj_id = u32::from_ne_bytes(header[0..U32_SIZE].try_into().unwrap());
    let size_opcode = u32::from_ne_bytes(header[U32_SIZE..HEADER_SIZE].try_into().unwrap());
    let size = (size_opcode >> SIZE_SHIFT) as usize;
    let opcode = (size_opcode & OPCODE_MASK) as u16;

    let mut data = vec![0u8; size - HEADER_SIZE];
    let fd_from_payload = if data.is_empty() {
        None
    } else {
        recv_with_fd(stream, &mut data)?
    };

    eprintln!("fd_from_header: {:?}", fd_from_header.map(|f| f.0));
    eprintln!("fd_from_payload: {:?}", fd_from_payload.map(|f| f.0));
    let fd = fd_from_header.or(fd_from_payload);
    Ok(WaylandEvent {
        obj_id,
        opcode,
        data,
        fd,
    })
}
