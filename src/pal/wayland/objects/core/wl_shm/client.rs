use super::super::super::super::types::Fd;
use super::super::super::helper::{create_memfd, send_with_fd};
use super::super::wl_buffer::WlBuffer;
use super::super::wl_shm_pool::WlShmPool;
use crate::write_msg;
use std::os::unix::net::UnixStream;

pub struct WlShm {
    pub id: u32,
}

impl WlShm {
    pub const CREATE_POOL: u32 = 0;
    pub const FORMAT_XRGB8888: u32 = 1;

    pub fn create_pool(
        &self,
        stream: &mut UnixStream,
        fd: Fd,
        size: i32,
        new_id: u32,
    ) -> Result<(), std::io::Error> {
        let msg = write_msg!(self.id, Self::CREATE_POOL, new_id, size as u32);
        send_with_fd(stream, &msg, fd)
    }

    pub fn alloc_buffer(
        &self,
        stream: &mut UnixStream,
        new_pool_id: u32,
        new_buffer_id: u32,
        width: i32,
        height: i32,
    ) -> Result<WlBuffer, std::io::Error> {
        let stride = width * 4;
        let size = (stride * height) as usize;

        let (fd, ptr) = create_memfd(size)?;

        self.create_pool(stream, Fd(fd.0), size as i32, new_pool_id)?;
        unsafe { libc::close(fd.0) };
        let pool = WlShmPool { id: new_pool_id };
        pool.create_buffer(
            stream,
            new_buffer_id,
            0,
            width,
            height,
            stride,
            Self::FORMAT_XRGB8888,
        )?;

        Ok(WlBuffer {
            id: new_buffer_id,
            ptr,
            size,
        })
    }
}
