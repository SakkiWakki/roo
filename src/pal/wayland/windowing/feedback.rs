use std::os::unix::net::UnixStream;

use crate::pal::platform::objects::{FeedbackState, ZwpLinuxDmabufFeedbackV1, ZwpLinuxDmabufV1};
use crate::pal::platform::types::DmabufFeedback;

use super::event_loop::event_loop;
use super::surfaces::next_id;

pub fn dmabuf_feedback(
    stream: &mut UnixStream,
    id_counter: &mut u32,
    dmabuf: &ZwpLinuxDmabufV1,
) -> Result<DmabufFeedback, std::io::Error> {
    let dmabuf_feedback_id = next_id(id_counter);
    dmabuf.get_default_feedback(stream, dmabuf_feedback_id)?;
    loop_until_feedback(stream, dmabuf_feedback_id)
}

fn loop_until_feedback(
    stream: &mut UnixStream,
    feedback_id: u32,
) -> Result<DmabufFeedback, std::io::Error> {
    let mut state = FeedbackState::default();
    let handlers = [
        (
            feedback_id,
            ZwpLinuxDmabufFeedbackV1::EVENT_MAIN_DEVICE,
            ZwpLinuxDmabufFeedbackV1::handle_main_device as _,
        ),
        (
            feedback_id,
            ZwpLinuxDmabufFeedbackV1::EVENT_FORMAT_TABLE,
            ZwpLinuxDmabufFeedbackV1::handle_format_table as _,
        ),
        (
            feedback_id,
            ZwpLinuxDmabufFeedbackV1::EVENT_TRANCHE_TARGET_DEVICE,
            ZwpLinuxDmabufFeedbackV1::handle_tranche_target_device as _,
        ),
        (
            feedback_id,
            ZwpLinuxDmabufFeedbackV1::EVENT_TRANCHE_FORMATS,
            ZwpLinuxDmabufFeedbackV1::handle_tranche_formats as _,
        ),
        (
            feedback_id,
            ZwpLinuxDmabufFeedbackV1::EVENT_TRANCHE_FLAGS,
            ZwpLinuxDmabufFeedbackV1::handle_tranche_flags as _,
        ),
        (
            feedback_id,
            ZwpLinuxDmabufFeedbackV1::EVENT_TRANCHE_DONE,
            ZwpLinuxDmabufFeedbackV1::handle_tranche_done as _,
        ),
        (
            feedback_id,
            ZwpLinuxDmabufFeedbackV1::EVENT_DONE,
            ZwpLinuxDmabufFeedbackV1::handle_done as _,
        ),
    ];
    event_loop(stream, &mut state, &handlers)?;
    Ok(DmabufFeedback {
        main_device: state.main_device,
        tranches: state.tranches,
    })
}
