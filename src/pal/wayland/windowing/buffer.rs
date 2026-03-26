use std::os::unix::net::UnixStream;

use crate::pal::platform::objects::{WlBuffer, WlShm};

use super::surfaces::next_id;

pub fn setup_buffer(
    stream: &mut UnixStream,
    id_counter: &mut u32,
    wl_shm: &WlShm,
    width: i32,
    height: i32,
) -> Result<WlBuffer, std::io::Error> {
    let pool_id = next_id(id_counter);
    let wl_buffer_id = next_id(id_counter);
    let mut wl_buffer = wl_shm.alloc_buffer(stream, pool_id, wl_buffer_id, width, height)?;
    wl_buffer.fill(0xFF000000);
    Ok(wl_buffer)
}
