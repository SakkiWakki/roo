use std::ffi::{c_void, CString};
use std::marker::PhantomData;

pub struct LinuxLoader {
    handle: *mut c_void,
}

/// We add this so the compiler can have more explicit information about 
/// the loader's lifetime for safety
pub struct Symbol<'a, T> {
    pub ptr: T,
    _loader: PhantomData<&'a LinuxLoader>,
}

impl LinuxLoader {
    pub fn open(name: &str) -> Self {
        let name = CString::new(name).unwrap();
        let handle = unsafe { libc::dlopen(name.as_ptr(), libc::RTLD_NOW | libc::RTLD_LOCAL) };
        assert!(!handle.is_null(), "dlopen failed: {}", unsafe {
            std::ffi::CStr::from_ptr(libc::dlerror()).to_string_lossy()
        });
        Self { handle }
    }

    /// Caller must ensure T matches the type in the called library!!!
    pub unsafe fn symbol<T>(&self, name: &str) -> Option<Symbol<T>> {
        let name = CString::new(name).unwrap();
        let ptr = unsafe { libc::dlsym(self.handle, name.as_ptr()) };
        if ptr.is_null() {
            return None;
        }
        Some(Symbol {
            ptr: unsafe { std::mem::transmute_copy(&ptr) },
            _loader: PhantomData,
        })
    }
}

impl Drop for LinuxLoader {
    fn drop(&mut self) {
        unsafe { libc::dlclose(self.handle) };
    }
}
