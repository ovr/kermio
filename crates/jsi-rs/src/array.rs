use std::ptr::NonNull;

/// Wrapper around facebook::jsi::Array providing a safe Rust API
pub struct JSArray {
    inner: NonNull<crate::sys::ffi::JSIArray>,
}

impl JSArray {
    pub(crate) unsafe fn from_raw(ptr: *mut crate::sys::ffi::JSIArray) -> Self {
        Self {
            inner: NonNull::new_unchecked(ptr),
        }
    }

    /// Access the inner NonNull pointer for advanced usage
    #[cfg(feature = "sys")]
    pub fn inner(&self) -> NonNull<crate::sys::ffi::JSIArray> {
        self.inner
    }
}
