pub struct ZwpLinuxDmabufV1 {
    pub id: u32,
}

impl ZwpLinuxDmabufV1 {
    const DESTROY: u32 = 1;
    const CREATE_PARAMS: u32 = 2;
    const GET_DEFAULT_FEEDBACK: u32 = 3;
    const GET_SURFACE_FEEDBACK: u32 = 4;

    pub fn destroy() {
        unimplemented!();
    }

    pub fn create_params() {
        unimplemented!();
    }

    pub fn get_default_feedback() {
        unimplemented!();
    }

    pub fn get_surface_feedback() {
        unimplemented!();
    }
}
