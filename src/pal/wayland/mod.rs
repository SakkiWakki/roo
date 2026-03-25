pub mod encoding;
pub mod objects;
pub mod types;
pub mod windowing;

pub use types::globals::*;
pub use types::protocol::*;
pub use types::sizes::*;

#[cfg(test)]
mod tests {
    use super::objects::{create_memfd, WlBuffer};
    use serial_test::serial;

    fn open_fd_count() -> usize {
        std::fs::read_dir("/proc/self/fd")
            .expect("Could not read /proc/self/fd")
            .count()
    }

    fn assert_valid_allocation(size: usize) {
        let (fd, ptr) = create_memfd(size).expect(&format!("Create_memfd({}) failed", size));

        assert_ne!(ptr as usize, libc::MAP_FAILED as usize);
        assert!(!ptr.is_null());

        unsafe {
            libc::close(fd);
            *ptr = 0xAA;
            assert_eq!(*ptr, 0xAA);
            *ptr.add(size - 1) = 0xBB;
            assert_eq!(*ptr.add(size - 1), 0xBB);

            let buf = std::slice::from_raw_parts_mut(ptr, size);
            buf.fill(0xCC);
            assert!(buf.iter().all(|&b| b == 0xCC));

            libc::munmap(ptr as *mut _, size)
        };
    }

    #[test]
    #[serial]
    fn typical_allocation() {
        assert_valid_allocation(800 * 600 * 4);
    }

    #[test]
    #[serial]
    fn repeated_allocations_do_not_leak_fds() {
        let _ = create_memfd(4096);

        let fd_before = open_fd_count();
        for _ in 0..100 {
            let (fd, ptr) = create_memfd(4096).expect("create_memfd failed");
            unsafe {
                libc::close(fd);
                libc::munmap(ptr as *mut _, 4096);
            }
        }
        let fd_after = open_fd_count();

        assert!(
            fd_after <= fd_before,
            "fd leak over 100 allocations: {} before, {} after",
            fd_before,
            fd_after
        );
    }

    #[test]
    #[serial]
    fn concurrent_allocations_no_panic() {
        let threads: Vec<_> = (0..8)
            .map(|_| {
                std::thread::spawn(|| {
                    for _ in 0..100 {
                        let (fd, ptr) = create_memfd(4096).expect("create_memfd failed");
                        unsafe {
                            libc::close(fd);
                            libc::munmap(ptr as *mut _, 4096);
                        }
                    }
                })
            })
            .collect();

        for t in threads {
            t.join().expect("thread panicked");
        }
    }

    #[test]
    #[serial]
    fn mmap_failure_does_not_leak_fd() {
        assert!(create_memfd(0).is_err());

        let fd_before = open_fd_count();
        for _ in 0..100 {
            let _ = create_memfd(0);
        }
        let fd_after = open_fd_count();
        assert!(
            fd_after == fd_before,
            "fd leaked on failed allocations (mmap path): {} before, {} after",
            fd_before,
            fd_after
        );
    }

    #[test]
    #[serial]
    fn ftruncate_failure_does_not_leak_fd() {
        let _ = create_memfd(usize::MAX);

        let fd_before = open_fd_count();
        for _ in 0..100 {
            let _ = create_memfd(usize::MAX);
        }
        let fd_after = open_fd_count();
        assert!(
            fd_after == fd_before,
            "fd leaked on failed allocations (ftruncate path): {} before, {} after",
            fd_before,
            fd_after
        );
    }

    #[test]
    #[serial]
    fn wl_buffer_drop_munmaps() {
        let size: usize = 4096;
        let (fd, ptr) = create_memfd(size).expect("create_memfd failed");
        unsafe {
            libc::close(fd);

            {
                let buf = WlBuffer { id: 0, ptr, size };
                drop(buf);
            }

            let ret = libc::msync(ptr as *mut _, size, libc::MS_SYNC);
            assert_eq!(
                ret, -1,
                "msync should fail on unmapped memory after WlBuffer drop"
            );
            assert_eq!(
                std::io::Error::last_os_error().raw_os_error(),
                Some(libc::ENOMEM),
                "expected ENOMEM after munmap, got different error"
            );
        };
    }


    #[test]
    #[serial]
    fn memfd_has_cloexec() {
        let (fd, ptr) = create_memfd(4096).expect("create_memfd failed");
        unsafe {
            let flags = libc::fcntl(fd, libc::F_GETFD);
            assert!(flags != -1, "fcntl failed");
            assert!(flags & libc::FD_CLOEXEC != 0, "FD_CLOEXEC not set");
            libc::munmap(ptr as *mut _, 4096);
            libc::close(fd);
        }
    }
}


