use crate::pal::platform::base_ids;
use crate::pal::platform::objects::WlRegistry;
use crate::write_msg;

/// Encodes a wl_registry.bind message (opcode 0)
/// Check wayland.xml for wire format
pub fn encode_bind(name: u32, interface: &str, version: u32, assign_id: u32) -> Vec<u8> {
    println!(
        "bind: name={} interface={} version={} id={}",
        name, interface, version, assign_id
    );
    write_msg!(
        base_ids::REGISTRY,
        WlRegistry::BIND,
        name,
        interface,
        version,
        assign_id
    )
}

/// Encodes a generic Wayland message with a single u32 arg
pub fn encode_op(obj_id: u32, arg: u32, opcode: u32) -> Vec<u8> {
    write_msg!(obj_id, opcode, arg)
}
