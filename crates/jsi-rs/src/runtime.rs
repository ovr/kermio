use std::pin::Pin;

use crate::sys::ffi;

/// Wrapper around facebook::jsi::Runtime providing a safe Rust API
pub struct JSRuntime {
    ptr: *mut ffi::JSIRuntime,
}

impl JSRuntime {
    pub unsafe fn from_raw(ptr: *mut ffi::JSIRuntime) -> Self {
        Self { ptr }
    }

    pub(crate) fn pin_mut(&mut self) -> Pin<&mut ffi::JSIRuntime> {
        unsafe { Pin::new_unchecked(&mut *self.ptr) }
    }

    /// Access the inner raw pointer for advanced usage
    #[cfg(feature = "sys")]
    pub fn inner(&self) -> *mut ffi::JSIRuntime {
        self.ptr
    }
}

// JSRuntime is Send but not Sync
// JavaScript runtimes are typically single-threaded
unsafe impl Send for JSRuntime {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_creation() {
        // This test just ensures the type compiles
        // Actual testing would require a real JSI runtime
    }
}
