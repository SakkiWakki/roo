use std::mem::size_of;
use std::os::unix::io::AsRawFd;
use std::os::unix::net::UnixStream;

use super::super::types::Fd;

/// Sends a message with an attached file descriptor via SCM_RIGHTS
pub fn send_with_fd(stream: &mut UnixStream, msg: &[u8], fd: Fd) -> Result<(), std::io::Error> {
    let fd = fd.0;
    unsafe {
        let cmsg_space = libc::CMSG_SPACE(size_of::<i32>() as u32) as usize;
        let mut cmsg_buf = vec![0u8; cmsg_space];
        let iov = create_iov(msg);
        let mhdr = create_msghdr(&iov, &mut cmsg_buf);
        let cmsg = create_cmsg(&mhdr);
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

pub fn recv_with_fd(
    stream: &mut UnixStream,
    recv_buf: &mut [u8],
) -> Result<Option<Fd>, std::io::Error> {
    unsafe {
        let cmsg_space = libc::CMSG_SPACE(size_of::<i32>() as u32) as usize;
        let mut cmsg_buf = vec![0u8; cmsg_space];
        let iov = create_iov(recv_buf);
        let mut mhdr = create_msghdr(&iov, &mut cmsg_buf);
        let ret = libc::recvmsg(stream.as_raw_fd(), &mut mhdr, 0);
        if ret < 0 {
            return Err(std::io::Error::last_os_error());
        }
        if ret as usize != recv_buf.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::WriteZero,
                format!(
                    "recvmsg received {} bytes, expected {}",
                    ret,
                    recv_buf.len()
                ),
            ));
        }
        let cmsg = libc::CMSG_FIRSTHDR(&mhdr);
        if cmsg.is_null() {
            return Ok(None);
        }
        let mut fd: i32 = -1;
        std::ptr::copy_nonoverlapping(
            libc::CMSG_DATA(cmsg),
            &mut fd as *mut i32 as *mut u8,
            size_of::<i32>(),
        );
        Ok(Some(Fd(fd)))
    }
}

fn create_iov(buf: &[u8]) -> libc::iovec {
    libc::iovec {
        iov_base: buf.as_ptr() as *mut _,
        iov_len: buf.len(),
    }
}

fn create_msghdr(iov: &libc::iovec, cmsg_buf: &mut Vec<u8>) -> libc::msghdr {
    libc::msghdr {
        msg_name: std::ptr::null_mut(),
        msg_namelen: 0,
        msg_iov: iov as *const _ as *mut _,
        msg_iovlen: 1,
        msg_control: cmsg_buf.as_mut_ptr() as *mut _,
        msg_controllen: cmsg_buf.len(),
        msg_flags: 0,
    }
}

fn create_cmsg(msghdr: &libc::msghdr) -> *mut libc::cmsghdr {
    unsafe {
        let cmsg = libc::CMSG_FIRSTHDR(msghdr);
        (*cmsg).cmsg_level = libc::SOL_SOCKET;
        (*cmsg).cmsg_type = libc::SCM_RIGHTS;
        (*cmsg).cmsg_len = libc::CMSG_LEN(size_of::<i32>() as u32) as _;
        cmsg
    }
}

pub trait ReadNeAt: Sized {
    fn read_ne_at(bytes: &[u8], offset: usize) -> Self;
}

macro_rules! impl_read_ne_at {
    ($ty:ty) => {
        impl ReadNeAt for $ty {
            fn read_ne_at(bytes: &[u8], offset: usize) -> Self {
                Self::from_ne_bytes(
                    bytes[offset..offset + size_of::<Self>()]
                        .try_into()
                        .unwrap(),
                )
            }
        }
    };
}

impl_read_ne_at!(u32);
impl_read_ne_at!(u64);
impl_read_ne_at!(i32);
impl_read_ne_at!(i64);

#[macro_export]
macro_rules! read_at {
    ($bytes:expr, $offset:expr, $ty:ty) => {
        <$ty as $crate::pal::platform::objects::helper::ReadNeAt>::read_ne_at($bytes, $offset)
    };
}

pub fn create_memfd(size: usize) -> Result<(Fd, *mut u8), std::io::Error> {
    unsafe {
        let fd = libc::memfd_create(b"wl-buffer\0".as_ptr() as *const _, libc::MFD_CLOEXEC);
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
        Ok((Fd(fd), ptr as *mut u8))
    }
}
