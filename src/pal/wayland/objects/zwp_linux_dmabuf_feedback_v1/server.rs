use super::client::ZwpLinuxDmabufFeedbackV1;

impl ZwpLinuxDmabufFeedbackV1 {
    pub const EVENT_DONE: u16 = 0;
    pub const EVENT_FORMAT_TABLE: u16 = 1;
    pub const EVENT_MAIN_DEVICE: u16 = 2;
    pub const EVENT_TRANCHE_DONE: u16 = 3;
    pub const EVENT_TRANCHE_TARGET_DEVICE: u16 = 4;
    pub const EVENT_TRANCHE_FORMATS: u16 = 5;
    pub const EVENT_TRANCHE_FLAGS: u16 = 6;
}
