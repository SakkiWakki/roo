use std::io::Cursor;
use std::os::unix::net::UnixStream;

use super::client::ZwpLinuxDmabufFeedbackV1;
use crate::pal::platform::types::{Fd, Tranche};
use crate::pal::platform::windowing::event_loop::LoopAction;
use crate::{read_at, read_msg};
pub struct FeedbackState {
    pub main_device: u64,
    pub format_table: Vec<(u32, u64)>,
    pub tranches: Vec<Tranche>,
    pub current_tranche: Tranche,
}

impl Default for FeedbackState {
    fn default() -> Self {
        Self {
            main_device: 0,
            format_table: Vec::new(),
            tranches: Vec::new(),
            current_tranche: Tranche::default()
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
        data: &[u8],
        _fd: Option<Fd>,
        state: &mut FeedbackState,
        _stream: &mut UnixStream,
    ) -> Result<LoopAction, std::io::Error> {
        let main_device = read_at!(data, 4, u64);
        state.main_device = main_device;
        Ok(LoopAction::Continue)
    }

    pub fn handle_tranche_done(
        _data: &[u8],
        _fd: Option<Fd>,
        state: &mut FeedbackState,
        _stream: &mut UnixStream,
    ) -> Result<LoopAction, std::io::Error> {
        let tranche_new = std::mem::replace(
            &mut state.current_tranche, Tranche::default()
        );
        state.tranches.push(tranche_new);
        Ok(LoopAction::Continue)
    }

    pub fn handle_tranche_target_device(
        data: &[u8],
        _fd: Option<Fd>,
        state: &mut FeedbackState,
        _stream: &mut UnixStream,
    ) -> Result<LoopAction, std::io::Error> {
        let target_device = read_at!(data, 4, u64);
        state.current_tranche.target_device = target_device;
        Ok(LoopAction::Continue)
    }

    pub fn handle_tranche_formats(
        data: &[u8],
        _fd: Option<Fd>,
        state: &mut FeedbackState,
        _stream: &mut UnixStream,
    ) -> Result<LoopAction, std::io::Error> {
        let array_len = read_at!(data, 0, u32) as usize;
        let index_count = array_len >> 1;
        for i in 0..index_count {
            let index = read_at!(data, 4 + (i << 1), u16) as usize;
            let entry = state.format_table[index];
            state.current_tranche.formats.push(entry);
        }
        Ok(LoopAction::Continue)
    }

    pub fn handle_tranche_flags(
        data: &[u8],
        _fd: Option<Fd>,
        state: &mut FeedbackState,
        _stream: &mut UnixStream,
    ) -> Result<LoopAction, std::io::Error> {
        state.current_tranche.flags = read_at!(data, 0, u32);
        Ok(LoopAction::Continue)
    }
}
