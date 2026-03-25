use std::io::Cursor;
use std::os::unix::net::UnixStream;

use super::client::ZwpLinuxDmabufFeedbackV1;
use crate::pal::platform::types::Fd;
use crate::pal::platform::windowing::event_loop::LoopAction;
use crate::{read_at, read_msg};

pub struct FeedbackState {
    pub main_device: u64,
    pub format_table: Vec<(u32, u64)>,
    pub formats: Vec<(u32, u64)>,
}

impl Default for FeedbackState {
    fn default() -> Self {
        Self {
            main_device: 0,
            format_table: Vec::new(),
            formats: Vec::new(),
        }
    }
}

impl ZwpLinuxDmabufFeedbackV1 {
    pub fn handle_done(
        _data: &[u8],
        fd: Option<Fd>,
        _state: &mut FeedbackState,
        _stream: &mut UnixStream,
    ) -> Result<LoopAction, std::io::Error> {
        if let Some(fd) = fd {
            unsafe { libc::close(fd.0) };
        }
        Ok(LoopAction::Break)
    }

    pub fn handle_format_table(
        data: &[u8],
        fd: Option<Fd>,
        state: &mut FeedbackState,
        _stream: &mut UnixStream,
    ) -> Result<LoopAction, std::io::Error> {
        let fd = fd.unwrap();
        let (size,) = read_msg!(Cursor::new(data), u32);
        let entry_count = size as usize >> 4;
        let ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                size as usize,
                libc::PROT_READ,
                libc::MAP_PRIVATE,
                fd.0,
                0,
            )
        };
        unsafe { libc::close(fd.0) };
        if ptr == libc::MAP_FAILED {
            return Err(std::io::Error::last_os_error());
        }
        let bytes = unsafe { std::slice::from_raw_parts(ptr as *const u8, size as usize) };
        for i in 0..entry_count {
            let offset = i << 4;
            let format = read_at!(bytes, offset, u32);
            // Bytes 4 to 8 is padding
            let modifier = read_at!(bytes, offset + 8, u64);
            state.format_table.push((format, modifier));
        }
        unsafe { libc::munmap(ptr, size as usize) };
        Ok(LoopAction::Continue)
    }

    pub fn handle_main_device(
        _data: &[u8],
        _fd: Option<Fd>,
        _state: &mut FeedbackState,
        _stream: &mut UnixStream,
    ) -> Result<LoopAction, std::io::Error> {
        unimplemented!()
    }

    pub fn handle_tranche_done(
        _data: &[u8],
        _fd: Option<Fd>,
        _state: &mut FeedbackState,
        _stream: &mut UnixStream,
    ) -> Result<LoopAction, std::io::Error> {
        unimplemented!()
    }

    pub fn handle_tranche_target_device(
        _data: &[u8],
        _fd: Option<Fd>,
        _state: &mut FeedbackState,
        _stream: &mut UnixStream,
    ) -> Result<LoopAction, std::io::Error> {
        unimplemented!()
    }

    pub fn handle_tranche_formats(
        _data: &[u8],
        _fd: Option<Fd>,
        _state: &mut FeedbackState,
        _stream: &mut UnixStream,
    ) -> Result<LoopAction, std::io::Error> {
        unimplemented!()
    }

    pub fn handle_tranche_flags(
        _data: &[u8],
        _fd: Option<Fd>,
        _state: &mut FeedbackState,
        _stream: &mut UnixStream,
    ) -> Result<LoopAction, std::io::Error> {
        unimplemented!()
    }
}
