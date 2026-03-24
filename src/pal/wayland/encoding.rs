use crate::pal::platform::objects::WlRegistry;

use super::{base_ids, WaylandEvent};
use super::{
    ALIGN_MASK, HEADER_SIZE, NULL_TERMINATOR, OBJECT_ID_SIZE, OPCODE_MASK, SIZE_SHIFT,
    STRING_HEADER_SIZE, U32_SIZE, UINT_SIZE,
};
use std::io::{Cursor, Read};
use std::os::unix::net::UnixStream;

pub trait MessageReader {
    fn read_u32(&mut self) -> u32;
    fn read_string(&mut self) -> String;
}

fn read_string_impl(r: &mut impl Read) -> String {
    let mut len_buf = [0u8; U32_SIZE];
    r.read_exact(&mut len_buf).unwrap();
    let str_len = u32::from_ne_bytes(len_buf) as usize;
    let padded = (str_len + ALIGN_MASK) & !ALIGN_MASK;
    let mut buf = vec![0u8; padded];
    r.read_exact(&mut buf).unwrap();
    String::from_utf8(buf[..str_len - NULL_TERMINATOR].to_vec()).unwrap()
}

impl MessageReader for UnixStream {
    fn read_u32(&mut self) -> u32 {
        let mut buf = [0u8; U32_SIZE];
        self.read_exact(&mut buf).unwrap();
        u32::from_ne_bytes(buf)
    }

    fn read_string(&mut self) -> String {
        read_string_impl(self)
    }
}

impl MessageReader for Cursor<Vec<u8>> {
    fn read_u32(&mut self) -> u32 {
        let mut buf = [0u8; U32_SIZE];
        self.read_exact(&mut buf).unwrap();
        u32::from_ne_bytes(buf)
    }

    fn read_string(&mut self) -> String {
        read_string_impl(self)
    }
}

pub trait MessageBuilder {
    fn write_u32(&mut self, val: u32);
    fn write_string(&mut self, val: &str);
}

impl MessageBuilder for Vec<u8> {
    fn write_u32(&mut self, val: u32) {
        self.extend_from_slice(&val.to_ne_bytes());
    }

    fn write_string(&mut self, val: &str) {
        let bytes = val.as_bytes();
        let str_len = bytes.len() + NULL_TERMINATOR;
        let str_padded = (str_len + ALIGN_MASK) & !ALIGN_MASK;
        self.write_u32(str_len as u32);
        self.extend_from_slice(bytes);
        let padding = str_padded - bytes.len();
        for _ in 0..padding {
            self.push(0u8);
        }
    }
}

/// Initializes a Wayland message Vec with the two-word header
pub fn build_msg(obj_id: u32, size: u32, opcode: u32) -> Vec<u8> {
    let mut msg = Vec::with_capacity(size as usize);
    msg.write_u32(obj_id);
    msg.write_u32((size << SIZE_SHIFT) | opcode);
    msg
}

/// Encodes a generic Wayland message with a single u32 arg
pub fn encode_op(obj_id: u32, arg: u32, opcode: u32) -> Vec<u8> {
    let size = (HEADER_SIZE + U32_SIZE) as u32;
    let size_and_opcode = (size << SIZE_SHIFT) | opcode;
    let mut msg = Vec::with_capacity(size as usize);
    msg.write_u32(obj_id);
    msg.write_u32(size_and_opcode);
    msg.write_u32(arg);
    msg
}

/// Encodes a wl_registry.bind message (opcode 0)
/// Check wayland.xml for wire format
pub fn encode_bind(name: u32, interface: &str, version: u32, assign_id: u32) -> Vec<u8> {
    println!(
        "bind: name={} interface={} version={} id={}",
        name, interface, version, assign_id
    );
    let registry_id = base_ids::REGISTRY;
    let str_len = interface.len() + 1;
    let str_padded = (str_len + ALIGN_MASK) & !ALIGN_MASK;
    let size = (HEADER_SIZE
        + OBJECT_ID_SIZE
        + STRING_HEADER_SIZE
        + str_padded
        + UINT_SIZE
        + OBJECT_ID_SIZE) as u32;

    let size_and_opcode = (size << SIZE_SHIFT) | WlRegistry::BIND;
    let mut msg = Vec::with_capacity(size as usize);
    msg.write_u32(registry_id);
    msg.write_u32(size_and_opcode);
    msg.write_u32(name);
    msg.write_string(interface);
    msg.write_u32(version);
    msg.write_u32(assign_id);
    msg
}

/// Reads a single Wayland event from the stream
pub fn read_event(stream: &mut UnixStream) -> Result<WaylandEvent, std::io::Error> {
    let mut buf = [0u8; U32_SIZE];
    stream.read_exact(&mut buf)?;
    let obj_id = u32::from_ne_bytes(buf);
    stream.read_exact(&mut buf)?;
    let size_opcode = u32::from_ne_bytes(buf);
    let size = (size_opcode >> SIZE_SHIFT) as usize;
    let opcode = (size_opcode & OPCODE_MASK) as u16;
    let mut data = vec![0u8; size - HEADER_SIZE];
    stream.read_exact(&mut data)?;
    if obj_id == 1 && opcode == 0 {
        let failed_obj = u32::from_ne_bytes(data[0..4].try_into().unwrap());
        let code = u32::from_ne_bytes(data[4..8].try_into().unwrap());
        let str_len = u32::from_ne_bytes(data[8..12].try_into().unwrap()) as usize;
        let msg = String::from_utf8_lossy(&data[12..12 + str_len.saturating_sub(1)]);
        eprintln!(
            "[wayland] ERROR: failed_obj={} code={} msg={}",
            failed_obj, code, msg
        );
    }
    Ok(WaylandEvent {
        obj_id,
        opcode,
        data,
    })
}
