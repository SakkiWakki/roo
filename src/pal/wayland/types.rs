#[derive(Clone, Copy)]
pub struct Fd(pub i32);

pub struct WaylandGlobal {
    pub name: u32,
    pub interface: String,
    pub version: u32,
}

pub struct WaylandEvent {
    pub obj_id: u32,
    pub opcode: u16,
    pub data: Vec<u8>,
    pub fd: Option<Fd>,
}

pub struct DmabufFeedback {
    pub main_device: u64,
    pub formats: Vec<(u32, u64)>,
}
