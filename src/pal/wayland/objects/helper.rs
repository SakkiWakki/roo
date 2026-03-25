use std::mem::size_of;
use std::os::unix::io::AsRawFd;
use std::os::unix::net::UnixStream;

/// Sends a message with an attached file descriptor via SCM_RIGHTS
pub fn send_with_fd(stream: &mut UnixStream, msg: &[u8], fd: i32) -> Result<(), std::io::Error> {
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

pub fn create_memfd(size: usize) -> Result<(i32, *mut u8), std::io::Error> {
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
        Ok((fd, ptr as *mut u8))
    }
}