use super::encoding::{build_msg, encode_op, MessageBuilder};
use super::{HEADER_SIZE, U32_SIZE};
use std::io::Write;
use std::os::unix::io::AsRawFd;
use std::os::unix::net::UnixStream;

/// Sends a message with an attached file descriptor via SCM_RIGHTS
fn send_with_fd(stream: &mut UnixStream, msg: &[u8], fd: i32) -> Result<(), std::io::Error> {
    use std::mem::size_of;
    unsafe {
        let iov = libc::iovec {
            iov_base: msg.as_ptr() as *mut _,
            iov_len: msg.len(),
        };
        let cmsg_space = libc::CMSG_SPACE(size_of::<i32>() as u32) as usize;
        let mut cmsg_buf = vec![0u8; cmsg_space];
        let mhdr = libc::msghdr {
            msg_name: std::ptr::null_mut(),
            msg_namelen: 0,
            msg_iov: &iov as *const _ as *mut _,
            msg_iovlen: 1,
            msg_control: cmsg_buf.as_mut_ptr() as *mut _,
            msg_controllen: cmsg_space,
            msg_flags: 0,
        };
        let cmsg = libc::CMSG_FIRSTHDR(&mhdr);
        (*cmsg).cmsg_level = libc::SOL_SOCKET;
        (*cmsg).cmsg_type = libc::SCM_RIGHTS;
        (*cmsg).cmsg_len = libc::CMSG_LEN(size_of::<i32>() as u32) as _;
        std::ptr::copy_nonoverlapping(
            &fd as *const i32 as *const u8,
            libc::CMSG_DATA(cmsg),
            size_of::<i32>(),
        );
        let ret = libc::sendmsg(stream.as_raw_fd(), &mhdr, 0);
        if ret < 0 {
            return Err(std::io::Error::last_os_error());
        }
        if ret as usize != msg.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::WriteZero,
                format!("sendmsg sent {} bytes, expected {}", ret, msg.len()),
            ));
        }
        Ok(())
    }
}

pub struct WlShm {
    pub id: u32,
}

impl WlShm {
    pub const CREATE_POOL: u32 = 0;
    pub const FORMAT_XRGB8888: u32 = 1;

    pub fn create_pool(
        &self,
        stream: &mut UnixStream,
        fd: i32,
        size: i32,
        new_id: u32,
    ) -> Result<(), std::io::Error> {
        let msg_size: u32 = (HEADER_SIZE + U32_SIZE * 2) as u32;
        let mut msg = build_msg(self.id, msg_size, Self::CREATE_POOL);
        msg.write_u32(new_id);
        msg.write_u32(size as u32);
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

        self.create_pool(stream, fd, size as i32, new_pool_id)?;
        unsafe { libc::close(fd) };
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

pub struct WlShmPool {
    pub id: u32,
}

impl WlShmPool {
    pub const CREATE_BUFFER: u32 = 0;
    pub const DESTROY: u32 = 1;

    pub fn create_buffer(
        &self,
        stream: &mut UnixStream,
        new_id: u32,
        offset: i32,
        width: i32,
        height: i32,
        stride: i32,
        format: u32,
    ) -> Result<(), std::io::Error> {
        let msg_size: u32 = (HEADER_SIZE + U32_SIZE * 6) as u32;
        let mut msg = build_msg(self.id, msg_size, Self::CREATE_BUFFER);
        msg.write_u32(new_id);
        msg.write_u32(offset as u32);
        msg.write_u32(width as u32);
        msg.write_u32(height as u32);
        msg.write_u32(stride as u32);
        msg.write_u32(format);
        stream.write_all(&msg)?;
        Ok(())
    }
}

pub fn create_memfd(size: usize) -> Result<(i32, *mut u8), std::io::Error> {
    unsafe {
        let fd = libc::memfd_create(b"wl-buffer\0".as_ptr() as *const _, 0);
        if fd == -1 {
            // Even without this check, it should still fail on ftruncate
            return Err(std::io::Error::last_os_error());
        }
        if libc::ftruncate(fd, size as i64) == -1 {
            libc::close(fd);
            return Err(std::io::Error::last_os_error());
        }
        let ptr = libc::mmap(
            std::ptr::null_mut(),
            size,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd,
            0,
        );
        if ptr == libc::MAP_FAILED {
            libc::close(fd);
            return Err(std::io::Error::last_os_error());
        }
        Ok((fd, ptr as *mut u8))
    }
}

pub struct WlBuffer {
    pub id: u32,
    pub ptr: *mut u8,
    pub size: usize,
}

impl WlBuffer {
    pub const DESTROY: u32 = 0;

    /// TODO: Change
    pub fn fill(&mut self, argb: u32) {
        let pixels = self.size / 4;
        let buf = unsafe { std::slice::from_raw_parts_mut(self.ptr as *mut u32, pixels) };
        buf.fill(argb);
    }
}

impl Drop for WlBuffer {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { libc::munmap(self.ptr as *mut _, self.size) };
        }
    }
}

pub struct WlCompositor {
    pub id: u32,
}

impl WlCompositor {
    pub const CREATE_SURFACE: u32 = 0;

    pub fn create_surface(
        &self,
        stream: &mut UnixStream,
        new_id: u32,
    ) -> Result<(), std::io::Error> {
        stream.write_all(&encode_op(self.id, new_id, Self::CREATE_SURFACE))?;
        Ok(())
    }
}

pub struct XdgWmBase {
    pub id: u32,
}

impl XdgWmBase {
    pub const GET_XDG_SURFACE: u32 = 2;
    pub const PONG: u32 = 3;
    pub const EVENT_PING: u16 = 0;

    pub fn get_xdg_surface(
        &self,
        stream: &mut UnixStream,
        surface_id: u32,
        new_id: u32,
    ) -> Result<(), std::io::Error> {
        let msg_size: u32 = (HEADER_SIZE + U32_SIZE * 2) as u32;
        let mut msg = build_msg(self.id, msg_size, Self::GET_XDG_SURFACE);
        msg.write_u32(new_id);
        msg.write_u32(surface_id);
        stream.write_all(&msg)?;
        Ok(())
    }

    pub fn pong(&self, stream: &mut UnixStream, serial: u32) -> Result<(), std::io::Error> {
        stream.write_all(&encode_op(self.id, serial, Self::PONG))?;
        Ok(())
    }
}

pub struct XdgSurface {
    pub id: u32,
}

impl XdgSurface {
    pub const GET_TOPLEVEL: u32 = 1;
    pub const ACK_CONFIGURE: u32 = 4;
    pub const EVENT_CONFIGURE: u16 = 0;

    pub fn get_toplevel(&self, stream: &mut UnixStream, new_id: u32) -> Result<(), std::io::Error> {
        stream.write_all(&encode_op(self.id, new_id, Self::GET_TOPLEVEL))?;
        Ok(())
    }

    pub fn ack_configure(
        &self,
        stream: &mut UnixStream,
        serial: u32,
    ) -> Result<(), std::io::Error> {
        stream.write_all(&encode_op(self.id, serial, Self::ACK_CONFIGURE))?;
        Ok(())
    }
}

pub struct WlSurface {
    pub id: u32,
}

impl WlSurface {
    pub const ATTACH: u32 = 1;
    pub const DAMAGE: u32 = 2;
    pub const COMMIT: u32 = 6;

    pub fn attach(&self, stream: &mut UnixStream, buffer_id: u32) -> Result<(), std::io::Error> {
        let msg_size: u32 = (HEADER_SIZE + U32_SIZE * 3) as u32;
        let mut msg = build_msg(self.id, msg_size, Self::ATTACH);
        msg.write_u32(buffer_id);
        msg.write_u32(0); // x offset
        msg.write_u32(0); // y offset
        stream.write_all(&msg)?;
        Ok(())
    }

    pub fn damage(
        &self,
        stream: &mut UnixStream,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), std::io::Error> {
        let msg_size: u32 = (HEADER_SIZE + U32_SIZE * 4) as u32;
        let mut msg = build_msg(self.id, msg_size, Self::DAMAGE);
        msg.write_u32(x as u32);
        msg.write_u32(y as u32);
        msg.write_u32(width as u32);
        msg.write_u32(height as u32);
        stream.write_all(&msg)?;
        Ok(())
    }

    pub fn commit(&self, stream: &mut UnixStream) -> Result<(), std::io::Error> {
        let msg = build_msg(self.id, HEADER_SIZE as u32, Self::COMMIT);
        stream.write_all(&msg)?;
        Ok(())
    }
}
