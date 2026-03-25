pub struct ZwpLinuxBufferParamsV1 {
    pub id: u32,
}

impl ZwpLinuxBufferParamsV1 {
    const DESTROY: u32 = 1;
    const ADD: u32 = 2;
    const CREATE: u32 = 3;
    const CREATE_IMMED: u32 = 4;
}
