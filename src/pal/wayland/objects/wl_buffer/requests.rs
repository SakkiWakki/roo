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
