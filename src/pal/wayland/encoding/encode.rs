use super::super::{
    ALIGN_MASK, HEADER_SIZE, OBJECT_ID_SIZE, SIZE_SHIFT, STRING_HEADER_SIZE, U32_SIZE, UINT_SIZE,
};
use super::message_builder::MessageBuilder;
use crate::pal::platform::base_ids;
use crate::pal::platform::objects::WlRegistry;

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

    let size_and_opcode = (size << SIZE_SHIFT) | WlRegistry::BIND as u32;
    let mut msg = Vec::with_capacity(size as usize);
    msg.write_u32(registry_id);
    msg.write_u32(size_and_opcode);
    msg.write_u32(name);
    msg.write_string(interface);
    msg.write_u32(version);
    msg.write_u32(assign_id);
    msg
}
