pub struct WaylandGlobal {
    pub name: u32,
    pub interface: String,
    pub version: u32,
}

pub struct WaylandEvent {
    pub obj_id: u32,
    pub opcode: u16,
    pub data: Vec<u8>,
}
